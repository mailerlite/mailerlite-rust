//! Automation draft and subscriber activity contracts.
use crate::support::{default_client, resource_mock};
use mailerlite_rs::{
    Error,
    resources::automations::{ActivityStatus, CreateAutomation},
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

fn automation() -> Value {
    json!({"id":"002","name":"Automation","enabled":false})
}

#[tokio::test]
async fn list_filters_by_enabled_state_and_group() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/automations"))
        .and(query_param("filter[enabled]", "true"))
        .and(query_param("filter[group]", "004"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[automation()]})))
        .expect(1)
        .mount(&server)
        .await;
    let response = default_client(&server)
        .automations()
        .list()
        .enabled(true)
        .group("004")
        .send()
        .await
        .unwrap();
    assert_eq!(response.data.data.len(), 1);
}

#[tokio::test]
async fn get_uses_the_automation_identifier_path() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/002",
        200,
        Some(json!({"data":automation()})),
        None,
    )
    .await;
    assert_eq!(
        default_client(&server)
            .automations()
            .get("002")
            .await
            .unwrap()
            .data
            .id
            .as_str(),
        "002"
    );
}

#[tokio::test]
async fn create_sends_a_name_and_accepts_a_disabled_201_draft() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/automations",
        201,
        Some(json!({"data":automation()})),
        Some(json!({"name":"Automation"})),
    )
    .await;
    let response = default_client(&server)
        .automations()
        .create(CreateAutomation::new("Automation"))
        .await
        .unwrap();
    assert!(!response.data.enabled);
    assert_eq!(response.metadata.status.as_u16(), 201);
}

#[tokio::test]
async fn activity_sends_required_status_and_date_filter() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/automations/002/activity"))
        .and(query_param("filter[status]", "completed"))
        .and(query_param("filter[date_from]", "2026-01-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"data":[{"id":"run","status":"Completed","stepRuns":[],"nextStep":null}]}),
        ))
        .expect(1)
        .mount(&server)
        .await;
    let response = default_client(&server)
        .automations()
        .activity("002", ActivityStatus::Completed)
        .date_from("2026-01-01")
        .send()
        .await
        .unwrap();
    assert_eq!(response.data.data[0].step_runs.as_ref().unwrap().len(), 0);
    assert!(response.data.data[0].next_step.is_none());
}

#[tokio::test]
async fn delete_accepts_empty_204() {
    let server = MockServer::start().await;
    resource_mock(&server, "DELETE", "/api/automations/002", 204, None, None).await;
    assert_eq!(
        default_client(&server)
            .automations()
            .delete("002")
            .await
            .unwrap()
            .metadata
            .status
            .as_u16(),
        204
    );
}

#[tokio::test]
async fn list_rejects_zero_page_before_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .automations()
            .list()
            .page(0)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn activity_rejects_zero_limit_before_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .automations()
            .activity("002", ActivityStatus::Active)
            .limit(0)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn get_returns_not_found_without_a_model_decode_failure() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/missing",
        404,
        Some(json!({"message":"Automation not found"})),
        None,
    )
    .await;
    let Error::Api(error) = default_client(&server)
        .automations()
        .get("missing")
        .await
        .unwrap_err()
    else {
        panic!("expected API error")
    };
    assert_eq!(error.status().as_u16(), 404);
    assert_eq!(error.message(), Some("Automation not found"));
}

#[tokio::test]
async fn list_keeps_an_empty_terminal_page_without_inventing_continuation() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations",
        200,
        Some(json!({"data":[],"meta":{"current_page":1,"last_page":1}})),
        None,
    )
    .await;
    let response = default_client(&server)
        .automations()
        .list()
        .send()
        .await
        .unwrap();
    assert!(response.data.data.is_empty());
    assert_eq!(response.data.next_page(), None);
}
