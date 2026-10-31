//! Independently runnable e-commerce contract scenarios.
use super::*;

#[tokio::test]
async fn list_returns_numbered_page() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").carts();
    let fixture = json!({"id":"1"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/carts",
        None,
        json!({"data":[fixture.clone()],"meta":{"current_page":1,"last_page":1}}),
        200,
    )
    .await;
    assert_eq!(api.list().send().await.unwrap().data.data.len(), 1);
}

#[tokio::test]
async fn get_decodes_cart() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").carts();
    let fixture = json!({"id":"1"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/carts/1",
        None,
        json!({"data":fixture.clone()}),
        200,
    )
    .await;
    let response = api.get("1").await.unwrap();

    assert_eq!(response.data.id.as_str(), "1");
}

#[tokio::test]
async fn update_preserves_checkout_url() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").carts();
    let fixture = json!({"id":"1","checkout_url":"https://checkout.example"});
    mount(
        &server,
        "PUT",
        "/ecommerce/shops/1/carts/1",
        Some(json!({"checkout_url":"https://checkout.example"})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    let response = api
        .update(
            "1",
            UpdateCart::new().checkout_url("https://checkout.example"),
        )
        .await
        .unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(
        response.data.checkout_url.as_deref(),
        Some("https://checkout.example")
    );
}

#[tokio::test]
async fn count_uses_limit_zero() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").carts();
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/carts"))
        .and(query_param("limit", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":42})))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(api.count().await.unwrap().data.total, 42);
}
