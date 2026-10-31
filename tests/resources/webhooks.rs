use crate::support::{default_client, resource_mock};
use mailerlite_rs::{
    Error,
    models::webhooks::WebhookEvent,
    resources::webhooks::{CreateWebhook, UpdateWebhook},
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

fn webhook() -> Value {
    json!({"id":"42","name":"hook","url":"https://example.test/callback","events":["subscriber.created"],"enabled":true,"secret":"signing-secret","batchable":false,"created_at":null,"updated_at":null})
}

#[tokio::test]
async fn list_preserves_page_metadata_and_redacts_signing_secret() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/webhooks",
        200,
        Some(json!({"data":[webhook()],"meta":{"current_page":1,"last_page":1}})),
        None,
    )
    .await;
    let response = default_client(&server).webhooks().list().await.unwrap();
    assert_eq!(response.data.data.len(), 1);
    assert!(!format!("{response:?}").contains("signing-secret"));
    assert_eq!(response.data.next_page(), None);
}

#[tokio::test]
async fn get_exposes_signing_secret_only_through_explicit_accessor() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/webhooks/42",
        200,
        Some(json!({"data":webhook()})),
        None,
    )
    .await;
    let response = default_client(&server).webhooks().get("42").await.unwrap();
    assert_eq!(response.data.secret.expose_secret(), "signing-secret");
    assert!(!format!("{response:?}").contains("signing-secret"));
}

#[tokio::test]
async fn create_serializes_callback_and_events_without_omitted_options() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/webhooks",
        200,
        Some(json!({"data":webhook()})),
        Some(json!({"url":"https://example.test/callback","events":["subscriber.created"]})),
    )
    .await;
    let response = default_client(&server)
        .webhooks()
        .create(CreateWebhook::new(
            "https://example.test/callback",
            [WebhookEvent::SubscriberCreated],
        ))
        .await
        .unwrap();
    assert_eq!(response.data.id.as_ref(), "42");
}

#[tokio::test]
async fn create_accepts_201_and_keeps_signing_secret_redacted() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/webhooks",
        201,
        Some(json!({"data":webhook()})),
        Some(json!({"url":"https://example.test/callback","events":["subscriber.created"]})),
    )
    .await;
    let response = default_client(&server)
        .webhooks()
        .create(CreateWebhook::new(
            "https://example.test/callback",
            [WebhookEvent::SubscriberCreated],
        ))
        .await
        .unwrap();

    assert_eq!(response.metadata.status.as_u16(), 201);
    assert_eq!(response.data.id.as_ref(), "42");
    assert_eq!(response.data.secret.expose_secret(), "signing-secret");
    assert!(!format!("{response:?}").contains("signing-secret"));
}

#[tokio::test]
async fn update_sends_explicit_false_without_replacing_other_properties() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "PUT",
        "/api/webhooks/42",
        200,
        Some(json!({"data":webhook()})),
        Some(json!({"enabled":false})),
    )
    .await;
    default_client(&server)
        .webhooks()
        .update("42", UpdateWebhook::new().enabled(false))
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_accepts_an_empty_204_response() {
    let server = MockServer::start().await;
    resource_mock(&server, "DELETE", "/api/webhooks/42", 204, None, None).await;
    assert_eq!(
        default_client(&server)
            .webhooks()
            .delete("42")
            .await
            .unwrap()
            .metadata
            .status
            .as_u16(),
        204
    );
}

#[tokio::test]
async fn create_rejects_batch_only_event_without_network() {
    let server = MockServer::start().await;
    let result = default_client(&server)
        .webhooks()
        .create(CreateWebhook::new(
            "https://example.test",
            [WebhookEvent::CampaignOpen],
        ))
        .await;
    assert!(matches!(result, Err(Error::Config(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn list_accepts_an_empty_collection_without_inventing_pagination() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/webhooks",
        200,
        Some(json!({"data":[]})),
        None,
    )
    .await;
    let response = default_client(&server).webhooks().list().await.unwrap();
    assert!(response.data.data.is_empty());
    assert_eq!(response.data.next_page(), None);
}

#[tokio::test]
async fn create_rejects_empty_events_without_network() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .webhooks()
            .create(CreateWebhook::new("https://example.test", []))
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn update_rejects_batch_only_event_without_explicit_batching() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .webhooks()
            .update(
                "42",
                UpdateWebhook::new().events([WebhookEvent::CampaignOpen])
            )
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn create_accepts_batch_only_event_when_batching_is_explicit() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/webhooks",
        200,
        Some(json!({"data":webhook()})),
        Some(json!({"url":"https://example.test","events":["campaign.open"],"batchable":true})),
    )
    .await;
    default_client(&server)
        .webhooks()
        .create(
            CreateWebhook::new("https://example.test", [WebhookEvent::CampaignOpen])
                .batchable(true),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_does_not_retry_a_rate_limit_even_with_retries_enabled() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/api/webhooks/42"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "0"))
        .expect(1)
        .mount(&server)
        .await;
    let Error::Api(error) = crate::support::client(&server)
        .webhooks()
        .delete("42")
        .await
        .unwrap_err()
    else {
        panic!("expected API error")
    };
    assert!(error.is_rate_limited());
}
