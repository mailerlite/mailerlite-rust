//! Automation mutations, request models, and email content contracts.
use crate::support;
use mailerlite_rs::{
    Error,
    models::Patch,
    resources::{automation_actions::*, automations::ActivityStatus},
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, body_string, header, method, path},
};

#[tokio::test]
async fn automation_update_submits_flat_trigger_replacement_and_typed_settings() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/api/automations/001"))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({
            "name":"Updated automation",
            "trigger_data":{"repeatable":true,"track_ecommerce":false,"show_analytics":true,
                "repeatable_settings":{"mode":"wait","value":2,"unit":"day"}},
            "triggers":[{"id":"003","type":"subscriber_joins_group","group_ids":["004"],
                "exclude_group_ids":null,"exit_when_no_longer_matches":true}]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{
            "id":"001","name":"Updated automation","enabled":false
        }})))
        .expect(1)
        .mount(&server)
        .await;
    let params = UpdateAutomation::new("Updated automation")
        .trigger_data(
            AutomationTriggerSettings::new()
                .repeatable(true)
                .track_ecommerce(false)
                .show_analytics(true)
                .repeatable_settings(
                    AutomationRepeatableSettings::wait(2, AutomationRepeatableUnit::Day).unwrap(),
                ),
        )
        .triggers(vec![
            AutomationTriggerDefinition::new("subscriber_joins_group")
                .id("003")
                .group_ids(["004"])
                .exclude_group_ids(Patch::Null)
                .configuration(mailerlite_rs::models::automation_configuration::TriggerConfiguration::Group(mailerlite_rs::models::automation_configuration::GroupTriggerConfiguration { group_ids: vec!["004".to_owned()].into(), exclude_group_ids: Patch::Null, exit_when_no_longer_matches: true.into() })).unwrap(),
        ]);
    let response = support::client(&server)
        .automations()
        .update("001", params)
        .await
        .unwrap();
    assert_eq!(response.data.name, "Updated automation");
}

#[tokio::test]
async fn update_sends_a_name_and_decodes_the_automation() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/api/automations/001"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({"name":"A & B"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":{"id":"001","name":"A & B","enabled":false}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .update("001", UpdateAutomation::new("A & B"))
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
    assert_eq!(response.data.id.as_str(), "001");
}

#[tokio::test]
async fn copy_posts_an_empty_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/001/copy"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(body_string(""))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":{"id":"001","name":"A & B","enabled":false}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .copy("001")
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
    assert_eq!(response.data.id.as_str(), "001");
}

#[tokio::test]
async fn enable_posts_an_empty_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/001/enable"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(body_string(""))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":{"id":"001","name":"A & B","enabled":false}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .enable("001")
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
    assert_eq!(response.data.id.as_str(), "001");
}

#[tokio::test]
async fn disable_posts_an_empty_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/001/disable"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(body_string(""))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":{"id":"001","name":"A & B","enabled":false}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .disable("001")
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
    assert_eq!(response.data.id.as_str(), "001");
}

#[tokio::test]
async fn test_email_accepts_empty_204() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/001/test"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({"email":"reader+test@example.test"})))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .test("001", TestAutomation::new("reader+test@example.test"))
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 204);
}

#[tokio::test]
async fn add_subscribers_sends_emails_groups_and_start_step() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/001/add-subscribers"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(header("content-type", "application/json"))
        .and(body_json(
            json!({"emails":["reader@example.test"],"groups":["003"],"step_id":"004"}),
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .add_subscribers(
            "001",
            AddAutomationSubscribers::new()
                .email("reader@example.test")
                .group("003")
                .step_id("004"),
        )
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 204);
}

#[tokio::test]
async fn readd_subscribers_sends_destination_and_status_filter() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/001/readd"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(header("content-type", "application/json"))
        .and(body_json(
            json!({"filter":{"status":"completed"},"readd_subscribers_to":"last_completed_step"}),
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .readd_subscribers(
            "001",
            ReaddAutomationSubscribers::new(
                ReaddDestination::LastCompletedStep,
                AutomationActivityFilter::new(ActivityStatus::Completed),
            ),
        )
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 204);
}

