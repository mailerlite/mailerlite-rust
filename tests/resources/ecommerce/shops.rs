//! Independently runnable e-commerce contract scenarios.
use super::*;

#[tokio::test]
async fn list_returns_numbered_page() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();
    let fixture = json!({"id":"1","name":"Shop"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops",
        None,
        json!({"data":[fixture.clone()],"meta":{"current_page":1,"last_page":1}}),
        200,
    )
    .await;
    assert_eq!(api.list().send().await.unwrap().data.data.len(), 1);
}

#[tokio::test]
async fn get_decodes_resource() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();
    let fixture = json!({"id":"1","name":"Shop"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1",
        None,
        json!({"data":fixture.clone()}),
        200,
    )
    .await;
    api.get("1").await.unwrap();
}

#[tokio::test]
async fn create_sends_documented_payload() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();
    let fixture = json!({"id":"1","name":"Shop"});
    mount(
        &server,
        "POST",
        "/ecommerce/shops",
        Some(json!({"name":"Shop","url":"https://example.com","currency":"USD"})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    api.create(SaveShop::new("Shop", "https://example.com", "USD"))
        .await
        .unwrap();
}

#[tokio::test]
async fn update_accepts_created_status() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();
    let fixture = json!({"id":"1","name":"Shop"});
    mount(
        &server,
        "PUT",
        "/ecommerce/shops/1",
        Some(json!({"name":"Shop","url":"https://example.com","currency":"USD"})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    api.update("1", SaveShop::new("Shop", "https://example.com", "USD"))
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_decodes_response_body() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();
    let fixture = json!({"id":"1","name":"Shop"});
    mount(
        &server,
        "DELETE",
        "/ecommerce/shops/1",
        None,
        json!({"data":fixture.clone()}),
        200,
    )
    .await;
    api.delete("1").await.unwrap();
}

#[tokio::test]
async fn count_uses_limit_zero() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops"))
        .and(query_param("limit", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":42})))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(api.count().await.unwrap().data.total, 42);
}

#[tokio::test]
async fn unsupported_sort_fails_before_network() {
    let server = MockServer::start().await;
    let client = client(&server);
    assert!(matches!(
        client.ecommerce().list().sort(Sort::CreatedAt).send().await,
        Err(mailerlite_rs::Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn list_never_retries_internal_shop_initialization() {
    let server = MockServer::start().await;
    let client = crate::support::client(&server);
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops"))
        .respond_with(ResponseTemplate::new(503).set_body_json(json!({"message":"unavailable"})))
        .expect(1)
        .mount(&server)
        .await;
    assert!(client.ecommerce().list().send().await.is_err());
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn count_never_retries_internal_shop_initialization() {
    let server = MockServer::start().await;
    let client = crate::support::client(&server);
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops"))
        .respond_with(ResponseTemplate::new(503).set_body_json(json!({"message":"unavailable"})))
        .expect(1)
        .mount(&server)
        .await;
    assert!(client.ecommerce().count().await.is_err());
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn lazy_shop_list_never_retries_initialization() {
    use futures_util::TryStreamExt;
    let server = MockServer::start().await;
    let client = crate::support::client(&server);
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops"))
        .respond_with(ResponseTemplate::new(503).set_body_json(json!({"message":"unavailable"})))
        .expect(1)
        .mount(&server)
        .await;
    let stream = client.ecommerce().list().into_stream();
    tokio::pin!(stream);
    assert!(server.received_requests().await.unwrap().is_empty());
    assert!(stream.try_next().await.is_err());
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}
