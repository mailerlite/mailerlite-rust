//! Lazy pagination preserves query context without trusting response destinations.
#![cfg(feature = "stream")]
#![allow(clippy::unwrap_used, clippy::expect_used)]
mod support;
use futures_util::{StreamExt, TryStreamExt};
use mailerlite_rs::{
    Error,
    models::subscribers::SubscriberStatus,
    resources::{SortDirection, groups::GroupSort},
};
use serde_json::json;
use support::{client, subscriber};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

#[tokio::test]
async fn cursor_stream_continues_empty_pages_preserves_filters_and_ignores_hostile_links() {
    let origin = MockServer::start().await;
    let hostile = MockServer::start().await;
    let sdk = client(&origin);
    Mock::given(method("GET")).and(path("/api/subscribers"))
        .and(query_param("filter[status]","active")).and(query_param("limit","100"))
        .and(|request: &wiremock::Request| !request.url.query_pairs().any(|(key,_)|key=="cursor"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[],"meta":{"next_cursor":"opaque+/=?"},"links":{"next":format!("{}/steal",hostile.uri())}})))
        .expect(1).mount(&origin).await;
    Mock::given(method("GET"))
        .and(path("/api/subscribers"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(query_param("filter[status]", "active"))
        .and(query_param("limit", "100"))
        .and(query_param("cursor", "opaque+/=?"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[subscriber()],"meta":{"next_cursor":null}})),
        )
        .expect(1)
        .mount(&origin)
        .await;
    let subscribers = sdk.subscribers();
    let stream = subscribers
        .list()
        .status(SubscriberStatus::Active)
        .limit(100)
        .into_stream();
    tokio::pin!(stream);
    assert_eq!(
        stream.try_next().await.unwrap().unwrap().id.as_ref(),
        "900719925474099312345"
    );
    assert!(stream.try_next().await.unwrap().is_none());
    assert!(hostile.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn constructing_or_dropping_stream_does_not_prefetch() {
    let server = MockServer::start().await;
    let sdk = client(&server);
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"data":[subscriber(),subscriber()],"meta":{"next_cursor":"next"}}),
        ))
        .expect(1)
        .mount(&server)
        .await;
    let subscribers = sdk.subscribers();
    let stream = subscribers.list().into_stream();
    assert!(server.received_requests().await.unwrap().is_empty());
    {
        tokio::pin!(stream);
        assert!(stream.try_next().await.unwrap().is_some());
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }
    tokio::task::yield_now().await;
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn repeated_cursor_stops_instead_of_requesting_forever() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[],"meta":{"next_cursor":"repeat"}})),
        )
        .expect(2)
        .mount(&server)
        .await;
    let sdk = client(&server);
    let subscribers = sdk.subscribers();
    let stream = subscribers.list().into_stream();
    tokio::pin!(stream);
    assert!(matches!(stream.try_next().await, Err(Error::Pagination(_))));
    assert!(stream.try_next().await.unwrap().is_none());
}

#[tokio::test]
async fn numbered_stream_retains_filters_and_uses_metadata_not_navigation_url() {
    let origin = MockServer::start().await;
    let hostile = MockServer::start().await;
    let sdk = client(&origin);
    Mock::given(method("GET"))
        .and(path("/api/groups"))
        .and(query_param("page", "2"))
        .and(query_param("limit", "25"))
        .and(query_param("filter[name]", "readers"))
        .and(query_param("sort", "-name"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [],
            "meta": {"current_page": 2, "last_page": 3},
            "links": {"next": format!("{}/steal?page=999", hostile.uri())}
        })))
        .expect(1)
        .mount(&origin)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/groups"))
        .and(query_param("page", "3"))
        .and(query_param("limit", "25"))
        .and(query_param("filter[name]", "readers"))
        .and(query_param("sort", "-name"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"data":[{"id":"12","name":"readers"}],"meta":{"current_page":3,"last_page":3}}),
        ))
        .expect(1)
        .mount(&origin)
        .await;
    let groups = sdk.groups();
    let stream = groups
        .list()
        .page(2)
        .limit(25)
        .name("readers")
        .sort(GroupSort::Name, SortDirection::Descending)
        .into_stream();
    tokio::pin!(stream);
    assert_eq!(stream.try_next().await.unwrap().unwrap().name, "readers");
    assert!(stream.try_next().await.unwrap().is_none());
    assert!(hostile.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn numbered_stream_rejects_a_mismatched_page_and_then_terminates() {
    let server = MockServer::start().await;
    let sdk = client(&server);
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[],"meta":{"current_page":1,"last_page":3}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let groups = sdk.groups();
    let stream = groups.list().page(2).into_stream();
    tokio::pin!(stream);
    assert!(matches!(stream.try_next().await, Err(Error::Pagination(_))));
    assert!(stream.try_next().await.unwrap().is_none());
}

#[tokio::test]
async fn numbered_stream_without_metadata_does_not_guess_from_navigation_links() {
    let server = MockServer::start().await;
    let sdk = client(&server);
    Mock::given(method("GET")).respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"1","name":"one"}],"links":{"next":"https://hostile.invalid/?page=2"}}))).expect(1).mount(&server).await;
    let groups = sdk.groups();
    let stream = groups.list().into_stream();
    tokio::pin!(stream);
    assert!(stream.try_next().await.unwrap().is_some());
    assert!(stream.try_next().await.unwrap().is_none());
}

#[tokio::test]
async fn subscriber_stream_rejects_zero_limit_without_http_and_then_terminates() {
    let server = MockServer::start().await;
    let sdk = client(&server);
    let subscribers = sdk.subscribers();
    let stream = subscribers.list().limit(0).into_stream();
    tokio::pin!(stream);
    assert!(matches!(stream.next().await, Some(Err(Error::Config(_)))));
    assert!(stream.next().await.is_none());
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn campaign_stream_rejects_zero_limit_without_http_and_then_terminates() {
    let server = MockServer::start().await;
    let sdk = client(&server);
    let campaigns = sdk.campaigns();
    let stream = campaigns.list().limit(0).into_stream();
    tokio::pin!(stream);
    assert!(matches!(stream.next().await, Some(Err(Error::Config(_)))));
    assert!(stream.next().await.is_none());
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn campaign_activity_stream_rejects_unsupported_limit_without_http_and_then_terminates() {
    let server = MockServer::start().await;
    let sdk = client(&server);
    let campaigns = sdk.campaigns();
    let stream = campaigns.activity("1").limit(7).into_stream();
    tokio::pin!(stream);
    assert!(matches!(stream.next().await, Some(Err(Error::Config(_)))));
    assert!(stream.next().await.is_none());
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn cursor_stream_yields_an_api_failure_once_and_then_terminates() {
    let server = MockServer::start().await;
    support::resource_mock(
        &server,
        "GET",
        "/api/subscribers",
        404,
        Some(json!({"message":"Resource not found"})),
        None,
    )
    .await;
    let sdk = support::default_client(&server);
    let subscribers = sdk.subscribers();
    let stream = subscribers.list().into_stream();
    tokio::pin!(stream);
    let Error::Api(error) = stream.try_next().await.unwrap_err() else {
        panic!("expected API error")
    };
    assert_eq!(error.status().as_u16(), 404);
    assert_eq!(error.message(), Some("Resource not found"));
    assert!(stream.try_next().await.unwrap().is_none());
}