#[tokio::test]
async fn add_subscribers_to_group_sends_destination_groups() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/001/activity/add-to-group"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(header("content-type", "application/json"))
        .and(body_json(
            json!({"filter":{"status":"active"},"destination_groups":["003"]}),
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .add_subscribers_to_group(
            "001",
            AddAutomationSubscribersToGroup::new(
                "003",
                AutomationActivityFilter::new(ActivityStatus::Active),
            ),
        )
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 204);
}

#[tokio::test]
async fn cancel_subscribers_sends_selected_run_ids() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/001/activity/cancel-automation"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(header("content-type", "application/json"))
        .and(body_json(
            json!({"filter":{"status":"active","id":["003"]}}),
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .cancel_subscribers(
            "001",
            AutomationActivityFilter::new(ActivityStatus::Active).id("003"),
        )
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 204);
}

#[tokio::test]
async fn skip_delay_sends_next_step_filter() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/001/activity/skip-delay"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(header("content-type", "application/json"))
        .and(body_json(
            json!({"filter":{"status":"active","next_step":"003"}}),
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .skip_delay(
            "001",
            AutomationActivityFilter::new(ActivityStatus::Active).next_step("003"),
        )
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 204);
}

#[tokio::test]
async fn email_content_decodes_html() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/automations/001/emails/002/content"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(body_string(""))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"data":{"html":"<p>Hello</p>"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .email_content("001", "002")
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
    assert_eq!(response.data.html, "<p>Hello</p>");
}

#[tokio::test]
async fn update_email_content_sends_html() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/api/automations/001/emails/002/content"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({"html":"<p>Hello & goodbye</p>"})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":{"id":"002","subject":"Hello"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .update_email_content(
            "001",
            "002",
            UpdateAutomationEmailContent::new("<p>Hello & goodbye</p>"),
        )
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
}

#[tokio::test]
async fn automation_actions_never_retry_writes_even_when_opted_in() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/1/enable"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    assert!(matches!(
        support::client(&server).automations().enable("1").await,
        Err(Error::Api(_))
    ));
}

#[tokio::test]
async fn action_ids_are_encoded_and_subscriber_arrays_are_json() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/automations/a%2Fb/add-subscribers"))
        .and(body_json(
            json!({"emails":["one@example.test","two@example.test"]}),
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    support::client(&server)
        .automations()
        .add_subscribers(
            "a/b",
            AddAutomationSubscribers::new()
                .email("one@example.test")
                .email("two@example.test"),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn add_subscribers_rejects_empty_selection() {
    let server = MockServer::start().await;
    let sdk = support::default_client(&server);
    assert!(matches!(
        sdk.automations()
            .add_subscribers("1", AddAutomationSubscribers::new())
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn cancel_subscribers_rejects_completed_status() {
    let server = MockServer::start().await;
    let sdk = support::default_client(&server);
    assert!(matches!(
        sdk.automations()
            .cancel_subscribers(
                "1",
                AutomationActivityFilter::new(ActivityStatus::Completed)
            )
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn skip_delay_rejects_failed_status() {
    let server = MockServer::start().await;
    let sdk = support::default_client(&server);
    assert!(matches!(
        sdk.automations()
            .skip_delay("1", AutomationActivityFilter::new(ActivityStatus::Failed))
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn readd_subscribers_requires_a_selected_step() {
    let server = MockServer::start().await;
    let sdk = support::default_client(&server);
    assert!(matches!(
        sdk.automations()
            .readd_subscribers(
                "1",
                ReaddAutomationSubscribers::new(
                    ReaddDestination::SelectedStep,
                    AutomationActivityFilter::new(ActivityStatus::Completed)
                )
            )
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}
