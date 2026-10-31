//! Group CRUD, membership contracts, and documented boundaries.
use crate::support::{default_client, resource_mock};
use mailerlite_rs::{
    Error,
    models::subscribers::SubscriberStatus,
    resources::{
        SortDirection,
        groups::{CreateGroup, GroupSort, UpdateGroup},
    },
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

fn group() -> Value {
    json!({"id":"001","name":"Newsletter"})
}

#[tokio::test]
async fn list_sorts_by_name_ascending_without_a_prefix() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/groups"))
        .and(query_param("sort", "name"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[group()]})))
        .expect(1)
        .mount(&server)
        .await;
    default_client(&server)
        .groups()
        .list()
        .sort(GroupSort::Name, SortDirection::Ascending)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn create_sends_name_and_accepts_201() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/groups",
        201,
        Some(json!({"data":group()})),
        Some(json!({"name":"Newsletter"})),
    )
    .await;
    default_client(&server)
        .groups()
        .create(CreateGroup::new("Newsletter"))
        .await
        .unwrap();
}

#[tokio::test]
async fn update_sends_replacement_name() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "PUT",
        "/api/groups/001",
        200,
        Some(json!({"data":group()})),
        Some(json!({"name":"Renamed"})),
    )
    .await;
    default_client(&server)
        .groups()
        .update("001", UpdateGroup::new("Renamed"))
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_accepts_empty_204() {
    let server = MockServer::start().await;
    resource_mock(&server, "DELETE", "/api/groups/001", 204, None, None).await;
    default_client(&server)
        .groups()
        .delete("001")
        .await
        .unwrap();
}

#[tokio::test]
async fn assign_places_subscriber_before_group_in_path() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/subscribers/2/groups/001",
        200,
        Some(json!({"data":group()})),
        None,
    )
    .await;
    default_client(&server)
        .groups()
        .assign("001", "2")
        .await
        .unwrap();
}

#[tokio::test]
async fn unassign_accepts_empty_204() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "DELETE",
        "/api/subscribers/2/groups/001",
        204,
        None,
        None,
    )
    .await;
    default_client(&server)
        .groups()
        .unassign("001", "2")
        .await
        .unwrap();
}

#[tokio::test]
async fn list_preserves_page_limit_name_filter_and_descending_sort() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/groups"))
        .and(query_param("page", "2"))
        .and(query_param("limit", "25"))
        .and(query_param("filter[name]", "News"))
        .and(query_param("sort", "-total"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[group()],"meta":{"current_page":2,"last_page":3}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = default_client(&server)
        .groups()
        .list()
        .page(2)
        .limit(25)
        .name("News")
        .sort(GroupSort::Total, SortDirection::Descending)
        .send()
        .await
        .unwrap();
    assert_eq!(response.data.next_page(), Some(3));
}

#[tokio::test]
async fn subscribers_accept_maximum_page_size_and_preserve_filters() {
    let server = MockServer::start().await;
    let subscriber: Value =
        serde_json::from_str(include_str!("../fixtures/audience/subscriber.json")).unwrap();
    Mock::given(method("GET"))
        .and(path("/api/groups/001/subscribers"))
        .and(query_param("cursor", "group-cursor"))
        .and(query_param("limit", "1000"))
        .and(query_param("filter[status]", "active"))
        .and(query_param("include", "groups"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[subscriber]})))
        .expect(1)
        .mount(&server)
        .await;
    default_client(&server)
        .groups()
        .subscribers("001")
        .cursor("group-cursor")
        .limit(1000)
        .status(SubscriberStatus::Active)
        .include_groups()
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn subscribers_reject_page_size_above_maximum_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .groups()
            .subscribers("1")
            .limit(1001)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn list_rejects_zero_page_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server).groups().list().page(0).send().await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn create_accepts_255_unicode_characters_not_just_255_bytes() {
    let server = MockServer::start().await;
    let name = "é".repeat(255);
    resource_mock(
        &server,
        "POST",
        "/api/groups",
        201,
        Some(json!({"data":{"id":"001","name":name}})),
        Some(json!({"name":name})),
    )
    .await;
    assert_eq!(
        default_client(&server)
            .groups()
            .create(CreateGroup::new(name))
            .await
            .unwrap()
            .data
            .name
            .chars()
            .count(),
        255
    );
}

#[tokio::test]
async fn create_rejects_256_character_name_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .groups()
            .create(CreateGroup::new("a".repeat(256)))
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn subscribers_reject_segment_only_after_parameter_without_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .groups()
            .subscribers("1")
            .after("2")
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}
