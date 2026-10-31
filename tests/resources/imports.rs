use crate::support::{default_client, resource_mock};
use mailerlite_rs::{Error, models::imports::ImportReceipt, resources::imports::ImportSubscriber};
use serde_json::{Value, json};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

fn report(done: bool) -> Value {
    json!({"data":{"id":"123","total":1,"processed":1,"imported":1,"updated":0,"errored":0,"percent":100,"done":done}})
}
fn receipt(server: &MockServer) -> ImportReceipt {
    ImportReceipt {
        import_progress_url: format!("{}/api/subscribers/import/123", server.uri()),
    }
}

#[tokio::test]
async fn import_to_group_serializes_fields_and_returns_an_acceptance_receipt() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/groups/7/import-subscribers",
        200,
        Some(json!({"import_progress_url":receipt(&server).import_progress_url})),
        Some(json!({"subscribers":[{"email":"jane@example.test","fields":{"name":"Jane"}}]})),
    )
    .await;
    let response = default_client(&server)
        .imports()
        .import_to_group(
            "7",
            vec![ImportSubscriber::new("jane@example.test").field("name", "Jane")],
        )
        .await
        .unwrap();
    assert_eq!(
        response.data.import_progress_url,
        receipt(&server).import_progress_url
    );
}

#[tokio::test]
async fn get_fetches_a_report_by_identifier() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/subscribers/import/123",
        200,
        Some(report(true)),
        None,
    )
    .await;
    assert!(
        default_client(&server)
            .imports()
            .get("123")
            .await
            .unwrap()
            .data
            .done
    );
}

#[tokio::test]
async fn progress_fetches_a_validated_receipt() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/subscribers/import/123",
        200,
        Some(report(true)),
        None,
    )
    .await;
    assert!(
        default_client(&server)
            .imports()
            .progress(&receipt(&server))
            .await
            .unwrap()
            .data
            .done
    );
}

#[tokio::test]
async fn wait_returns_immediately_for_an_already_complete_report() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/subscribers/import/123",
        200,
        Some(report(true)),
        None,
    )
    .await;
    assert!(
        default_client(&server)
            .imports()
            .wait(
                &receipt(&server),
                Duration::from_millis(1),
                Duration::from_secs(1)
            )
            .await
            .unwrap()
            .data
            .done
    );
}

#[tokio::test]
async fn wait_polls_an_incomplete_report_until_completion() {
    let server = MockServer::start().await;
    let count = Arc::new(AtomicUsize::new(0));
    let calls = Arc::clone(&count);
    Mock::given(method("GET"))
        .and(path("/api/subscribers/import/123"))
        .respond_with(move |_: &wiremock::Request| {
            ResponseTemplate::new(200)
                .set_body_json(report(calls.fetch_add(1, Ordering::SeqCst) > 0))
        })
        .expect(2)
        .mount(&server)
        .await;
    let response = default_client(&server)
        .imports()
        .wait(
            &receipt(&server),
            Duration::from_millis(1),
            Duration::from_secs(5),
        )
        .await
        .unwrap();
    assert!(response.data.done);
    assert_eq!(count.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn wait_propagates_report_failure_without_polling_again() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/subscribers/import/123",
        404,
        Some(json!({"message":"Import not found"})),
        None,
    )
    .await;
    let Error::Api(error) = default_client(&server)
        .imports()
        .wait(
            &receipt(&server),
            Duration::from_millis(1),
            Duration::from_secs(5),
        )
        .await
        .unwrap_err()
    else {
        panic!("expected API error")
    };
    assert_eq!(error.status().as_u16(), 404);
}

#[tokio::test]
async fn progress_never_sends_credentials_to_another_origin() {
    let server = MockServer::start().await;
    let other = MockServer::start().await;
    let result = default_client(&server)
        .imports()
        .progress(&receipt(&other))
        .await;
    assert!(matches!(result, Err(Error::Config(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
    assert!(other.received_requests().await.unwrap().is_empty());
}

async fn assert_invalid_receipt_suffix(suffix: &str) {
    let server = MockServer::start().await;
    let value = ImportReceipt {
        import_progress_url: format!("{}{suffix}", server.uri()),
    };
    assert!(matches!(
        default_client(&server).imports().progress(&value).await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn progress_rejects_an_unrelated_path_without_network() {
    assert_invalid_receipt_suffix("/api/groups/123").await;
}

#[tokio::test]
async fn progress_rejects_queries_without_network() {
    assert_invalid_receipt_suffix("/api/subscribers/import/123?token=bad").await;
}

#[tokio::test]
async fn progress_rejects_encoded_traversal_without_network() {
    assert_invalid_receipt_suffix("/api/subscribers/import/%2e%2e").await;
}

#[tokio::test]
async fn progress_rejects_fragments_without_network() {
    assert_invalid_receipt_suffix("/api/subscribers/import/123#ignored").await;
}

#[tokio::test]
async fn progress_rejects_url_credentials_without_network() {
    let server = MockServer::start().await;
    let value = ImportReceipt {
        import_progress_url: receipt(&server)
            .import_progress_url
            .replace("http://", "http://private:secret@"),
    };
    assert!(matches!(
        default_client(&server).imports().progress(&value).await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn import_to_group_rejects_empty_input_without_network() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .imports()
            .import_to_group("7", vec![])
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn import_to_group_does_not_retry_when_safe_read_retries_are_enabled() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/groups/7/import-subscribers"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    let result = crate::support::client(&server)
        .imports()
        .import_to_group("7", vec![ImportSubscriber::new("jane@example.test")])
        .await;
    assert!(matches!(result, Err(Error::Api(_))));
}

#[tokio::test]
async fn import_to_group_rejects_an_unsafe_receipt_without_following_it() {
    let server = MockServer::start().await;
    let other = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/groups/7/import-subscribers",
        200,
        Some(json!({"import_progress_url":receipt(&other).import_progress_url})),
        None,
    )
    .await;
    assert!(matches!(
        default_client(&server)
            .imports()
            .import_to_group("7", vec![ImportSubscriber::new("jane@example.test")])
            .await,
        Err(Error::Config(_))
    ));
    assert!(other.received_requests().await.unwrap().is_empty());
}

async fn assert_invalid_wait_durations(interval: Duration, timeout: Duration) {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .imports()
            .wait(&receipt(&server), interval, timeout)
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn wait_rejects_overflowing_interval_without_network() {
    assert_invalid_wait_durations(Duration::MAX, Duration::from_secs(1)).await;
}

#[tokio::test]
async fn wait_rejects_overflowing_timeout_without_network() {
    assert_invalid_wait_durations(Duration::from_secs(1), Duration::MAX).await;
}

#[tokio::test]
async fn wait_rejects_zero_interval_without_network() {
    assert_invalid_wait_durations(Duration::ZERO, Duration::from_secs(1)).await;
}

#[tokio::test]
async fn wait_rejects_zero_timeout_without_network() {
    assert_invalid_wait_durations(Duration::from_secs(1), Duration::ZERO).await;
}

#[tokio::test]
async fn get_rejects_malformed_known_outcome_sample_instead_of_retaining_untyped_json() {
    let server = MockServer::start().await;
    let mut value = report(true);
    value["data"]["invalid"] = json!([{"id":"001","email":false}]);
    resource_mock(
        &server,
        "GET",
        "/api/subscribers/import/123",
        200,
        Some(value),
        None,
    )
    .await;
    let Error::Decode(error) = default_client(&server)
        .imports()
        .get("123")
        .await
        .unwrap_err()
    else {
        panic!("expected typed sample decode error")
    };
    assert!(error.detail().contains("invalid[0].email"));
}
