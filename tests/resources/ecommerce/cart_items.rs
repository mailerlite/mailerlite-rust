//! Independently runnable e-commerce contract scenarios.
use super::*;

#[tokio::test]
async fn list_returns_numbered_page() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").carts().items("1");
    let fixture = json!({"id":"1"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/carts/1/items",
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
    let api = client.ecommerce().shop("1").carts().items("1");
    let fixture = json!({"id":"1"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/carts/1/items/1",
        None,
        json!({"data":fixture.clone()}),
        200,
    )
    .await;
    let response = api.get("1").await.unwrap();

    assert_eq!(response.data.id.as_str(), "1");
}

#[tokio::test]
async fn create_sends_documented_payload() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").carts().items("1");
    let fixture = json!({"id":"1","quantity":2,"price":"12.34"});
    mount(
        &server,
        "POST",
        "/ecommerce/shops/1/carts/1/items",
        Some(json!({"ecommerce_product_id":"3","quantity":2})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    let response = api
        .create(SaveCartItem::new("3").quantity(2))
        .await
        .unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.quantity, Some(2));
    assert_eq!(response.data.price, Some(Decimal::new(1234, 2)));
}

#[tokio::test]
async fn update_accepts_created_status() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").carts().items("1");
    let fixture = json!({"id":"1","quantity":2,"price":"12.34"});
    mount(
        &server,
        "PUT",
        "/ecommerce/shops/1/carts/1/items/1",
        Some(json!({"ecommerce_product_id":"3","quantity":2})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    let response = api
        .update("1", SaveCartItem::new("3").quantity(2))
        .await
        .unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.quantity, Some(2));
    assert_eq!(response.data.price, Some(Decimal::new(1234, 2)));
}

#[tokio::test]
async fn delete_decodes_response_body() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").carts().items("1");
    let fixture = json!({"id":"1","quantity":2,"price":"12.34"});
    mount(
        &server,
        "DELETE",
        "/ecommerce/shops/1/carts/1/items/1",
        None,
        json!({"data":fixture.clone()}),
        200,
    )
    .await;
    let response = api.delete("1").await.unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.quantity, Some(2));
    assert_eq!(response.data.price, Some(Decimal::new(1234, 2)));
}

#[tokio::test]
async fn count_uses_limit_zero() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").carts().items("1");
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/carts/1/items"))
        .and(query_param("limit", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":42})))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(api.count().await.unwrap().data.total, 42);
}
