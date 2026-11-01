//! Verify bounded client retries preserve write identity after transient failures.
#![allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]

use mailerlite_rs::{
    Error, MailerLite, Response, RetryPolicy,
    models::emails::{BlockContent, EmailBlock, EmailResponseMetadata, WithEtag},
    resources::emails::NewBlock,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::sync::Notify;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

// Long real-time budgets allow local I/O to finish even on loaded CI runners.
// Timeout scenarios advance only the SDK runtime's clock after the request arrives.
const TIMEOUT: Duration = Duration::from_secs(30);

fn retry_policy() -> RetryPolicy {
    RetryPolicy::safe_reads()
        .with_idempotent_writes()
        .max_retries(1)
        .initial_delay(Duration::ZERO)
}

#[derive(Clone, Copy)]
enum Failure {
    TransportTimeout,
    IncompleteJson,
}

fn success(replayed: bool) -> ResponseTemplate {
    ResponseTemplate::new(201)
        .insert_header("Idempotent-Replayed", replayed.to_string())
        .set_body_json(serde_json::json!({
            "data": {"id": "456", "type": "divider", "editor_content": {}},
            "etag": "43"
        }))
}

fn write_mock() -> wiremock::MockBuilder {
    Mock::given(method("POST"))
        .and(path("/emails/123/blocks"))
        .and(header("Idempotency-Key", "same-write"))
}

fn client(
    server: &MockServer,
    request_timeout: Duration,
    operation_timeout: Duration,
) -> MailerLite {
    MailerLite::builder("test-token")
        .base_url(server.uri())
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .retry_policy(retry_policy())
        .request_timeout(request_timeout)
        .operation_timeout(operation_timeout)
        .build()
        .unwrap()
}

async fn create(
    client: &MailerLite,
) -> mailerlite_rs::Result<Response<WithEtag<EmailBlock>, EmailResponseMetadata>> {
    let email = client.emails().with_idempotency_key("same-write");
    let block = NewBlock::new(BlockContent::Divider(Default::default()));
    email.create_block("123", "42", &block).await
}

async fn assert_same_write(server: &MockServer) {
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 2, "one original request and one retry");
    assert_eq!(
        requests[0].body, requests[1].body,
        "retry must preserve the body"
    );
    assert_eq!(
        requests[0].body_json::<serde_json::Value>().unwrap()["etag"],
        "42"
    );
}

