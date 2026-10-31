//! Trigger operations and execution subscriber contracts.
use crate::support::{default_client, resource_mock};
use mailerlite_rs::{
    Error,
    resources::automation_triggers::{
        CreateAutomationTrigger, TriggerSubscriberStatus, UpdateAutomationTrigger,
    },
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};
fn trigger() -> Value {
    json!({"id":"002","type":"subscriber_joins_group","group_ids":["003"],"exclude_group_ids":[],"broken":false,"groups":[{"id":"003","name":"Group"}]})
}

#[tokio::test]
async fn list_preserves_trigger_identifiers() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/001/triggers",
        200,
        Some(json!({"data":[trigger()]})),
        None,
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk.automation_triggers().list("001").await.unwrap();
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].id, "002");
}

#[tokio::test]
async fn get_preserves_group_ids_and_type_specific_settings() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/001/triggers/002",
        200,
        Some(json!({"data":trigger()})),
        None,
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk.automation_triggers().get("001", "002").await.unwrap();
    assert_eq!(response.data.group_ids.unwrap(), vec!["003"]);
    assert_eq!(
        response.data.settings.groups.as_ref().unwrap()[0]
            .name
            .as_deref(),
        Some("Group")
    );
}

#[tokio::test]
async fn run_counts_decodes_trigger_and_manual_execution_counts() {
    let server = MockServer::start().await;
    resource_mock(&server, "GET", "/api/automations/001/triggers/run-counts", 200, Some(json!({"data":{"triggers":[{"id":"002","completed_runs":7,"queued_runs":2}],"manual_runs":{"in_progress_runs":3,"completed_runs":4}}})), None).await;
    let sdk = default_client(&server);
    let response = sdk.automation_triggers().run_counts("001").await.unwrap();
    assert_eq!(response.data.triggers[0].counts.completed_runs, 7);
    assert_eq!(response.data.manual_runs.unwrap().in_progress_runs, 3);
}

#[tokio::test]
async fn run_count_decodes_queued_execution_count() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/001/triggers/run-counts/002",
        200,
        Some(json!({"data":{"completed_runs":7,"queued_runs":2}})),
        None,
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk
        .automation_triggers()
        .run_count("001", "002")
        .await
        .unwrap();
    assert_eq!(response.data.queued_runs, 2);
}

#[tokio::test]
async fn create_replaces_group_selection_and_retains_empty_exclusions() {
    let server = MockServer::start().await;
    resource_mock(&server, "POST", "/api/automations/001/triggers", 201, Some(json!({"data":trigger()})), Some(json!({"type":"subscriber_joins_group","data":{"group_ids":["003","004"],"exclude_group_ids":[]}}))).await;
    let sdk = default_client(&server);
    let response = sdk
        .automation_triggers()
        .create(
            "001",
            CreateAutomationTrigger::new("subscriber_joins_group")
                .group_ids(["old"])
                .group_ids(["003", "004"])
                .exclude_group_ids(Vec::<String>::new()),
        )
        .await
        .unwrap();
    assert_eq!(response.data.id, "002");
}

#[tokio::test]
async fn update_decodes_nested_trigger_and_qualification_count() {
    let server = MockServer::start().await;
    resource_mock(&server, "PUT", "/api/automations/001/triggers/002", 200, Some(json!({"data":{"trigger":trigger(),"trigger_data":{"repeatable":false},"qualified_subscribers_count":42}})), Some(json!({"data":{"group_ids":["003"]}}))).await;
    let sdk = default_client(&server);
    let response = sdk
        .automation_triggers()
        .update(
            "001",
            "002",
            UpdateAutomationTrigger::new().group_ids(["003"]),
        )
        .await
        .unwrap();
    assert_eq!(response.data.trigger.id, "002");
    assert_eq!(response.data.qualified_subscribers_count, Some(42));
}

#[tokio::test]
async fn delete_accepts_empty_204() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "DELETE",
        "/api/automations/001/triggers/002",
        204,
        None,
        None,
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk
        .automation_triggers()
        .delete("001", "002")
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 204);
}

