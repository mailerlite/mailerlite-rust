mod common;

use common::{fixture, mailerlite};
use mailerlite_rs::{campaign::EDITOR_SIMPLE, data::Data, response::Response, MailerLite};
use mockito::{Matcher, Mock, Server, ServerGuard};
use serde_json::json;

#[tokio::test]
async fn find_uses_the_custom_base_url() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = server
        .mock("GET", "/campaigns/1")
        .match_header("Authorization", "Bearer api_key")
        .with_status(200)
        .with_body(r#"{"data": {"id": "1"}}"#)
        .create_async()
        .await;

    let mailerlite: MailerLite = MailerLite::new("api_key".to_string()).with_base_url(server.url());
    let response: Response = mailerlite.campaign().find("1").await;

    mock.assert_async().await;
    assert_eq!(response.status_code, 200);
    assert_eq!(response.content, json!({"data": {"id": "1"}}));
}

const BASE_FORM: &str = "name=Simple+editor+campaign&type=regular\
    &emails%5B0%5D%5Bsubject%5D=Hello\
    &emails%5B0%5D%5Bfrom_name%5D=Sender\
    &emails%5B0%5D%5Bfrom%5D=sender%40example.com";

fn base_data() -> Data {
    Data::new()
        .add("name", "Simple editor campaign")
        .add("type", "regular")
        .add("emails[0][subject]", "Hello")
        .add("emails[0][from_name]", "Sender")
        .add("emails[0][from]", "sender@example.com")
}

#[tokio::test]
async fn create_sends_the_simple_editor_and_the_template() {
    let mut server: ServerGuard = Server::new_async().await;
    let body: String =
        format!("{BASE_FORM}&emails%5B0%5D%5Beditor%5D=simple&emails%5B0%5D%5Btemplate_id%5D=42");
    let mock: Mock = server
        .mock("POST", "/campaigns")
        .match_body(Matcher::Exact(body))
        .with_status(201)
        .with_body(fixture("campaign_simple_editor"))
        .create_async()
        .await;

    let data: Data = base_data()
        .add("emails[0][editor]", EDITOR_SIMPLE)
        .add("emails[0][template_id]", "42");
    let response: Response = mailerlite(&server).campaign().create(data).await;

    mock.assert_async().await;
    assert_eq!(response.status_code, 201);
    let email = &response.content["data"]["emails"][0];
    assert_eq!(email["id"], "123");
    assert_eq!(email["type"], "builder_inline_email");
    assert_eq!(response.content["data"]["default_email_id"], "123");
}

#[tokio::test]
async fn create_without_the_editor_sends_the_same_body_as_before() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = server
        .mock("POST", "/campaigns")
        .match_body(Matcher::Exact(BASE_FORM.to_string()))
        .with_status(201)
        .with_body(r#"{"data": {"id": "456"}}"#)
        .create_async()
        .await;

    let response: Response = mailerlite(&server).campaign().create(base_data()).await;

    mock.assert_async().await;
    assert_eq!(response.status_code, 201);
}

#[test]
fn the_simple_editor_value_is_simple() {
    assert_eq!(EDITOR_SIMPLE, "simple");
}
