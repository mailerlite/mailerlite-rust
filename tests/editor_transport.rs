//! Simple-editor requests inherit v2 transport policy and redaction guarantees.
#![allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]
use mailerlite_rs::{
    ApiVersion, Error, MailerLite, RetryPolicy, models::emails::BlockType,
    resources::emails::NewBlock,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

#[tokio::test]
async fn email_reads_use_pinned_version_and_opt_in_retries() {
    let server = MockServer::start().await;
    let attempts = Arc::new(AtomicUsize::new(0));
    let received = Arc::clone(&attempts);
    Mock::given(method("GET"))
        .and(path("/emails/123"))
        .and(header("X-Version", "2026-10-30"))
        .respond_with(move |_: &wiremock::Request| {
            if received.fetch_add(1, Ordering::SeqCst) == 0 {
                ResponseTemplate::new(503)
            } else {
                ResponseTemplate::new(200).set_body_string(include_str!("fixtures/email.json"))
            }
        })
        .expect(2)
        .mount(&server)
        .await;
    let client = MailerLite::builder("test-token")
        .base_url(server.uri())
        .api_version(ApiVersion::pinned("2026-10-30").unwrap())
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .retry_policy(RetryPolicy::safe_reads().initial_delay(Duration::ZERO))
        .build()
        .unwrap();
    assert_eq!(client.emails().get("123").await.unwrap().data.etag, "42");
    assert_eq!(attempts.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn an_idempotency_key_does_not_enable_automatic_write_retries() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/emails/123/blocks"))
        .and(header("Idempotency-Key", "private-key"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    let client = MailerLite::builder("test-token")
        .base_url(server.uri())
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .retry_policy(RetryPolicy::safe_reads().initial_delay(Duration::ZERO))
        .build()
        .unwrap();
    let emails = client.emails().with_idempotency_key("private-key");
    assert!(!format!("{emails:?}").contains("private-key"));
    let block = NewBlock {
        block_type: BlockType::Divider,
        editor_content: Default::default(),
        placement: None,
    };
    assert!(
        matches!(emails.create_block("123","42",&block).await,Err(Error::Api(error)) if error.status()==503)
    );
}

#[tokio::test]
async fn conflict_details_are_accessible_but_redacted() {
    let server = MockServer::start().await;
    Mock::given(method("POST")).and(path("/emails/123/versions/9/restore"))
        .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({"message":"private-message","code":"version_conflict","etag":"private-etag"})))
        .expect(1).mount(&server).await;
    let client = MailerLite::builder("test-token")
        .base_url(server.uri())
        .build()
        .unwrap();
    let error = client
        .emails()
        .restore_version("123", "9", "old-etag")
        .await
        .unwrap_err();
    let rendered = format!("{error:?} {error}");
    assert!(!rendered.contains("private-"));
    let Error::Api(error) = error else {
        panic!("expected API error")
    };
    assert!(error.is_version_conflict());
    assert_eq!(error.etag(), Some("private-etag"));
}
