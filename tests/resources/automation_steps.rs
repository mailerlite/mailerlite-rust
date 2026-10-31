//! Workflow step operations and backend response contracts.
use crate::support::{default_client, resource_mock};
use mailerlite_rs::models::automation_configuration::{
    DelayConfiguration, DelayUnit, StepConfiguration,
};
use mailerlite_rs::{
    Error, MailerLite, RetryPolicy,
    models::automation_steps::StepValue,
    resources::automation_steps::{
        AbWinner, CopyStep, CreateStep, DeleteBranch, DeleteStep, ReorderSteps, StepBranch,
        StepSubscriberStatus, StepSubscribers, StepType, UpdateStep,
    },
};
use serde_json::{Value, json};
use std::time::Duration;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};
fn step() -> Value {
    json!({"id":"2","type":"delay","parent_id":null,"unit":"days","value":1,"complete":true,"description":"Wait 1 day","type_count":1})
}
fn counts() -> Value {
    json!({"step_id":"2","completed_step_runs":9,"in_queue_step_runs":3})
}
fn stats() -> Value {
    json!({"step_id":"2","name":"Test","ab_test_winner":null,"parent_id":null,"can_select_winner":true,"a_path_step_id":"3","b_path_step_id":"4","c_path_step_id":null,"split_percentages":[{"label":"A","path":"a_path","value":50},{"label":"B","path":"b_path","value":50}],"stats":{"a_path":{"subscribers":5,"sent":4,"opens":2,"clicks":1,"open_rate":{"float":0.5,"string":"50%"},"click_rate":{"float":0.25,"string":"25%"},"click_to_open_rate":{"float":0.5,"string":"50%"}}}})
}
fn automation() -> Value {
    json!({"id":"1","name":"Workflow","enabled":false,"steps":[step()]})
}

#[tokio::test]
async fn list_decodes_workflow_order() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/1/steps/index",
        200,
        Some(json!({"data":[step()]})),
        None,
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk.automation_steps().list("1").await.unwrap();
    assert_eq!(response.data.len(), 1);
}

#[tokio::test]
async fn get_preserves_numeric_values_and_unknown_properties() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/1/steps/2",
        200,
        Some(json!({"data":step()})),
        None,
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk.automation_steps().get("1", "2").await.unwrap();
    assert!(matches!(response.data.value, Some(StepValue::Number(_))));
    assert_eq!(response.data.properties.type_count, Some(1));
}

#[tokio::test]
async fn run_counts_decodes_completed_execution_counts() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/1/steps/run-counts",
        200,
        Some(json!({"data":[counts()]})),
        None,
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk.automation_steps().run_counts("1").await.unwrap();
    assert_eq!(response.data[0].completed_step_runs, 9);
}

#[tokio::test]
async fn run_count_decodes_queued_execution_count() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/1/steps/run-counts/2",
        200,
        Some(json!({"data":counts()})),
        None,
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk.automation_steps().run_count("1", "2").await.unwrap();
    assert_eq!(response.data.in_queue_step_runs, 3);
}

#[tokio::test]
async fn ab_stats_decodes_all_split_results() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/1/ab-steps-stats",
        200,
        Some(json!({"data":[stats()]})),
        None,
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk.automation_steps().ab_stats("1").await.unwrap();
    assert_eq!(response.data.len(), 1);
}

#[tokio::test]
async fn ab_stat_decodes_path_metrics() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/automations/1/ab-steps-stats/2",
        200,
        Some(json!({"data":stats()})),
        None,
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk.automation_steps().ab_stat("1", "2").await.unwrap();
    assert_eq!(response.data.stats["a_path"].sent, 4);
}

#[tokio::test]
async fn create_sends_nested_delay_configuration() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/automations/1/steps",
        200,
        Some(json!({"data":step()})),
        Some(json!({"type":"delay","parent_id":"3","data":{"unit":"days","value":1}})),
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk
        .automation_steps()
        .create(
            "1",
            CreateStep::new(StepType::Delay)
                .parent_id("3")
                .configuration(StepConfiguration::Delay(DelayConfiguration::new(
                    DelayUnit::Days,
                    1,
                )))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.data.id, "2");
}

#[tokio::test]
async fn update_sends_configuration_and_rescheduling_flag() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "PUT",
        "/api/automations/1/steps/2",
        200,
        Some(json!({"data":step()})),
        Some(json!({"data":{"unit":"days","value":2},"reschedule_waiting_subscribers":true})),
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk
        .automation_steps()
        .update(
            "1",
            "2",
            UpdateStep::new()
                .configuration(StepConfiguration::Delay(DelayConfiguration::new(
                    DelayUnit::Days,
                    2,
                )))
                .unwrap()
                .reschedule_waiting_subscribers(true),
        )
        .await
        .unwrap();
    assert_eq!(response.data.id, "2");
}

#[tokio::test]
async fn copy_sends_parent_and_branch_and_returns_automation() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/automations/1/steps/2/copy",
        200,
        Some(json!({"data":automation()})),
        Some(json!({"parent_id":"3","branch":"yes"})),
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk
        .automation_steps()
        .copy(
            "1",
            "2",
            CopyStep::new().parent_id("3").branch(StepBranch::Yes),
        )
        .await
        .unwrap();
    assert_eq!(response.data.name, "Workflow");
}

#[tokio::test]
async fn reorder_sends_moved_step_and_parent() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/automations/1/steps/reorder",
        200,
        Some(json!({"data":automation()})),
        Some(json!({"dragged_step_id":"3","new_parent_step_id":"4"})),
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk
        .automation_steps()
        .reorder("1", ReorderSteps::new("3").new_parent_step_id("4"))
        .await
        .unwrap();
    assert_eq!(response.data.name, "Workflow");
}

