//! Simple-editor requests inherit v2 transport policy and redaction guarantees.
#![allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]
use mailerlite_rs::{
    ApiVersion, Error, MailerLite, RetryPolicy, models::emails::BlockContent,
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
        content: BlockContent::Divider(Default::default()),
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

#[test]
fn published_response_literals_and_mapping_remain_compatible() {
    use mailerlite_rs::models::emails::EmailResponseMetadata;
    use mailerlite_rs::{Response, ResponseMetadata, StatusCode};

    // Keep the complete 2.0 struct literal: adding even a private field breaks it.
    let response: Response<u8> = Response {
        data: 7,
        metadata: ResponseMetadata {
            status: StatusCode::OK,
            rate_limit: Some(120),
            rate_remaining: Some(119),
            retry_after: None,
            request_id: Some("test-request".into()),
        },
    };
    let response: Response<u16> = response.map(u16::from);
    assert_eq!(response.data, 7);
    assert_eq!(response.metadata.rate_remaining, Some(119));

    let email: Response<u16, EmailResponseMetadata> = Response {
        data: response.data,
        metadata: EmailResponseMetadata {
            http: response.metadata,
            idempotent_replayed: true,
        },
    };
    let mapped = email.map(|value| value.to_string());
    assert_eq!(mapped.data, "7");
    assert_eq!(mapped.metadata.http.status, StatusCode::OK);
    assert_eq!(
        mapped.metadata.http.request_id.as_deref(),
        Some("test-request")
    );
    assert!(mapped.metadata.is_idempotent_replay());
}

#[test]
fn annotated_response_literals_accept_application_metadata_conversions() {
    use mailerlite_rs::{Response, ResponseMetadata, StatusCode};

    struct AppMetadata(ResponseMetadata);
    impl From<AppMetadata> for ResponseMetadata {
        fn from(value: AppMetadata) -> Self {
            value.0
        }
    }
    let metadata = AppMetadata(ResponseMetadata {
        status: StatusCode::CREATED,
        rate_limit: None,
        rate_remaining: None,
        retry_after: None,
        request_id: Some("converted-request".into()),
    });
    // The annotation supplies the target of `.into()` even with generic metadata.
    let response: Response<u8> = Response {
        data: 7,
        metadata: metadata.into(),
    };
    let mapped = response.map(u16::from);
    assert_eq!(mapped.data, 7);
    assert_eq!(mapped.metadata.status, StatusCode::CREATED);
    assert_eq!(
        mapped.metadata.request_id.as_deref(),
        Some("converted-request")
    );
}
