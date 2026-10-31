//! Automation template pagination and asynchronous selection.
use crate::support::{client, default_client};
use mailerlite_rs::Error;
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};
#[tokio::test]
async fn list_preserves_category_type_and_numbered_pagination() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/automation-templates"))
        .and(query_param("filter[type]","welcome")).and(query_param("filter[category]","007"))
        .and(query_param("limit","10")).and(query_param("page","1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"003","name":"Welcome","type":"welcome","workflow":{"steps":[]},"description":null,"excerpt":"Welcome new readers","workflow_steps_description":[],"url":null}],"meta":{"current_page":1,"last_page":1,"per_page":10}})))
        .expect(1).mount(&server).await;
    let response = default_client(&server)
        .automation_templates()
        .list()
        .kind("welcome")
        .category("007")
        .limit(10)
        .page(1)
        .send()
        .await
        .unwrap();
    assert_eq!(response.data.data[0].id, "003");
    assert_eq!(response.data.next_page(), None);
}
#[tokio::test]
async fn select_preserves_background_build_flags() {
    let server = MockServer::start().await;
    Mock::given(method("POST")).and(path("/api/automations/select-template/003"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({"data":{"id":"004","name":"Welcome","enabled":false,"building_in_progress":true,"trigger_data":{"building_in_progress":true}}})))
        .expect(1).mount(&server).await;
    let response = default_client(&server)
        .automation_templates()
        .select("003")
        .await
        .unwrap();
    assert_eq!(response.data.building_in_progress, Some(true));
    assert_eq!(
        response.data.trigger_data.unwrap().building_in_progress,
        Some(true)
    );
}
#[tokio::test]
async fn select_never_retries_even_with_safe_read_retries_enabled() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/select-template/003"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    assert!(matches!(
        client(&server).automation_templates().select("003").await,
        Err(Error::Api(_))
    ));
}

#[tokio::test]
async fn list_rejects_zero_page_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .automation_templates()
            .list()
            .page(0)
            .send()
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
            .automation_templates()
            .list()
            .limit(0)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn invalid_stream_page_yields_one_error_without_http() {
    use futures_util::{StreamExt, pin_mut};
    let server = MockServer::start().await;
    let sdk = default_client(&server);
    let stream = sdk.automation_templates().list().page(0).into_stream();
    pin_mut!(stream);
    assert!(matches!(stream.next().await, Some(Err(Error::Config(_)))));
    assert!(stream.next().await.is_none());
    assert!(server.received_requests().await.unwrap().is_empty());
}
