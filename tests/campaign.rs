use mailerlite_rs::{response::Response, MailerLite};
use mockito::{Mock, Server, ServerGuard};
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
