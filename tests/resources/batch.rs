use crate::support::default_client;
use mailerlite_rs::{
    Error,
    resources::batch::{BatchMethod, BatchRequest},
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path},
};

#[tokio::test]
async fn send_preserves_partial_failures_and_redacts_bodies() {
    let server = MockServer::start().await;
    Mock::given(method("POST")).and(path("/api/batch"))
        .and(body_json(json!({"requests":[{"method":"GET","path":"api/groups"},{"method":"POST","path":"api/subscribers","body":{"email":"invalid"}}]})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":2,"successful":1,"failed":1,"responses":[{"code":200,"body":{"data":[]}},{"code":422,"body":{"message":"echoed-secret","errors":{"email":["invalid"]}}}]})))
        .expect(1).mount(&server).await;
    let response = default_client(&server)
        .batch()
        .send(vec![
            BatchRequest::new(BatchMethod::Get, "api/groups").unwrap(),
            BatchRequest::new(BatchMethod::Post, "api/subscribers")
                .unwrap()
                .body(json!({"email":"invalid"}))
                .unwrap(),
        ])
        .await
        .unwrap();
    assert!(response.data.responses[0].is_success());
    assert!(!response.data.responses[1].is_success());
    assert_eq!(
        response.data.responses[1].body["errors"]["email"][0],
        "invalid"
    );
    assert!(!format!("{response:?}").contains("echoed-secret"));
}

#[tokio::test]
async fn send_rejects_empty_batch_without_network() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server).batch().send(vec![]).await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn send_rejects_fifty_one_requests_without_network() {
    let server = MockServer::start().await;
    let request = BatchRequest::new(BatchMethod::Get, "api/groups").unwrap();
    assert!(matches!(
        default_client(&server)
            .batch()
            .send(vec![request; 51])
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn send_accepts_exactly_fifty_requests() {
    let server = MockServer::start().await;
    let items = vec![json!({"method":"GET","path":"api/groups?limit=1"}); 50];
    Mock::given(method("POST")).and(path("/api/batch"))
        .and(body_json(json!({"requests":items})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":50,"successful":50,"failed":0,"responses":vec![json!({"code":200,"body":{"data":[]}});50]})))
        .expect(1).mount(&server).await;
    let request = BatchRequest::new(BatchMethod::Get, "api/groups?limit=1").unwrap();
    assert_eq!(
        default_client(&server)
            .batch()
            .send(vec![request; 50])
            .await
            .unwrap()
            .data
            .responses
            .len(),
        50
    );
}

#[tokio::test]
async fn send_does_not_retry_write_batches_when_safe_read_retries_are_enabled() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/batch"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    let result = crate::support::client(&server)
        .batch()
        .send(vec![
            BatchRequest::new(BatchMethod::Post, "api/subscribers")
                .unwrap()
                .body(json!({"email":"jane@example.test"}))
                .unwrap(),
        ])
        .await;
    assert!(matches!(result, Err(Error::Api(_))));
}

#[tokio::test]
async fn send_does_not_retry_a_read_only_batch() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/batch"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    let result = crate::support::client(&server)
        .batch()
        .send(vec![
            BatchRequest::new(BatchMethod::Get, "api/groups").unwrap(),
        ])
        .await;
    assert!(matches!(result, Err(Error::Api(_))));
}

fn paced_import_client(server: &MockServer) -> mailerlite_rs::MailerLite {
    use std::time::Duration;
    mailerlite_rs::MailerLite::builder("test-token")
        .base_url(format!("{}/api", server.uri()))
        .rate_limit_intervals(Duration::ZERO, Duration::from_secs(60))
        .operation_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn upsert_batch_request() -> BatchRequest {
    // A query suffix must not disguise an upsert's rate-limit classification.
    // The empty query does not imply an additional supported API parameter.
    BatchRequest::new(BatchMethod::Post, "api/subscribers?")
        .unwrap()
        .body(json!({"email":"reader@example.test"}))
        .unwrap()
}

#[tokio::test]
async fn subscriber_batch_reserves_the_bucket_used_by_group_imports_on_cloned_clients() {
    use mailerlite_rs::resources::imports::ImportSubscriber;
    use std::time::Duration;
    let server = MockServer::start().await;
    Mock::given(method("POST")).and(path("/api/batch"))
        .and(body_json(json!({"requests":[{"method":"POST","path":"api/subscribers?","body":{"email":"reader@example.test"}}]})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":1,"successful":1,"failed":0,"responses":[{"code":200,"body":{"data":crate::support::subscriber()}}]})))
        .expect(1).mount(&server).await;
    let sdk = paced_import_client(&server);
    sdk.batch()
        .send(vec![upsert_batch_request()])
        .await
        .unwrap();

    // Pause only after network IO has completed. The next call must expire
    // while waiting for the already-reserved bucket, before any HTTP request.
    tokio::time::pause();
    let started = tokio::time::Instant::now();
    let result = sdk
        .clone()
        .imports()
        .import_to_group("7", vec![ImportSubscriber::new("reader@example.test")])
        .await;
    assert!(matches!(result, Err(Error::Timeout)));
    // Tokio timers round to millisecond ticks; assert the semantic boundary,
    // not nanosecond equality after pausing a previously running clock.
    assert!(started.elapsed() >= Duration::from_secs(5));
    assert!(started.elapsed() < Duration::from_secs(60));
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn group_import_reserves_the_bucket_used_by_subscriber_batches() {
    use mailerlite_rs::resources::imports::ImportSubscriber;
    use std::time::Duration;
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/groups/7/import-subscribers"))
        .and(body_json(
            json!({"subscribers":[{"email":"reader@example.test"}]}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"import_progress_url":format!("{}/api/subscribers/import/123",server.uri())}),
        ))
        .expect(1)
        .mount(&server)
        .await;
    let sdk = paced_import_client(&server);
    sdk.imports()
        .import_to_group("7", vec![ImportSubscriber::new("reader@example.test")])
        .await
        .unwrap();
    tokio::time::pause();
    let started = tokio::time::Instant::now();
    assert!(matches!(
        sdk.batch().send(vec![upsert_batch_request()]).await,
        Err(Error::Timeout)
    ));
    // Tokio timers round to millisecond ticks; assert the semantic boundary,
    // not nanosecond equality after pausing a previously running clock.
    assert!(started.elapsed() >= Duration::from_secs(5));
    assert!(started.elapsed() < Duration::from_secs(60));
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}
