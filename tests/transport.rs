//! Deterministic transport, diagnostics, retry, and credential-boundary tests.
#![allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod support;
use mailerlite_rs::{
    ApiVersion, Error, MailerLite, RetryPolicy, resources::subscribers::UpsertSubscriber,
};
use serde_json::json;
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, SystemTime},
};
use support::{client, subscriber};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

#[tokio::test]
async fn injected_http_client_preserves_base_prefix_auth_version_and_encoded_ids() {
    const PINNED: &str = "2026-10-30";

    let server = MockServer::start().await;
    let http = MailerLite::http_client_builder().build().unwrap();
    let builder = MailerLite::builder("private-test-token")
        .base_url(format!("{}/proxy/api/", server.uri()))
        .api_version(ApiVersion::pinned(PINNED).unwrap())
        .http_client(http)
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO);
    assert!(!format!("{builder:?}").contains("private-test-token"));
    let sdk = builder.build().unwrap();
    assert!(!format!("{sdk:?}").contains("private-test-token"));
    Mock::given(method("GET"))
        .and(path(
            "/proxy/api/subscribers/folder%2Freader+tag@example.test",
        ))
        .and(header("authorization", "Bearer private-test-token"))
        .and(header("accept", "application/json"))
        .and(header("content-type", "application/json"))
        .and(header("x-version", PINNED))
        .and(header(
            "user-agent",
            concat!("mailerlite-rs/", env!("CARGO_PKG_VERSION")),
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":subscriber()}))
                .insert_header("x-ratelimit-limit", "120")
                .insert_header("x-ratelimit-remaining", "119")
                .insert_header("x-request-id", "request-1"),
        )
        .expect(1)
        .mount(&server)
        .await;
    let result = sdk
        .subscribers()
        .get("folder/reader+tag@example.test")
        .await
        .unwrap();
    assert_eq!(result.data.id.as_ref(), "900719925474099312345");
    assert_eq!(result.metadata.rate_limit, Some(120));
    assert_eq!(result.metadata.rate_remaining, Some(119));
    assert_eq!(result.metadata.request_id.as_deref(), Some("request-1"));
}

#[tokio::test]
async fn default_version_omits_header() {
    assert!(matches!(ApiVersion::default(), ApiVersion::Latest));
    let server = MockServer::start().await;
    let sdk = MailerLite::builder("private-test-token")
        .base_url(format!("{}/api", server.uri()))
        .build()
        .unwrap();
    Mock::given(method("GET"))
        .and(path("/api/subscribers"))
        .and(|request: &wiremock::Request| !request.headers.contains_key("x-version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":0})))
        .expect(1)
        .mount(&server)
        .await;
    sdk.subscribers().count().await.unwrap();
}

#[tokio::test]
async fn latest_version_omits_header() {
    let server = MockServer::start().await;
    let sdk = MailerLite::builder("private-test-token")
        .base_url(format!("{}/api", server.uri()))
        .api_version(ApiVersion::Latest)
        .build()
        .unwrap();
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":0})))
        .expect(1)
        .mount(&server)
        .await;
    sdk.subscribers().count().await.unwrap();
    let received = server.received_requests().await.unwrap();
    assert!(!received[0].headers.contains_key("x-version"));
}

#[tokio::test]
async fn validation_error_preserves_metadata_and_private_diagnostics() {
    let server = MockServer::start().await;
    let sdk = client(&server);
    let body = json!({"message":"private-test-token", "errors":{"email":["private-test-token"]}});
    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(422)
                .set_body_json(&body)
                .insert_header("x-request-id", "validation-1"),
        )
        .expect(1)
        .mount(&server)
        .await;
    let error = sdk
        .subscribers()
        .upsert(UpsertSubscriber::new("invalid"))
        .await
        .unwrap_err();
    assert!(!format!("{error:?} {error}").contains("private-test-token"));
    let Error::Api(error) = error else {
        panic!("expected API error")
    };
    assert_eq!(error.status().as_u16(), 422);
    assert_eq!(error.message(), Some("private-test-token"));
    assert_eq!(error.validation_errors()["email"], ["private-test-token"]);
    assert_eq!(error.metadata().request_id.as_deref(), Some("validation-1"));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(error.body()).unwrap(),
        body
    );
}

#[tokio::test]
async fn html_error_preserves_status_and_private_diagnostics() {
    let server = MockServer::start().await;
    let sdk = client(&server);
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(403).set_body_string("<html>private-test-token</html>"))
        .expect(1)
        .mount(&server)
        .await;
    let error = sdk.subscribers().get("1").await.unwrap_err();
    assert!(!format!("{error:?} {error}").contains("private-test-token"));
    let Error::Api(error) = error else {
        panic!("expected API error")
    };
    assert_eq!(error.status().as_u16(), 403);
    assert_eq!(error.body(), b"<html>private-test-token</html>");
}

