//! Subscriber operations, cursor parameters, model compatibility, and validation.
use crate::support::{default_client, resource_mock};
use mailerlite_rs::{
    Error,
    models::subscribers::{FieldValue, SubscriberStatus},
    resources::subscribers::{ActivityKind, UpdateSubscriber, UpsertSubscriber},
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

fn subscriber() -> Value {
    serde_json::from_str(include_str!("../fixtures/audience/subscriber.json")).unwrap()
}

#[tokio::test]
async fn upsert_sends_fields_groups_and_resubscribe_and_decodes_created_subscriber() {
    let server = MockServer::start().await;
    resource_mock(&server, "POST", "/api/subscribers", 201,
        Some(json!({"data":subscriber()})),
        Some(json!({"email":"jane@example.com","fields":{"name":"Jane"},"groups":["001"],"resubscribe":false}))).await;
    let response = default_client(&server)
        .subscribers()
        .upsert(
            UpsertSubscriber::new("jane@example.com")
                .field("name", "Jane")
                .groups(["001"])
                .resubscribe(false),
        )
        .await
        .unwrap();
    assert_eq!(response.data.id.as_str(), "31986843064993537");
    assert_eq!(response.metadata.status.as_u16(), 201);
}

#[tokio::test]
async fn update_sends_explicit_null_custom_field_and_empty_replacement_groups() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "PUT",
        "/api/subscribers/31986843064993537",
        200,
        Some(json!({"data":subscriber()})),
        Some(json!({"fields":{"last_name":null},"groups":[]})),
    )
    .await;
    default_client(&server)
        .subscribers()
        .update(
            "31986843064993537",
            UpdateSubscriber::new()
                .field("last_name", FieldValue::Null)
                .groups(Vec::<String>::new()),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_accepts_empty_204() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "DELETE",
        "/api/subscribers/31986843064993537",
        204,
        None,
        None,
    )
    .await;
    let response = default_client(&server)
        .subscribers()
        .delete("31986843064993537")
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 204);
}

#[tokio::test]
async fn forget_preserves_asynchronous_receipt_message() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/subscribers/31986843064993537/forget",
        200,
        Some(json!({"message":"Scheduled for forgetting","data":subscriber()})),
        None,
    )
    .await;
    let receipt = default_client(&server)
        .subscribers()
        .forget("31986843064993537")
        .await
        .unwrap();
    assert_eq!(receipt.data.message, "Scheduled for forgetting");
}

#[tokio::test]
async fn get_accepts_email_identifiers_with_plus_tags() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/subscribers/jane+tag@example.com",
        200,
        Some(json!({"data":subscriber()})),
        None,
    )
    .await;
    default_client(&server)
        .subscribers()
        .get("jane+tag@example.com")
        .await
        .unwrap();
}

#[tokio::test]
async fn count_sends_zero_limit_and_decodes_top_level_total() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/subscribers"))
        .and(query_param("limit", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":123})))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(
        default_client(&server)
            .subscribers()
            .count()
            .await
            .unwrap()
            .data
            .total,
        123
    );
}

#[tokio::test]
async fn list_preserves_status_cursor_limit_and_group_inclusion() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/subscribers"))
        .and(query_param("filter[status]", "active"))
        .and(query_param("cursor", "opaque+/="))
        .and(query_param("limit", "50"))
        .and(query_param("include", "groups"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[subscriber()],"meta":{"next_cursor":"next"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = default_client(&server)
        .subscribers()
        .list()
        .status(SubscriberStatus::Active)
        .cursor("opaque+/=")
        .limit(50)
        .include_groups()
        .send()
        .await
        .unwrap();
    assert_eq!(response.data.next_cursor(), Some("next"));
}

#[tokio::test]
async fn activity_sends_manual_page_and_log_filter() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/subscribers/1/activity-log"))
        .and(query_param("page", "2"))
        .and(query_param("limit", "10"))
        .and(query_param("filter[log_name]", "email_open"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(
                json!({"data":[{"id":"2","log_name":"email_open","properties":[]}]}),
            ),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = default_client(&server)
        .subscribers()
        .activity("1")
        .page(2)
        .limit(10)
        .log_name(ActivityKind::EmailOpen)
        .send()
        .await
        .unwrap();
    assert_eq!(response.data.len(), 1);
}

#[tokio::test]
async fn upsert_preserves_422_field_validation_details() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/subscribers",
        422,
        Some(json!({"message":"Invalid data","errors":{"email":["Invalid email"]}})),
        Some(json!({"email":"invalid"})),
    )
    .await;
    let Error::Api(error) = default_client(&server)
        .subscribers()
        .upsert(UpsertSubscriber::new("invalid"))
        .await
        .unwrap_err()
    else {
        panic!("expected API validation error")
    };
    assert_eq!(error.status().as_u16(), 422);
    assert_eq!(error.validation_errors()["email"], ["Invalid email"]);
}

#[tokio::test]
async fn unknown_request_status_is_rejected_without_http() {
    let server = MockServer::start().await;
    let params = UpsertSubscriber::new("jane@example.com")
        .status(SubscriberStatus::Unknown("future-status".into()));
    assert!(serde_json::to_value(&params).is_err());
    assert!(matches!(
        default_client(&server).subscribers().upsert(params).await,
        Err(Error::Serialize(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn invalid_stream_limit_yields_one_error_without_http() {
    use futures_util::TryStreamExt;
    let server = MockServer::start().await;
    let client = default_client(&server);
    let subscribers = client.subscribers();
    let stream = subscribers.list().limit(0).into_stream();
    tokio::pin!(stream);
    assert!(matches!(stream.try_next().await, Err(Error::Config(_))));
    assert!(stream.try_next().await.unwrap().is_none());
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn empty_list_with_null_cursor_is_a_terminal_page() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/subscribers",
        200,
        Some(json!({"data":[],"meta":{"next_cursor":null}})),
        None,
    )
    .await;
    let response = default_client(&server)
        .subscribers()
        .list()
        .send()
        .await
        .unwrap();
    assert!(response.data.data.is_empty());
    assert_eq!(response.data.next_cursor(), None);
}

#[tokio::test]
async fn missing_subscriber_preserves_404_instead_of_decoding_a_subscriber() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/subscribers/absent",
        404,
        Some(json!({"message":"Subscriber not found"})),
        None,
    )
    .await;
    let Error::Api(error) = default_client(&server)
        .subscribers()
        .get("absent")
        .await
        .unwrap_err()
    else {
        panic!("expected API error")
    };
    assert_eq!(error.status().as_u16(), 404);
    assert_eq!(error.message(), Some("Subscriber not found"));
}

#[tokio::test]
async fn activity_rejects_zero_page_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .subscribers()
            .activity("1")
            .page(0)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn activity_malformed_known_email_properties_return_decode_errors() {
    let server = MockServer::start().await;
    resource_mock(&server, "GET", "/api/subscribers/1/activity-log", 200, Some(json!({"data":[{"id":"2","log_name":"email_open","properties":{"type":"campaigns","campaign_id":false,"preview_url":""}}]})), None).await;
    let Error::Decode(error) = default_client(&server)
        .subscribers()
        .activity("1")
        .send()
        .await
        .unwrap_err()
    else {
        panic!("expected known activity decode error")
    };
    assert_eq!(error.status().as_u16(), 200);
    assert!(!error.body().is_empty());
}