async fn assert_retry(failure: Failure) {
    let server = MockServer::start().await;
    let attempts = Arc::new(AtomicUsize::new(0));
    let received = Arc::clone(&attempts);
    let first_received = Arc::new(Notify::new());
    let notification = Arc::clone(&first_received);
    write_mock()
        .respond_with(move |_: &wiremock::Request| {
            if received.fetch_add(1, Ordering::SeqCst) == 0 {
                notification.notify_one();
                match failure {
                    Failure::IncompleteJson => {
                        ResponseTemplate::new(201).set_body_string(r#"{"data":"#)
                    }
                    // Wiremock runs on its own runtime; keep the first response pending
                    // while the test advances the SDK's request or operation deadline.
                    _ => success(false).set_delay(TIMEOUT * 4),
                }
            } else {
                success(true)
            }
        })
        .expect(2)
        .mount(&server)
        .await;
    let client = client(&server, TIMEOUT, TIMEOUT * 2);
    let mut write = std::pin::pin!(create(&client));
    if !matches!(failure, Failure::IncompleteJson) {
        tokio::select! {
            biased;
            _ = first_received.notified() => {}
            result = &mut write => panic!("write finished before the injected timeout: {result:?}"),
        }
        tokio::time::pause();
        tokio::time::advance(TIMEOUT + Duration::from_secs(1)).await;
        // Resume before polling the retry so real network I/O cannot trigger the
        // paused clock's automatic advance to the new attempt's deadline.
        tokio::time::resume();
    }
    let response = write.await.unwrap();
    assert_eq!(response.data.data.id, "456");
    assert!(
        response.metadata.is_idempotent_replay(),
        "retry must decode replay metadata"
    );
    assert_same_write(&server).await;
}

#[tokio::test]
async fn retry_preserves_key_and_payload_after_transport_timeout() {
    assert_retry(Failure::TransportTimeout).await;
}

#[tokio::test]
async fn retry_preserves_key_and_payload_after_incomplete_json() {
    assert_retry(Failure::IncompleteJson).await;
}

#[tokio::test]
async fn initial_success_is_not_retried() {
    let server = MockServer::start().await;
    write_mock()
        .respond_with(success(false))
        .expect(1)
        .mount(&server)
        .await;
    let response = create(&client(&server, TIMEOUT, TIMEOUT * 2))
        .await
        .unwrap();
    assert_eq!(response.data.data.id, "456");
    assert!(
        !response.metadata.is_idempotent_replay(),
        "original response is not a replay"
    );
}

#[tokio::test]
async fn api_errors_are_not_retried() {
    let server = MockServer::start().await;
    write_mock()
        .respond_with(
            ResponseTemplate::new(422)
                .set_body_json(serde_json::json!({"message": "Invalid block"})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let error = create(&client(&server, TIMEOUT, TIMEOUT * 2))
        .await
        .unwrap_err();
    assert!(
        matches!(error, Error::Api(ref api) if api.status().as_u16() == 422),
        "expected the original API error: {error:?}"
    );
}

#[tokio::test]
async fn second_ambiguous_failure_is_returned_without_a_third_attempt() {
    let server = MockServer::start().await;
    write_mock()
        .respond_with(ResponseTemplate::new(201).set_body_string(r#"{"data":"#))
        .expect(2)
        .mount(&server)
        .await;
    let error = create(&client(&server, TIMEOUT, TIMEOUT * 2))
        .await
        .unwrap_err();
    assert!(
        matches!(error, Error::Decode(ref decode) if decode.json_error().is_some_and(serde_json::Error::is_eof)),
        "expected the second incomplete response: {error:?}"
    );
    assert_same_write(&server).await;
}

#[tokio::test]
async fn operation_deadline_stops_retries_without_starting_a_new_budget() {
    let server = MockServer::start().await;
    let received = Arc::new(Notify::new());
    let notification = Arc::clone(&received);
    write_mock()
        .respond_with(move |_: &wiremock::Request| {
            notification.notify_one();
            success(false).set_delay(TIMEOUT * 4)
        })
        .expect(1)
        .mount(&server)
        .await;
    let client = client(&server, TIMEOUT * 2, TIMEOUT);
    let mut write = std::pin::pin!(create(&client));
    tokio::select! {
        biased;
        _ = received.notified() => {}
        result = &mut write => panic!("write finished before the deadline: {result:?}"),
    }
    tokio::time::pause();
    tokio::time::advance(TIMEOUT + Duration::from_secs(1)).await;
    assert!(matches!(write.await, Err(Error::Timeout)));
    tokio::time::resume();
}

#[tokio::test]
async fn retryable_statuses_preserve_the_original_write() {
    for status in [408, 429, 500, 502, 503, 504] {
        let server = MockServer::start().await;
        let attempts = AtomicUsize::new(0);
        write_mock()
            .respond_with(move |_: &wiremock::Request| {
                if attempts.fetch_add(1, Ordering::SeqCst) == 0 {
                    ResponseTemplate::new(status)
                } else {
                    success(true)
                }
            })
            .expect(2)
            .mount(&server)
            .await;
        let response = create(&client(&server, TIMEOUT, TIMEOUT * 2))
            .await
            .unwrap();
        assert!(response.metadata.is_idempotent_replay());
        assert_same_write(&server).await;
    }
}

const IN_PROGRESS: &str = "A request with this Idempotency-Key is still in progress.";

#[tokio::test]
async fn in_progress_conflict_waits_before_replaying_the_same_write() {
    let server = MockServer::start().await;
    let attempts = AtomicUsize::new(0);
    write_mock()
        .respond_with(move |_: &wiremock::Request| {
            if attempts.fetch_add(1, Ordering::SeqCst) == 0 {
                ResponseTemplate::new(409)
                    .set_body_json(serde_json::json!({"message": IN_PROGRESS}))
            } else {
                success(true)
            }
        })
        .expect(2)
        .mount(&server)
        .await;
    // Check the minimum even with zero configured backoff. Real time allows the
    // mock's separate runtime to participate without automatic clock advancement.
    let start = std::time::Instant::now();
    let response = create(&client(&server, TIMEOUT, TIMEOUT * 2))
        .await
        .unwrap();
    assert!(start.elapsed() >= Duration::from_secs(1));
    assert!(response.metadata.is_idempotent_replay());
    assert_same_write(&server).await;
}

#[tokio::test]
async fn conflicts_are_classified_conservatively_and_not_retried() {
    for (status, body, version_conflict) in [
        (
            409,
            serde_json::json!({"code": "version_conflict", "message": IN_PROGRESS, "etag": "43"}),
            true,
        ),
        (
            409,
            serde_json::json!({"message": "Unknown conflict"}),
            false,
        ),
        (
            409,
            serde_json::json!({"message": "This idempotency request is still being processed"}),
            false,
        ),
        (
            409,
            serde_json::json!({"code": "other", "message": IN_PROGRESS}),
            false,
        ),
        (422, serde_json::json!({"message": IN_PROGRESS}), false),
    ] {
        let server = MockServer::start().await;
        write_mock()
            .respond_with(ResponseTemplate::new(status).set_body_json(body))
            .expect(1)
            .mount(&server)
            .await;
        let Error::Api(error) = create(&client(&server, TIMEOUT, TIMEOUT * 2))
            .await
            .unwrap_err()
        else {
            panic!("expected original conflict");
        };
        assert_eq!(error.is_version_conflict(), version_conflict);
        assert!(!error.is_idempotency_in_progress());
    }
}

#[tokio::test]
async fn retry_after_is_not_shortened_to_fit_the_deadline() {
    for status in [409, 429, 503] {
        let server = MockServer::start().await;
        write_mock()
            .respond_with(
                ResponseTemplate::new(status)
                    .insert_header("Retry-After", "120")
                    .set_body_json(serde_json::json!({"message": IN_PROGRESS})),
            )
            .expect(1)
            .mount(&server)
            .await;
        let Error::Api(error) = create(&client(&server, TIMEOUT, TIMEOUT * 2))
            .await
            .unwrap_err()
        else {
            panic!("expected API error when the retry cannot fit");
        };
        assert_eq!(error.is_idempotency_in_progress(), status == 409);
        assert!(!error.is_version_conflict());
        assert_eq!(
            error.metadata().retry_after(),
            Some(Duration::from_secs(120))
        );
    }
}

#[tokio::test]
async fn complete_invalid_responses_are_not_retried() {
    for (status, body) in [
        (201, ""),
        (201, " \t\r\n"),
        (201, "null"),
        (201, r#"{"data":{"id":123},"etag":"43"}"#),
        (201, r#"{"data":no}"#),
        (201, r#"{} {}"#),
        (200, r#"{"data":"#),
    ] {
        let server = MockServer::start().await;
        write_mock()
            .respond_with(ResponseTemplate::new(status).set_body_string(body))
            .expect(1)
            .mount(&server)
            .await;
        let Error::Decode(error) = create(&client(&server, TIMEOUT, TIMEOUT * 2))
            .await
            .unwrap_err()
        else {
            panic!("expected permanent decode failure");
        };
        assert_eq!(error.status().as_u16(), status);
        assert_eq!(error.body(), body.as_bytes());
    }
}

#[tokio::test]
async fn response_byte_limit_is_not_retried_even_with_truncated_json() {
    let server = MockServer::start().await;
    write_mock()
        .respond_with(success(false))
        .expect(1)
        .mount(&server)
        .await;
    let client = MailerLite::builder("test-token")
        .base_url(server.uri())
        .retry_policy(retry_policy())
        .max_response_bytes(8)
        .build()
        .unwrap();
    let Error::Decode(error) = create(&client).await.unwrap_err() else {
        panic!("expected response limit error");
    };
    assert!(error.body_truncated());
    assert!(error.detail().contains("byte limit"));
}

#[tokio::test]
async fn keyed_write_retries_require_both_opt_in_and_a_nonzero_limit() {
    for policy in [
        RetryPolicy::none(),
        RetryPolicy::safe_reads(),
        RetryPolicy::none().with_idempotent_writes(),
    ] {
        for response in [
            ResponseTemplate::new(503),
            ResponseTemplate::new(201).set_body_string(r#"{"data":"#),
        ] {
            let server = MockServer::start().await;
            write_mock()
                .respond_with(response)
                .expect(1)
                .mount(&server)
                .await;
            let client = MailerLite::builder("test-token")
                .base_url(server.uri())
                .retry_policy(policy.clone())
                .build()
                .unwrap();
            assert!(create(&client).await.is_err());
        }
    }
}

#[tokio::test]
async fn opting_into_keyed_writes_does_not_retry_unkeyed_writes() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(|request: &wiremock::Request| !request.headers.contains_key("idempotency-key"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    let client = client(&server, TIMEOUT, TIMEOUT * 2);
    let result = client
        .emails()
        .create_block(
            "123",
            "42",
            &NewBlock::new(BlockContent::Divider(Default::default())),
        )
        .await;
    assert!(matches!(result, Err(Error::Api(_))));
}

#[tokio::test]
async fn interrupted_http_body_retries_the_original_write() {
    assert_interrupted_body(201, r#"{"data":"#, 2).await;
}

#[tokio::test]
async fn interrupted_conflict_body_retries_without_guessing_its_message() {
    assert_interrupted_body(409, r#"{"message":"A request with this Idempotency-Key"#, 2).await;
}

#[tokio::test]
async fn interrupted_permanent_errors_and_unexpected_statuses_are_not_retried() {
    for (status, body) in [
        (
            409,
            r#"{"code":"version_conflict","message":"Changed","etag":"43"}"#,
        ),
        (409, r#"{"code":"other","message":"Conflict"}"#),
        (422, r#"{"message":"#),
        (200, r#"{"data":"#),
    ] {
        assert_interrupted_body(status, body, 1).await;
    }
}

async fn assert_interrupted_body(status: u16, partial: &'static str, attempts: usize) {
    use std::io::{BufRead, Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let mut bodies = Vec::new();
        for attempt in 0..attempts {
            let (mut socket, _) = listener.accept().unwrap();
            socket.set_read_timeout(Some(TIMEOUT)).unwrap();
            let mut reader = std::io::BufReader::new(&mut socket);
            let mut headers = String::new();
            loop {
                let mut line = String::new();
                assert!(reader.read_line(&mut line).unwrap() > 0);
                if line == "\r\n" {
                    break;
                }
                headers.push_str(&line);
            }
            let headers = headers.to_ascii_lowercase();
            assert!(headers.contains("idempotency-key: same-write\r\n"));
            let length: usize = headers
                .lines()
                .find_map(|line| {
                    line.strip_prefix("content-length:")
                        .map(|value| value.trim().parse().unwrap())
                })
                .unwrap();
            let mut body = vec![0; length];
            reader.read_exact(&mut body).unwrap();
            bodies.push(body);
            if attempt == 0 {
                // Advertise more bytes than are sent, then close the connection.
                write!(socket, "HTTP/1.1 {status} Test\r\nContent-Length: 1000\r\nConnection: close\r\n\r\n{partial}").unwrap();
            } else {
                let body =
                    r#"{"data":{"id":"456","type":"divider","editor_content":{}},"etag":"43"}"#;
                write!(socket, "HTTP/1.1 201 Created\r\nContent-Length: {}\r\nIdempotent-Replayed: true\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
            }
        }
        if attempts == 2 {
            assert_eq!(bodies[0], bodies[1]);
        }
    });
    let client = MailerLite::builder("test-token")
        .base_url(format!("http://{address}"))
        .retry_policy(retry_policy())
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .build()
        .unwrap();
    let started = std::time::Instant::now();
    let result = create(&client).await;
    if attempts == 2 {
        let response = result.unwrap();
        assert_eq!(response.data.data.id, "456");
        assert!(response.metadata.is_idempotent_replay());
        if status == 409 {
            assert!(started.elapsed() >= Duration::from_secs(1));
        }
    } else {
        let Error::Decode(error) = result.unwrap_err() else {
            panic!("expected interrupted body diagnostics")
        };
        assert_eq!(error.status().as_u16(), status);
        assert_eq!(error.body(), partial.as_bytes());
    }
    server.join().unwrap();
}

#[tokio::test]
async fn safe_reads_retry_nonempty_json_eof_only_when_enabled() {
    for policy in [
        RetryPolicy::none(),
        RetryPolicy::safe_reads()
            .max_retries(1)
            .initial_delay(Duration::ZERO),
    ] {
        let enabled = policy.retry_limit() > 0;
        let server = MockServer::start().await;
        let attempts = AtomicUsize::new(0);
        Mock::given(method("GET"))
            .and(path("/emails/123/blocks/456"))
            .respond_with(move |_: &wiremock::Request| {
                if attempts.fetch_add(1, Ordering::SeqCst) == 0 {
                    ResponseTemplate::new(200).set_body_string(r#"{"data":"#)
                } else {
                    ResponseTemplate::new(200).set_body_json(serde_json::json!({
                        "data": {"id":"456", "type":"divider", "editor_content":{}}, "etag":"43"
                    }))
                }
            })
            .expect(if enabled { 2 } else { 1 })
            .mount(&server)
            .await;
        let client = MailerLite::builder("test-token")
            .base_url(server.uri())
            .retry_policy(policy)
            .build()
            .unwrap();
        let result = client.emails().get_block("123", "456").await;
        if enabled {
            assert_eq!(result.unwrap().data.data.id, "456");
        } else {
            assert!(
                matches!(result, Err(Error::Decode(ref error)) if error.json_error().is_some_and(serde_json::Error::is_eof))
            );
        }
    }
}

#[tokio::test]
async fn safe_reads_do_not_retry_empty_or_permanently_invalid_json() {
    for (status, body) in [
        (200, ""),
        (200, " \t\r\n"),
        (200, "null"),
        (200, r#"{"data":null,"etag":"43"}"#),
        (200, r#"{"data":invalid}"#),
        (200, "{} {}"),
        (201, r#"{"data":"#),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(status).set_body_string(body))
            .expect(1)
            .mount(&server)
            .await;
        let client = MailerLite::builder("test-token")
            .base_url(server.uri())
            .retry_policy(RetryPolicy::safe_reads().initial_delay(Duration::ZERO))
            .build()
            .unwrap();
        let Error::Decode(error) = client.emails().get_block("123", "456").await.unwrap_err()
        else {
            panic!("expected decoding error")
        };
        assert_eq!(error.body(), body.as_bytes());
        assert_eq!(error.status().as_u16(), status);
    }
}

#[tokio::test]
async fn json_eof_and_http_failures_share_the_retry_limit() {
    let server = MockServer::start().await;
    let attempts = AtomicUsize::new(0);
    write_mock()
        .respond_with(
            move |_: &wiremock::Request| match attempts.fetch_add(1, Ordering::SeqCst) {
                0 => ResponseTemplate::new(503),
                1 => ResponseTemplate::new(201).set_body_string(r#"{"data":"#),
                _ => success(true),
            },
        )
        .expect(2)
        .mount(&server)
        .await;
    let error = create(&client(&server, TIMEOUT, TIMEOUT * 2))
        .await
        .unwrap_err();
    assert!(
        matches!(error, Error::Decode(ref error) if error.json_error().is_some_and(serde_json::Error::is_eof))
    );
    assert_same_write(&server).await;
}

#[tokio::test]
async fn json_eof_keeps_the_last_error_when_retry_delay_will_not_fit() {
    let server = MockServer::start().await;
    write_mock()
        .respond_with(
            ResponseTemplate::new(201)
                .insert_header("Retry-After", "120")
                .set_body_string(r#"{"data":"#),
        )
        .expect(1)
        .mount(&server)
        .await;
    let Error::Decode(error) = create(&client(&server, TIMEOUT, TIMEOUT * 2))
        .await
        .unwrap_err()
    else {
        panic!("expected last decode error, not operation timeout")
    };
    assert!(error.json_error().unwrap().is_eof());
    assert_eq!(
        error.metadata().retry_after(),
        Some(Duration::from_secs(120))
    );
}
