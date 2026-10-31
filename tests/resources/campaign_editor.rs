//! HTTP contract regression tests for the migrated simple-editor API.
#![allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]
use crate::editor_support::{
    assert_no_request, expect_no_request, expect_request, fixture, mailerlite,
};
use mailerlite_rs::{
    Error,
    resources::campaigns::{CreateCampaign, Email},
};
use mockito::Server;
use serde_json::json;
fn draft() -> CreateCampaign {
    CreateCampaign::new(
        "Simple editor campaign",
        Email::new("Hello", "Sender", "sender@example.com"),
    )
}
#[tokio::test]
async fn create_sends_simple_editor_and_template_as_json() {
    let mut server = Server::new_async().await;
    let mock = expect_request(&mut server,"POST","/campaigns",Some(json!({"name":"Simple editor campaign","type":"regular","emails":[{"subject":"Hello","from_name":"Sender","from":"sender@example.com","editor":"simple","template_id":"42"}]})),201,&fixture("campaign_simple_editor")).await;
    let response = mailerlite(&server)
        .campaigns()
        .create(draft().simple_editor_template("42"))
        .await
        .unwrap();
    mock.assert_async().await;
    assert_eq!(response.data.default_email_id.as_deref(), Some("123"));
    assert_eq!(
        response.data.emails.unwrap()[0].kind.as_deref(),
        Some("builder_inline_email")
    );
}
#[tokio::test]
async fn ordinary_campaign_omits_editor_and_template() {
    let mut server = Server::new_async().await;
    let mock = expect_request(&mut server,"POST","/campaigns",Some(json!({"name":"Simple editor campaign","type":"regular","emails":[{"subject":"Hello","from_name":"Sender","from":"sender@example.com"}]})),201,&fixture("campaign_simple_editor")).await;
    mailerlite(&server)
        .campaigns()
        .create(draft())
        .await
        .unwrap();
    mock.assert_async().await;
}
#[tokio::test]
async fn simple_editor_rejects_html_before_dispatch() {
    let mut server = Server::new_async().await;
    let mocks = expect_no_request(&mut server).await;
    let draft = CreateCampaign::new(
        "Draft",
        Email::new("Subject", "Sender", "sender@example.com").content("<p>HTML</p>"),
    )
    .simple_editor();
    assert!(matches!(
        mailerlite(&server).campaigns().create(draft).await,
        Err(Error::Config(_))
    ));
    assert_no_request(&mocks).await;
}