#[tokio::test]
async fn select_ab_test_winner_sends_wire_path() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "PUT",
        "/api/automations/1/steps/2/select-ab-test-winner",
        200,
        Some(json!({"data":stats()})),
        Some(json!({"ab_test_winner":"a_path"})),
    )
    .await;
    let sdk = default_client(&server);
    let response = sdk
        .automation_steps()
        .select_ab_test_winner("1", "2", AbWinner::APath)
        .await
        .unwrap();
    assert_eq!(response.data.stats["a_path"].sent, 4);
}

#[tokio::test]
async fn delete_sends_branch_and_accepts_empty_204() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/api/automations/1/steps/2"))
        .and(query_param("branch_to_delete", "all"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(
        default_client(&server)
            .automation_steps()
            .delete(
                "1",
                "2",
                DeleteStep::new().branch_to_delete(DeleteBranch::All)
            )
            .await
            .unwrap()
            .metadata
            .status
            .as_u16(),
        204
    );
}
#[tokio::test]
async fn subscribers_preserve_filters_and_numbered_pagination() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/automations/1/activity/steps/2/subscribers"))
        .and(query_param("status","queued")).and(query_param("filter[search]","reader+tag@example.test"))
        .and(query_param("limit","10")).and(query_param("page","1"))
        .and(query_param("sort","-date")).and(query_param("filter[date_from]","2026-01-01"))
        .and(query_param("filter[id]","11,12"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"11","subscriber":{"id":"7","email":"reader+tag@example.test"},"date":"2026-01-02 12:00:00"}],"meta":{"current_page":1,"last_page":2}})))
        .expect(1).mount(&server).await;
    let response = default_client(&server)
        .automation_steps()
        .subscribers(
            "1",
            "2",
            StepSubscribers::new(StepSubscriberStatus::Queued)
                .search("reader+tag@example.test")
                .limit(10)
                .page(1)
                .sort("-date")
                .date_from("2026-01-01")
                .ids(["11", "12"]),
        )
        .await
        .unwrap();
    assert_eq!(response.data.next_page(), Some(2));
    assert_eq!(response.data.data[0].subscriber.id, "7");
}
#[tokio::test]
async fn copy_never_retries_even_with_safe_read_retries_enabled() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/1/steps/2/copy"))
        .respond_with(ResponseTemplate::new(503).set_body_string("{}"))
        .expect(1)
        .mount(&server)
        .await;
    let sdk = MailerLite::builder("step-test-token")
        .base_url(format!("{}/api", server.uri()))
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .retry_policy(RetryPolicy::safe_reads().initial_delay(Duration::ZERO))
        .build()
        .unwrap();
    assert!(
        matches!(sdk.automation_steps().copy("1", "2", CopyStep::new()).await, Err(Error::Api(error)) if error.status().as_u16() == 503)
    );
}

#[tokio::test]
async fn subscribers_reject_limit_above_maximum_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .automation_steps()
            .subscribers(
                "1",
                "2",
                StepSubscribers::new(StepSubscriberStatus::Queued).limit(1001)
            )
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
            .automation_steps()
            .subscribers(
                "1",
                "2",
                StepSubscribers::new(StepSubscriberStatus::Completed).page(0)
            )
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn subscribers_reject_zero_limit_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .automation_steps()
            .subscribers(
                "1",
                "2",
                StepSubscribers::new(StepSubscriberStatus::Queued).limit(0)
            )
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
            .automation_steps()
            .subscribers(
                "1",
                "2",
                StepSubscribers::new(StepSubscriberStatus::Completed).sort("name")
            )
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn update_sends_explicit_null_parent_and_nested_json_types() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "PUT",
        "/api/automations/1/steps/2",
        200,
        Some(json!({"data":step()})),
        Some(json!({"parent_id":null,"data":{"track_opens":false,"preheader":null}})),
    )
    .await;
    default_client(&server)
        .automation_steps()
        .update(
            "1",
            "2",
            UpdateStep::new()
                .clear_parent()
                .configuration(StepConfiguration::Email(
                    mailerlite_rs::models::automation_configuration::EmailConfiguration {
                        track_opens: false.into(),
                        preheader: mailerlite_rs::models::Patch::Null,
                        ..Default::default()
                    },
                ))
                .unwrap(),
        )
        .await
        .unwrap();
}
#[tokio::test]
async fn create_preserves_backend_validation_errors() {
    let server = MockServer::start().await;
    resource_mock(&server, "POST", "/api/automations/1/steps", 422, Some(json!({"message":"Invalid delay", "errors":{"data.value":["The value must be positive."]}})), Some(json!({"type":"delay","data":{"value":0}}))).await;
    let error = default_client(&server)
        .automation_steps()
        .create(
            "1",
            CreateStep::new(StepType::Delay)
                .configuration(StepConfiguration::Delay(DelayConfiguration {
                    value:
                        mailerlite_rs::models::automation_configuration::ConfigurationValue::from(
                            0u64,
                        )
                        .into(),
                    ..Default::default()
                }))
                .unwrap(),
        )
        .await
        .unwrap_err();
    let Error::Api(error) = error else {
        panic!("expected API validation error")
    };
    assert_eq!(error.status().as_u16(), 422);
    assert_eq!(
        error.validation_errors()["data.value"],
        ["The value must be positive."]
    );
}
