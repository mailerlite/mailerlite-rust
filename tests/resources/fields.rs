//! Custom field CRUD, list filters, and local name validation.
use crate::support::{default_client, resource_mock};
use mailerlite_rs::{
    Error,
    models::fields::FieldType,
    resources::{
        SortDirection,
        fields::{CreateField, FieldSort, UpdateField},
    },
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

fn field() -> Value {
    json!({"id":"004","name":"Birthday","key":"birthday","type":"date"})
}

#[tokio::test]
async fn list_sorts_by_type_ascending_without_a_prefix() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/fields"))
        .and(query_param("sort", "type"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[field()]})))
        .expect(1)
        .mount(&server)
        .await;
    default_client(&server)
        .fields()
        .list()
        .sort(FieldSort::Type, SortDirection::Ascending)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn create_sends_name_and_wire_type() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/fields",
        200,
        Some(json!({"data":field()})),
        Some(json!({"name":"Birthday","type":"date"})),
    )
    .await;
    default_client(&server)
        .fields()
        .create(CreateField::new("Birthday", FieldType::Date))
        .await
        .unwrap();
}

#[tokio::test]
async fn update_renames_without_changing_type() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "PUT",
        "/api/fields/004",
        200,
        Some(json!({"data":field()})),
        Some(json!({"name":"Birth date"})),
    )
    .await;
    default_client(&server)
        .fields()
        .update("004", UpdateField::new("Birth date"))
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_accepts_empty_204() {
    let server = MockServer::start().await;
    resource_mock(&server, "DELETE", "/api/fields/004", 204, None, None).await;
    default_client(&server)
        .fields()
        .delete("004")
        .await
        .unwrap();
}

#[tokio::test]
async fn list_preserves_page_limit_keyword_type_and_descending_sort() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/fields"))
        .and(query_param("page", "1"))
        .and(query_param("limit", "25"))
        .and(query_param("filter[keyword]", "birth"))
        .and(query_param("filter[type]", "date"))
        .and(query_param("sort", "-name"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[field()]})))
        .expect(1)
        .mount(&server)
        .await;
    default_client(&server)
        .fields()
        .list()
        .page(1)
        .limit(25)
        .keyword("birth")
        .field_type(FieldType::Date)
        .sort(FieldSort::Name, SortDirection::Descending)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn create_rejects_empty_name_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .fields()
            .create(CreateField::new("", FieldType::Text))
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn list_preserves_unknown_response_field_types() {
    let server = MockServer::start().await;
    let mut value = field();
    value["type"] = json!("future-field-type");
    resource_mock(
        &server,
        "GET",
        "/api/fields",
        200,
        Some(json!({"data":[value]})),
        None,
    )
    .await;
    let response = default_client(&server)
        .fields()
        .list()
        .send()
        .await
        .unwrap();
    assert_eq!(response.data.data[0].field_type, "future-field-type");
}

#[tokio::test]
async fn update_rejects_overlong_name_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .fields()
            .update("004", UpdateField::new("x".repeat(256)))
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn list_rejects_zero_page_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server).fields().list().page(0).send().await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn list_accepts_corrected_page_and_limit() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/fields"))
        .and(query_param("page", "2"))
        .and(query_param("limit", "25"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .expect(1)
        .mount(&server)
        .await;
    let client = default_client(&server);

    client
        .fields()
        .list()
        .page(0)
        .limit(0)
        .page(2)
        .limit(25)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn corrected_limit_does_not_clear_invalid_page() {
    let server = MockServer::start().await;
    let client = default_client(&server);

    let result = client
        .fields()
        .list()
        .page(0)
        .limit(0)
        .limit(25)
        .send()
        .await;

    assert!(matches!(result, Err(Error::Config(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn corrected_page_does_not_clear_invalid_limit() {
    let server = MockServer::start().await;
    let client = default_client(&server);

    let result = client.fields().list().limit(0).page(0).page(2).send().await;

    assert!(matches!(result, Err(Error::Config(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn stream_accepts_corrected_page_and_limit() {
    use futures_util::TryStreamExt;
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/fields"))
        .and(query_param("page", "2"))
        .and(query_param("limit", "25"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .expect(1)
        .mount(&server)
        .await;
    let client = default_client(&server);
    let stream = client
        .fields()
        .list()
        .page(0)
        .limit(0)
        .page(2)
        .limit(25)
        .into_stream();
    tokio::pin!(stream);

    assert!(stream.try_next().await.unwrap().is_none());
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn stream_preserves_unrelated_validation_failure_after_correction() {
    use futures_util::TryStreamExt;
    let server = MockServer::start().await;
    let client = default_client(&server);
    let stream = client
        .fields()
        .list()
        .page(0)
        .limit(0)
        .limit(25)
        .into_stream();
    tokio::pin!(stream);

    assert!(matches!(stream.try_next().await, Err(Error::Config(_))));
    assert!(stream.try_next().await.unwrap().is_none());
    assert!(server.received_requests().await.unwrap().is_empty());
}