async fn assert_decode_error_without_retry(status: u16, body: &str) {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(status).set_body_string(body))
        .expect(1)
        .mount(&server)
        .await;
    let error = client(&server).subscribers().get("1").await.unwrap_err();
    let Error::Decode(error) = error else {
        panic!("expected decode error")
    };
    assert_eq!(error.status().as_u16(), status);
    assert_eq!(error.body(), body.as_bytes());
}
#[tokio::test]
async fn malformed_json_success_is_not_retried() {
    assert_decode_error_without_retry(200, "{invalid").await;
}
#[tokio::test]
async fn wrong_response_schema_is_not_retried() {
    assert_decode_error_without_retry(200, r#"{"data":{"id":1}}"#).await;
}
#[tokio::test]
async fn unexpected_accepted_response_is_not_retried() {
    assert_decode_error_without_retry(202, "{}").await;
}
#[tokio::test]
async fn unexpected_no_content_response_is_not_retried() {
    assert_decode_error_without_retry(204, "").await;
}

#[tokio::test]
async fn delete_accepts_empty_success() {
    let server = MockServer::start().await;
    let sdk = client(&server);
    Mock::given(method("DELETE"))
        .and(path("/api/subscribers/1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(
        sdk.subscribers()
            .delete("1")
            .await
            .unwrap()
            .metadata
            .status
            .as_u16(),
        204
    );
}

#[tokio::test]
async fn json_endpoint_rejects_empty_success_body() {
    let server = MockServer::start().await;
    let sdk = client(&server);
    Mock::given(method("GET"))
        .and(path("/api/subscribers/1"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;
    assert!(matches!(
        sdk.subscribers().get("1").await,
        Err(Error::Decode(_))
    ));
}

#[test]
fn default_policy_disables_retries() {
    assert_eq!(RetryPolicy::default().retry_limit(), 0);
}
async fn assert_default_read_not_retried(status: u16) {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/subscribers/1"))
        .respond_with(ResponseTemplate::new(status))
        .expect(1)
        .mount(&server)
        .await;
    let sdk = MailerLite::builder("test-token")
        .base_url(format!("{}/api", server.uri()))
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .build()
        .unwrap();
    let Error::Api(error) = sdk.subscribers().get("1").await.unwrap_err() else {
        panic!("expected original API error")
    };
    assert_eq!(error.status().as_u16(), status);
}
#[tokio::test]
async fn default_policy_does_not_retry_read_408() {
    assert_default_read_not_retried(408).await;
}
#[tokio::test]
async fn default_policy_does_not_retry_read_429() {
    assert_default_read_not_retried(429).await;
}
#[tokio::test]
async fn default_policy_does_not_retry_read_500() {
    assert_default_read_not_retried(500).await;
}
#[tokio::test]
async fn default_policy_does_not_retry_read_502() {
    assert_default_read_not_retried(502).await;
}
#[tokio::test]
async fn default_policy_does_not_retry_read_503() {
    assert_default_read_not_retried(503).await;
}
#[tokio::test]
async fn default_policy_does_not_retry_read_504() {
    assert_default_read_not_retried(504).await;
}

#[tokio::test]
async fn safe_read_recovers_after_two_transient_failures() {
    let server = MockServer::start().await;
    let count = Arc::new(AtomicUsize::new(0));
    let state = Arc::clone(&count);
    Mock::given(method("GET"))
        .respond_with(move |_: &wiremock::Request| {
            if state.fetch_add(1, Ordering::SeqCst) < 2 {
                ResponseTemplate::new(503)
                    .set_body_json(json!({"message":"temporarily unavailable"}))
            } else {
                ResponseTemplate::new(200).set_body_json(json!({"data":subscriber()}))
            }
        })
        .expect(3)
        .mount(&server)
        .await;
    client(&server).subscribers().get("1").await.unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn safe_read_stops_after_retry_budget_is_exhausted() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(500))
        .expect(3)
        .mount(&server)
        .await;
    assert!(matches!(
        client(&server).subscribers().get("1").await,
        Err(Error::Api(_))
    ));
}

async fn assert_write_not_retried(status: u16) {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(status).set_body_json(json!({"message":"failed"})))
        .expect(1)
        .mount(&server)
        .await;
    let error = client(&server)
        .subscribers()
        .upsert(UpsertSubscriber::new("reader@example.test"))
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Api(_)));
}
#[tokio::test]
async fn safe_read_policy_does_not_retry_write_408() {
    assert_write_not_retried(408).await;
}
#[tokio::test]
async fn safe_read_policy_does_not_retry_write_429() {
    assert_write_not_retried(429).await;
}
#[tokio::test]
async fn safe_read_policy_does_not_retry_write_500() {
    assert_write_not_retried(500).await;
}
#[tokio::test]
async fn safe_read_policy_does_not_retry_write_502() {
    assert_write_not_retried(502).await;
}
#[tokio::test]
async fn safe_read_policy_does_not_retry_write_503() {
    assert_write_not_retried(503).await;
}
#[tokio::test]
async fn safe_read_policy_does_not_retry_write_504() {
    assert_write_not_retried(504).await;
}

