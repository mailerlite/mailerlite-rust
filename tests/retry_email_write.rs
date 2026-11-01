//! Verify that the retry example preserves write identity after ambiguous failures.
#![allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]

// Exercise the example's helper directly so its retry behavior cannot drift from these tests.
#[path = "../examples/retry_email_write.rs"]
#[allow(
    dead_code,
    reason = "The example entry point is not run by integration tests"
)]
mod example;

use example::create_block_with_retry;
use mailerlite_rs::{
    Error, MailerLite, Response,
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

#[derive(Clone, Copy)]
enum Failure {
    TransportTimeout,
    OperationTimeout,
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
    create_block_with_retry(&email, "123", "42", &block).await
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
    let (request_timeout, operation_timeout) = match failure {
        Failure::OperationTimeout => (TIMEOUT * 2, TIMEOUT),
        _ => (TIMEOUT, TIMEOUT * 2),
    };
    let client = client(&server, request_timeout, operation_timeout);
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
async fn retry_preserves_key_and_payload_after_operation_timeout() {
    assert_retry(Failure::OperationTimeout).await;
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
