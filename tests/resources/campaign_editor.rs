//! HTTP contracts for simple-editor campaign creation.
#![allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]
use crate::editor_support::{assert_no_request, expect_request, fixture, mailerlite};
use mailerlite_rs::{
    Error,
    resources::campaigns::{CreateCampaign, Email},
};
use serde_json::json;
use wiremock::MockServer;
fn draft() -> CreateCampaign {
    CreateCampaign::new(
        "Simple editor campaign",
        Email::new("Hello", "Sender", "sender@example.com"),
    )
}
#[tokio::test]
async fn create_sends_simple_editor_and_template_as_json() {
    let server = MockServer::start().await;
    let mock = expect_request(&server,"POST","/campaigns",Some(json!({"name":"Simple editor campaign","type":"regular","emails":[{"subject":"Hello","from_name":"Sender","from":"sender@example.com","editor":"simple","template_id":"42"}]})),201,&fixture("campaign_simple_editor")).await;
    let response = mailerlite(&server)
        .campaigns()
        .create(
            draft()
                .simple_editor()
                .simple_editor_template("old")
                .simple_editor_template("42"),
        )
        .await
        .unwrap();
    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(response.data.default_email_id.as_deref(), Some("123"));
    assert_eq!(
        response.data.emails.unwrap()[0].kind,
        Some(mailerlite_rs::models::emails::EmailType::SimpleEditor)
    );
}
#[tokio::test]
async fn ordinary_campaign_omits_editor_and_template() {
    let server = MockServer::start().await;
    let mock = expect_request(&server,"POST","/campaigns",Some(json!({"name":"Simple editor campaign","type":"regular","emails":[{"subject":"Hello","from_name":"Sender","from":"sender@example.com"}]})),201,&fixture("campaign_simple_editor")).await;
    mailerlite(&server)
        .campaigns()
        .create(draft())
        .await
        .unwrap();
    assert_eq!(mock.received_requests().await.len(), 1);
}
#[tokio::test]
async fn simple_editor_rejects_html_before_dispatch() {
    let server = MockServer::start().await;

    let draft = CreateCampaign::new(
        "Draft",
        Email::new("Subject", "Sender", "sender@example.com").content("<p>HTML</p>"),
    )
    .simple_editor();
    assert!(matches!(
        mailerlite(&server).campaigns().create(draft).await,
        Err(Error::Config(_))
    ));
    assert_no_request(&server).await;
}

#[tokio::test]
async fn simple_editor_clears_a_previous_template_selection() {
    let server = MockServer::start().await;
    let mock = expect_request(
        &server, "POST", "/campaigns",
        Some(json!({"name":"Simple editor campaign","type":"regular","emails":[{"subject":"Hello","from_name":"Sender","from":"sender@example.com","editor":"simple"}]})),
        201, &fixture("campaign_simple_editor"),
    ).await;

    mailerlite(&server)
        .campaigns()
        .create(draft().simple_editor_template("42").simple_editor())
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
}