async fn assert_retry_after_exceeds_budget(value: String) {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", value)
                .set_body_json(json!({"message":"rate limited"})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let sdk = MailerLite::builder("token")
        .base_url(format!("{}/api", server.uri()))
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .operation_timeout(Duration::from_secs(2))
        .retry_policy(RetryPolicy::safe_reads())
        .build()
        .unwrap();
    let Error::Api(error) = sdk.subscribers().get("1").await.unwrap_err() else {
        panic!("expected original API error")
    };
    assert!(error.is_rate_limited());
    assert!(error.metadata().retry_after().unwrap() >= Duration::from_secs(59));
}
#[tokio::test]
async fn retry_after_seconds_beyond_budget_returns_original_error_without_early_retry() {
    assert_retry_after_exceeds_budget("60".to_owned()).await;
}
#[tokio::test]
async fn maximum_retry_after_returns_original_error_without_early_retry() {
    assert_retry_after_exceeds_budget(u64::MAX.to_string()).await;
}
#[tokio::test]
async fn retry_after_http_date_beyond_budget_returns_original_error_without_early_retry() {
    assert_retry_after_exceeds_budget(httpdate::fmt_http_date(
        SystemTime::now() + Duration::from_secs(3600),
    ))
    .await;
}

#[tokio::test]
async fn client_clones_share_server_cooldown_and_deadline_covers_limiter_wait() {
    assert_clones_respect_server_cooldown("60").await;
}

#[tokio::test]
async fn maximum_retry_after_bounds_clone_wait_without_sending_an_early_request() {
    assert_clones_respect_server_cooldown(&u64::MAX.to_string()).await;
}

async fn assert_clones_respect_server_cooldown(retry_after: &str) {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", retry_after))
        .expect(1)
        .mount(&server)
        .await;
    let sdk = MailerLite::builder("token")
        .base_url(format!("{}/api", server.uri()))
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .retry_policy(RetryPolicy::none())
        .operation_timeout(Duration::from_millis(100))
        .build()
        .unwrap();
    assert!(matches!(
        sdk.subscribers().get("1").await,
        Err(Error::Api(_))
    ));
    assert!(matches!(
        sdk.clone().subscribers().get("2").await,
        Err(Error::Timeout)
    ));
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn default_client_recovers_after_oversized_429_cooldown() {
    assert_client_recovers_after_cooldown(429, u64::MAX, RetryPolicy::none()).await;
}

#[tokio::test]
async fn default_client_recovers_after_oversized_503_cooldown() {
    assert_client_recovers_after_cooldown(503, u64::MAX, RetryPolicy::none()).await;
}

#[tokio::test]
async fn retry_enabled_client_recovers_without_replaying_oversized_429() {
    assert_client_recovers_after_cooldown(429, u64::MAX, RetryPolicy::safe_reads()).await;
}

#[tokio::test]
async fn retry_enabled_client_recovers_after_long_representable_503_cooldown() {
    assert_client_recovers_after_cooldown(503, 365 * 24 * 60 * 60, RetryPolicy::safe_reads()).await;
}

async fn assert_client_recovers_after_cooldown(status: u16, seconds: u64, policy: RetryPolicy) {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/subscribers/1"))
        .respond_with(
            ResponseTemplate::new(status)
                .insert_header("retry-after", seconds.to_string())
                .set_body_json(json!({"message":"try later"})),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/subscribers/2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": subscriber()})))
        .expect(1)
        .mount(&server)
        .await;
    let sdk = MailerLite::builder("token")
        .base_url(format!("{}/api", server.uri()))
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .operation_timeout(Duration::from_secs(2))
        .retry_policy(policy)
        .build()
        .unwrap();

    let Error::Api(error) = sdk.subscribers().get("1").await.unwrap_err() else {
        panic!("expected original API error without a retry")
    };
    assert_eq!(error.status().as_u16(), status);
    assert_eq!(
        error.metadata().retry_after(),
        Some(Duration::from_secs(seconds))
    );
    assert_eq!(server.received_requests().await.unwrap().len(), 1);

    // Skip only the cooldown; run network I/O with the clock resumed so mock
    // sockets cannot race Tokio's automatic advancement to request deadlines.
    tokio::time::pause();
    tokio::time::advance(Duration::from_secs(300)).await;
    tokio::time::resume();

    let response = sdk.clone().subscribers().get("2").await.unwrap();

    assert_eq!(response.data.id.as_str(), "900719925474099312345");
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}

#[tokio::test]
async fn redirects_cannot_forward_credentials_to_another_server() {
    let origin = MockServer::start().await;
    let hostile = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(302)
                .insert_header("location", format!("{}/steal", hostile.uri())),
        )
        .expect(1)
        .mount(&origin)
        .await;
    let error = client(&origin).subscribers().get("1").await.unwrap_err();
    let Error::Api(error) = error else {
        panic!("redirect must remain visible")
    };
    assert_eq!(error.status().as_u16(), 302);
    assert!(hostile.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn response_and_diagnostic_sizes_are_bounded() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("x".repeat(9000)))
        .expect(1)
        .mount(&server)
        .await;
    let sdk = MailerLite::builder("token")
        .base_url(format!("{}/api", server.uri()))
        .max_response_bytes(8500)
        .build()
        .unwrap();
    let Error::Decode(error) = sdk.subscribers().get("1").await.unwrap_err() else {
        panic!("expected bounded decode error")
    };
    assert_eq!(error.body().len(), 8192);
    assert!(error.body_truncated());
    assert_eq!(error.status().as_u16(), 200);
}

#[tokio::test]
async fn write_timeout_is_not_replayed_and_transport_source_omits_request_url() {
    use std::error::Error as _;
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":subscriber()}))
                .set_delay(Duration::from_secs(2)),
        )
        .expect(1)
        .mount(&server)
        .await;
    let sdk = MailerLite::builder("private-test-token")
        .base_url(format!("{}/api", server.uri()))
        .request_timeout(Duration::from_millis(200))
        .operation_timeout(Duration::from_secs(3))
        .retry_policy(RetryPolicy::safe_reads().initial_delay(Duration::ZERO))
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .build()
        .unwrap();
    let error = sdk
        .subscribers()
        .upsert(UpsertSubscriber::new("reader@example.test"))
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Transport(_)));
    assert!(!format!("{error:?} {error}").contains("private-test-token"));
    if let Some(source) = error.source() {
        let description = format!("{source:?} {source}");
        assert!(!description.contains(&server.uri()));
        assert!(!description.contains("private-test-token"));
    }
}

