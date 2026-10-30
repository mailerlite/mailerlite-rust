//! Public error classification and diagnostic behavior for unusual HTTP responses.
#![allow(clippy::unwrap_used, clippy::expect_used)]
mod support;

use mailerlite_rs::{Error, resources::subscribers::UpsertSubscriber};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn validation_status_does_not_require_a_validation_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/subscribers"))
        .respond_with(ResponseTemplate::new(422).insert_header("x-request-id", "empty-validation"))
        .expect(1)
        .mount(&server)
        .await;
    let Error::Api(error) = support::default_client(&server)
        .subscribers()
        .upsert(UpsertSubscriber::new("invalid"))
        .await
        .unwrap_err()
    else {
        panic!("expected API error")
    };
    assert!(error.is_validation());
    assert!(!error.is_unauthorized());
    assert!(!error.is_server_error());
    assert!(!error.is_rate_limited());
    assert!(error.message().is_none());
    assert!(error.validation_errors().is_empty());
    assert_eq!(
        error.metadata().request_id.as_deref(),
        Some("empty-validation")
    );
    assert!(error.body().is_empty());
}

#[tokio::test]
async fn unauthorized_response_preserves_message_only_through_explicit_accessors() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(401).set_body_json(json!({"message":"private-credential"})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let failure = support::default_client(&server)
        .subscribers()
        .get("1")
        .await
        .unwrap_err();
    assert!(!format!("{failure:?} {failure}").contains("private-credential"));
    let Error::Api(error) = failure else {
        panic!("expected API error")
    };
    assert!(error.is_unauthorized());
    assert!(!error.is_validation());
    assert!(!error.is_server_error());
    assert_eq!(error.message(), Some("private-credential"));
}

#[tokio::test]
async fn forbidden_response_is_not_classified_as_unauthorized_or_validation() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_json(json!({"message":"Forbidden","errors":{"email":["Denied"]}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let Error::Api(error) = support::default_client(&server)
        .subscribers()
        .get("1")
        .await
        .unwrap_err()
    else {
        panic!("expected API error")
    };
    assert_eq!(error.status().as_u16(), 403);
    assert!(!error.is_unauthorized());
    assert!(!error.is_validation());
    assert!(!error.is_server_error());
    assert_eq!(error.validation_errors()["email"], ["Denied"]);
}

#[tokio::test]
async fn nonstandard_server_status_preserves_non_json_diagnostics_without_retrying() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(599)
                .set_body_string("upstream failure")
                .insert_header("retry-after", "12"),
        )
        .expect(1)
        .mount(&server)
        .await;
    let Error::Api(error) = support::default_client(&server)
        .subscribers()
        .get("1")
        .await
        .unwrap_err()
    else {
        panic!("expected API error")
    };
    assert!(error.is_server_error());
    assert!(!error.is_rate_limited());
    assert_eq!(error.body(), b"upstream failure");
    assert_eq!(
        error.metadata().retry_after(),
        Some(std::time::Duration::from_secs(12))
    );
}

#[tokio::test]
async fn large_api_error_keeps_structured_details_while_bounding_raw_diagnostics() {
    let server = MockServer::start().await;
    // Put structured fields beyond the retained diagnostic prefix: parsing must
    // use the complete response, not just the first 8192 bytes.
    let body = format!(
        r#"{{"padding":"{}","message":"Invalid data","errors":{{"email":["private-input"]}}}}"#,
        "x".repeat(9000)
    );
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(422)
                .set_body_string(body)
                .insert_header("x-request-id", "large-error"),
        )
        .expect(1)
        .mount(&server)
        .await;
    let failure = support::default_client(&server)
        .subscribers()
        .get("1")
        .await
        .unwrap_err();
    assert!(!format!("{failure:?} {failure}").contains("private-input"));
    let Error::Api(error) = failure else {
        panic!("expected API error")
    };
    assert!(error.body_truncated());
    assert_eq!(error.body().len(), 8192);
    assert_eq!(error.message(), Some("Invalid data"));
    assert_eq!(error.validation_errors()["email"], ["private-input"]);
    assert_eq!(error.metadata().request_id.as_deref(), Some("large-error"));
}

#[tokio::test]
async fn successful_json_followed_by_another_document_is_a_decode_error() {
    let server = MockServer::start().await;
    let body = format!(
        "{} {{\"extra\":true}}",
        json!({"data":support::subscriber()})
    );
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(body.clone())
                .insert_header("x-request-id", "trailing-json"),
        )
        .expect(1)
        .mount(&server)
        .await;
    let Error::Decode(error) = support::client(&server)
        .subscribers()
        .get("1")
        .await
        .unwrap_err()
    else {
        panic!("expected decode error")
    };
    assert_eq!(error.status().as_u16(), 200);
    assert_eq!(error.body(), body.as_bytes());
    assert_eq!(
        error.metadata().request_id.as_deref(),
        Some("trailing-json")
    );
    assert!(error.detail().contains("trailing"));
}

#[tokio::test]
async fn configured_response_cap_marks_short_diagnostics_as_truncated() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("x".repeat(64)))
        .expect(1)
        .mount(&server)
        .await;
    let sdk = mailerlite_rs::MailerLite::builder("test-token")
        .base_url(format!("{}/api", server.uri()))
        .max_response_bytes(16)
        .build()
        .unwrap();
    let Error::Decode(error) = sdk.subscribers().get("1").await.unwrap_err() else {
        panic!("expected bounded decode error")
    };
    assert_eq!(error.body(), b"xxxxxxxxxxxxxxxx");
    assert!(error.body_truncated());
    assert_eq!(error.status().as_u16(), 200);
}

#[tokio::test]
async fn response_exactly_at_configured_byte_limit_is_accepted() {
    let server = MockServer::start().await;
    let body = json!({"data":support::subscriber()}).to_string();
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body.clone()))
        .expect(1)
        .mount(&server)
        .await;
    let sdk = mailerlite_rs::MailerLite::builder("test-token")
        .base_url(format!("{}/api", server.uri()))
        .max_response_bytes(body.len())
        .build()
        .unwrap();
    assert_eq!(
        sdk.subscribers().get("1").await.unwrap().data.id.as_str(),
        "900719925474099312345"
    );
}
