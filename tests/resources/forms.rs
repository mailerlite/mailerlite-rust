//! Form metadata and cursor subscriber contracts.
use crate::support::{default_client, resource_mock};
use mailerlite_rs::{
    Error,
    resources::forms::{FormType, UpdateForm},
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

fn form() -> Value {
    json!({"id":"003","name":"Form","type":"embedded"})
}

#[tokio::test]
async fn get_rejects_malformed_double_optin_as_a_decode_error() {
    let server = MockServer::start().await;
    let mut value = form();
    value["double_optin"] = json!("false");
    resource_mock(
        &server,
        "GET",
        "/api/forms/003",
        200,
        Some(json!({"data":value})),
        None,
    )
    .await;
    let Error::Decode(error) = default_client(&server)
        .forms()
        .get("003")
        .await
        .unwrap_err()
    else {
        panic!("expected malformed boolean diagnostics")
    };
    assert_eq!(error.status().as_u16(), 200);
    assert!(error.detail().contains("double_optin"));
}

#[tokio::test]
async fn list_uses_required_type_in_path_and_name_filter() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/forms/embedded"))
        .and(query_param("filter[name]", "Form"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[form()]})))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(
        default_client(&server)
            .forms()
            .list(FormType::Embedded)
            .name("Form")
            .send()
            .await
            .unwrap()
            .data
            .data
            .len(),
        1
    );
}

#[tokio::test]
async fn get_decodes_form_metadata() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/forms/003",
        200,
        Some(json!({"data":form()})),
        None,
    )
    .await;
    assert_eq!(
        default_client(&server)
            .forms()
            .get("003")
            .await
            .unwrap()
            .data
            .kind,
        "embedded"
    );
}

#[tokio::test]
async fn update_sends_only_the_replacement_name() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "PUT",
        "/api/forms/003",
        200,
        Some(json!({"data":form()})),
        Some(json!({"name":"Renamed"})),
    )
    .await;
    default_client(&server)
        .forms()
        .update("003", UpdateForm::new("Renamed"))
        .await
        .unwrap();
}

#[tokio::test]
async fn subscribers_preserve_opaque_cursor_and_empty_page_continuation() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/forms/003/subscribers"))
        .and(query_param("cursor", "opaque+/="))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[],"meta":{"next_cursor":"next"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = default_client(&server)
        .forms()
        .subscribers("003")
        .cursor("opaque+/=")
        .send()
        .await
        .unwrap();
    assert!(response.data.data.is_empty());
    assert_eq!(response.data.next_cursor(), Some("next"));
}

#[tokio::test]
async fn delete_accepts_empty_204() {
    let server = MockServer::start().await;
    resource_mock(&server, "DELETE", "/api/forms/003", 204, None, None).await;
    assert_eq!(
        default_client(&server)
            .forms()
            .delete("003")
            .await
            .unwrap()
            .metadata
            .status
            .as_u16(),
        204
    );
}

#[tokio::test]
async fn list_rejects_zero_limit_before_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .forms()
            .list(FormType::Popup)
            .limit(0)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn subscribers_reject_zero_limit_before_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .forms()
            .subscribers("003")
            .limit(0)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn get_encodes_an_identifier_as_one_path_segment() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/forms/a%2Fb",
        200,
        Some(json!({"data":form()})),
        None,
    )
    .await;
    default_client(&server).forms().get("a/b").await.unwrap();
}

#[tokio::test]
async fn update_preserves_name_validation_details() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "PUT",
        "/api/forms/003",
        422,
        Some(json!({"message":"Invalid name","errors":{"name":["The name is required"]}})),
        Some(json!({"name":""})),
    )
    .await;
    let Error::Api(error) = default_client(&server)
        .forms()
        .update("003", UpdateForm::new(""))
        .await
        .unwrap_err()
    else {
        panic!("expected API error")
    };
    assert!(error.is_validation());
    assert_eq!(error.validation_errors()["name"], ["The name is required"]);
}