#[tokio::test]
async fn truncated_error_response_preserves_status_headers_and_is_not_retried() {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut request = [0; 4096];
        let received = socket.read(&mut request).unwrap();
        assert!(received > 0);
        socket.write_all(b"HTTP/1.1 422 Unprocessable Entity\r\nContent-Length: 1000\r\nContent-Type: application/json\r\nRetry-After: 12\r\nX-Request-Id: truncated-test\r\nConnection: close\r\n\r\n{\"message\":").unwrap();
    });
    let sdk = MailerLite::builder("private-test-token")
        .base_url(format!("http://{address}/api"))
        .request_timeout(Duration::from_secs(2))
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .retry_policy(RetryPolicy::safe_reads().initial_delay(Duration::ZERO))
        .build()
        .unwrap();
    let error = sdk.subscribers().get("123").await.unwrap_err();
    let Error::Decode(error) = error else {
        panic!("expected partial-response diagnostics");
    };
    assert_eq!(error.status().as_u16(), 422);
    assert_eq!(
        error.metadata().retry_after(),
        Some(Duration::from_secs(12))
    );
    assert_eq!(
        error.metadata().request_id.as_deref(),
        Some("truncated-test")
    );
    assert!(error.detail().contains("response body could not be read"));
    assert!(error.body_truncated());
    server.join().unwrap();
}

#[test]
fn base_url_rejects_embedded_credentials() {
    assert!(
        MailerLite::builder("secret")
            .base_url("https://user:secret@example.test/api")
            .build()
            .is_err()
    );
}

#[test]
fn base_url_rejects_query_parameters() {
    assert!(
        MailerLite::builder("secret")
            .base_url("https://example.test/api?token=secret")
            .build()
            .is_err()
    );
}

#[test]
fn base_url_rejects_fragments() {
    assert!(
        MailerLite::builder("secret")
            .base_url("https://example.test/api#secret")
            .build()
            .is_err()
    );
}

#[test]
fn base_url_rejects_non_http_schemes() {
    assert!(
        MailerLite::builder("secret")
            .base_url("file:///api")
            .build()
            .is_err()
    );
}

#[test]
fn token_rejects_header_injection() {
    assert!(MailerLite::new("secret\r\nInjected: value").is_err());
}

#[test]
fn token_rejects_empty_value() {
    assert!(MailerLite::new("").is_err());
}

#[test]
fn pinned_version_rejects_invalid_calendar_date() {
    assert!(ApiVersion::pinned("2025-02-29").is_err());
}