#[tokio::test]
async fn subscribers_preserve_status_filters_and_numbered_pages() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/automations/001/activity/triggers/002/subscribers"))
        .and(query_param("status","completed")).and(query_param("limit","10"))
        .and(query_param("page","2")).and(query_param("sort","-email"))
        .and(query_param("filter[search]","a+b@example.test"))
        .and(query_param("filter[date_from]","2026-01-01"))
        .and(query_param("filter[date_to]","2026-01-31"))
        .and(query_param("filter[id]","001,002"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"001","subscriber":{"id":"002","email":"a+b@example.test"},"date":"2026-01-10 12:00:00"}],"meta":{"current_page":2,"last_page":3,"per_page":10},"links":{}})))
        .expect(1).mount(&server).await;
    let response = default_client(&server)
        .automation_triggers()
        .subscribers("001", "002", TriggerSubscriberStatus::Completed)
        .limit(10)
        .page(2)
        .sort("-email")
        .search("a+b@example.test")
        .date_from("2026-01-01")
        .date_to("2026-01-31")
        .ids(["001", "002"])
        .send()
        .await
        .unwrap();
    assert_eq!(response.data.data[0].subscriber.id, "002");
    assert_eq!(response.data.next_page(), Some(3));
}

#[tokio::test]
async fn subscribers_reject_zero_limit_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .automation_triggers()
            .subscribers("001", "002", TriggerSubscriberStatus::Queued)
            .limit(0)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn subscribers_reject_limit_above_maximum_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .automation_triggers()
            .subscribers("001", "002", TriggerSubscriberStatus::Queued)
            .limit(1001)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn subscribers_reject_zero_page_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .automation_triggers()
            .subscribers("001", "002", TriggerSubscriberStatus::Queued)
            .page(0)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn subscribers_reject_unsupported_sort_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .automation_triggers()
            .subscribers("001", "002", TriggerSubscriberStatus::Queued)
            .sort("name")
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn run_counts_accepts_absent_manual_trigger() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/001/triggers/run-counts",
        200,
        Some(json!({"data":{"triggers":[],"manual_runs":null}})),
        None,
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk.automation_triggers().run_counts("001").await.unwrap();
    assert!(response.data.triggers.is_empty());
    assert!(response.data.manual_runs.is_none());
}

#[tokio::test]
async fn update_preserves_explicit_null_and_boolean_configuration() {
    let server = MockServer::start().await;
    resource_mock(&server, "PUT", "/api/automations/001/triggers/002", 200, Some(json!({"data":{"trigger":trigger(),"trigger_data":null,"qualified_subscribers_count":null}})), Some(json!({"data":{"exclude_group_ids":null,"exit_when_no_longer_matches":false}}))).await;
    default_client(&server)
        .automation_triggers()
        .update(
            "001",
            "002",
            UpdateAutomationTrigger::new()
                .configuration(mailerlite_rs::models::automation_configuration::TriggerConfiguration::Group(mailerlite_rs::models::automation_configuration::GroupTriggerConfiguration { exclude_group_ids: mailerlite_rs::models::Patch::Null, exit_when_no_longer_matches: false.into(), ..Default::default() })).unwrap(),
        )
        .await
        .unwrap();
}
#[cfg(feature = "stream")]
#[tokio::test]
async fn subscriber_stream_preserves_status_and_filters_across_pages() {
    use futures_util::{StreamExt, pin_mut};
    let server = MockServer::start().await;
    for page in 1..=2 {
        Mock::given(method("GET")).and(path("/api/automations/001/activity/triggers/002/subscribers"))
            .and(query_param("status", "queued")).and(query_param("filter[search]", "a+b@example.test"))
            .and(query_param("filter[id]", "001,002")).and(query_param("sort", "-date"))
            .and(query_param("filter[date_from]", "2026-01-01")).and(query_param("filter[date_to]", "2026-01-31"))
            .and(query_param("limit", "1")).and(query_param("page", page.to_string()))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":format!("00{page}"),"subscriber":{"id":"002","email":"a+b@example.test"},"date":"2026-01-10 12:00:00"}],"meta":{"current_page":page,"last_page":2}})))
            .expect(1).mount(&server).await;
    }
    let sdk = default_client(&server);
    let stream = sdk
        .automation_triggers()
        .subscribers("001", "002", TriggerSubscriberStatus::Queued)
        .search("a+b@example.test")
        .ids(["001", "002"])
        .sort("-date")
        .date_from("2026-01-01")
        .date_to("2026-01-31")
        .limit(1)
        .page(1)
        .into_stream();
    pin_mut!(stream);
    assert_eq!(stream.next().await.unwrap().unwrap().id, "001");
    assert_eq!(stream.next().await.unwrap().unwrap().id, "002");
    assert!(stream.next().await.is_none());
}
