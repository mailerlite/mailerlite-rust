//! Independently runnable e-commerce contract scenarios.
use super::*;

#[tokio::test]
async fn list_returns_numbered_page() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").orders();
    let fixture = json!({"id":"1","status":"pending"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/orders",
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
    let api = client.ecommerce().shop("1").orders();
    let fixture = json!({"id":"1","status":"pending"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/orders/1",
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
    let api = client.ecommerce().shop("1").orders();
    let fixture = json!({"id":"1","status":"pending"});
    mount(
        &server,
        "POST",
        "/ecommerce/shops/1/orders",
        Some(json!({"customer":{"email":"a@example.com"},"cart":{"items":[]}})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    api.create(CreateOrder::new(
        SaveCustomer::new("a@example.com"),
        OrderCart::new(vec![]),
    ))
    .await
    .unwrap();
}

#[tokio::test]
async fn update_accepts_created_status() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").orders();
    let fixture = json!({"id":"1","status":"pending"});
    mount(
        &server,
        "PUT",
        "/ecommerce/shops/1/orders/1",
        Some(json!({"status":"complete"})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    api.update("1", UpdateOrder::new().status(OrderStatus::Complete))
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_decodes_response_body() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").orders();
    let fixture = json!({"id":"1","status":"pending"});
    mount(
        &server,
        "DELETE",
        "/ecommerce/shops/1/orders/1",
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
    let api = client.ecommerce().shop("1").orders();
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/orders"))
        .and(query_param("limit", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":42})))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(api.count().await.unwrap().data.total, 42);
}

#[tokio::test]
async fn list_preserves_attribution_and_customer_filters() {
    let server = MockServer::start().await;
    let client = client(&server);
    let shop = client.ecommerce().shop("1");

    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/orders"))
        .and(query_param("filter[status]", "complete"))
        .and(query_param("filter[customer]", "3"))
        .and(query_param("filter[attributed]", "true"))
        .and(query_param("filter[email_id]", "4"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .expect(1)
        .mount(&server)
        .await;
    shop.orders()
        .list()
        .status(OrderStatus::Complete)
        .customer("3")
        .attributed(true)
        .email("4")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn unsupported_name_sort_fails_before_network() {
    let server = MockServer::start().await;
    let client = client(&server);
    assert!(matches!(
        client
            .ecommerce()
            .shop("1")
            .orders()
            .list()
            .sort(Sort::Name)
            .send()
            .await,
        Err(mailerlite_rs::Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}
