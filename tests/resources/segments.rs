//! Segment listing, updates, deletion, and membership pagination.
use crate::support::{default_client, resource_mock};
use mailerlite_rs::{
    Error, models::subscribers::SubscriberStatus, resources::segments::UpdateSegment,
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

fn segment() -> Value {
    json!({"id":"003","name":"Segment","total":1})
}

#[tokio::test]
async fn list_sends_page_and_limit() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/segments"))
        .and(query_param("page", "1"))
        .and(query_param("limit", "25"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[segment()]})))
        .expect(1)
        .mount(&server)
        .await;
    default_client(&server)
        .segments()
        .list()
        .page(1)
        .limit(25)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn update_sends_only_replacement_name() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "PUT",
        "/api/segments/003",
        200,
        Some(json!({"data":segment()})),
        Some(json!({"name":"Renamed"})),
    )
    .await;
    default_client(&server)
        .segments()
        .update("003", UpdateSegment::new("Renamed"))
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_accepts_empty_204() {
    let server = MockServer::start().await;
    resource_mock(&server, "DELETE", "/api/segments/003", 204, None, None).await;
    default_client(&server)
        .segments()
        .delete("003")
        .await
        .unwrap();
}

#[tokio::test]
async fn subscribers_preserve_cursor_and_unsubscribed_filter() {
    let server = MockServer::start().await;
    let subscriber: Value =
        serde_json::from_str(include_str!("../fixtures/audience/subscriber.json")).unwrap();
    Mock::given(method("GET"))
        .and(path("/api/segments/003/subscribers"))
        .and(query_param("cursor", "segment-cursor"))
        .and(query_param("filter[status]", "unsubscribed"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[subscriber]})))
        .expect(1)
        .mount(&server)
        .await;
    default_client(&server)
        .segments()
        .subscribers("003")
        .cursor("segment-cursor")
        .status(SubscriberStatus::Unsubscribed)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn subscribers_preserve_legacy_after_id_without_inventing_a_cursor() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/segments/003/subscribers"))
        .and(query_param("after", "00042"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .expect(1)
        .mount(&server)
        .await;
    let response = default_client(&server)
        .segments()
        .subscribers("003")
        .after("00042")
        .send()
        .await
        .unwrap();
    assert!(response.data.data.is_empty());
    assert_eq!(response.data.next_cursor(), None);
}

#[tokio::test]
async fn subscribers_reject_group_only_include_parameter_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .segments()
            .subscribers("003")
            .include_groups()
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn update_rejects_empty_name_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .segments()
            .update("003", UpdateSegment::new(""))
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn list_rejects_zero_limit_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .segments()
            .list()
            .limit(0)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn list_accepts_corrected_page_and_limit() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/segments"))
        .and(query_param("page", "2"))
        .and(query_param("limit", "25"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .expect(1)
        .mount(&server)
        .await;
    let client = default_client(&server);

    client
        .segments()
        .list()
        .page(0)
        .limit(0)
        .page(2)
        .limit(25)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn corrected_limit_does_not_clear_invalid_page() {
    let server = MockServer::start().await;
    let client = default_client(&server);

    let result = client
        .segments()
        .list()
        .page(0)
        .limit(0)
        .limit(25)
        .send()
        .await;

    assert!(matches!(result, Err(Error::Config(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn corrected_page_does_not_clear_invalid_limit() {
    let server = MockServer::start().await;
    let client = default_client(&server);

    let result = client
        .segments()
        .list()
        .limit(0)
        .page(0)
        .page(2)
        .send()
        .await;

    assert!(matches!(result, Err(Error::Config(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn stream_accepts_corrected_page_and_limit() {
    use futures_util::TryStreamExt;
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/segments"))
        .and(query_param("page", "2"))
        .and(query_param("limit", "25"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .expect(1)
        .mount(&server)
        .await;
    let client = default_client(&server);
    let stream = client
        .segments()
        .list()
        .page(0)
        .limit(0)
        .page(2)
        .limit(25)
        .into_stream();
    tokio::pin!(stream);

    assert!(stream.try_next().await.unwrap().is_none());
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn stream_preserves_unrelated_validation_failure_after_correction() {
    use futures_util::TryStreamExt;
    let server = MockServer::start().await;
    let client = default_client(&server);
    let stream = client
        .segments()
        .list()
        .page(0)
        .limit(0)
        .limit(25)
        .into_stream();
    tokio::pin!(stream);

    assert!(matches!(stream.try_next().await, Err(Error::Config(_))));
    assert!(stream.try_next().await.unwrap().is_none());
    assert!(server.received_requests().await.unwrap().is_empty());
}
