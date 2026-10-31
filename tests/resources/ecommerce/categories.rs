//! Independently runnable e-commerce contract scenarios.
use super::*;

#[tokio::test]
async fn list_returns_numbered_page() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").categories();
    let fixture = json!({"id":"1","name":"Category"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/categories",
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
    let api = client.ecommerce().shop("1").categories();
    let fixture = json!({"id":"1","name":"Category"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/categories/1",
        None,
        json!({"data":fixture.clone()}),
        200,
    )
    .await;
    let response = api.get("1").await.unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.name, "Category");
}

#[tokio::test]
async fn create_sends_documented_payload() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").categories();
    let fixture = json!({"id":"1","name":"Category"});
    mount(
        &server,
        "POST",
        "/ecommerce/shops/1/categories",
        Some(json!({"name":"Category"})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    let response = api.create(SaveCategory::new("Category")).await.unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.name, "Category");
}

#[tokio::test]
async fn update_accepts_created_status() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").categories();
    let fixture = json!({"id":"1","name":"Category"});
    mount(
        &server,
        "PUT",
        "/ecommerce/shops/1/categories/1",
        Some(json!({"name":"Category"})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    let response = api
        .update("1", SaveCategory::new("Category"))
        .await
        .unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.name, "Category");
}

#[tokio::test]
async fn delete_decodes_response_body() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").categories();
    let fixture = json!({"id":"1","name":"Category"});
    mount(
        &server,
        "DELETE",
        "/ecommerce/shops/1/categories/1",
        None,
        json!({"data":fixture.clone()}),
        200,
    )
    .await;
    let response = api.delete("1").await.unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.name, "Category");
}

#[tokio::test]
async fn count_uses_limit_zero() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").categories();
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/categories"))
        .and(query_param("limit", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":42})))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(api.count().await.unwrap().data.total, 42);
}

#[tokio::test]
async fn assign_multiple_categories_preserves_replace_flag() {
    let server = MockServer::start().await;
    let client = client(&server);
    let p = client.ecommerce().shop("1").products();
    let product = json!({"id":"2","name":"Product","price":"12.34"});
    mount(
        &server,
        "PUT",
        "/ecommerce/shops/1/products/2/categories/multiple",
        Some(json!({"categories":["3"],"replace":true})),
        json!({"data":product.clone()}),
        200,
    )
    .await;
    p.assign_categories("2", vec!["3".into()], true)
        .await
        .unwrap();
}

#[tokio::test]
async fn list_products_in_category() {
    let server = MockServer::start().await;
    let client = client(&server);
    let c = client.ecommerce().shop("1").categories();
    let product = json!({"id":"2","name":"Product","price":"12.34"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/categories/3/products",
        None,
        json!({"data":[product.clone()]}),
        200,
    )
    .await;
    c.products("3").send().await.unwrap();
}

#[tokio::test]
async fn assign_product_to_category() {
    let server = MockServer::start().await;
    let client = client(&server);
    let c = client.ecommerce().shop("1").categories();
    let product = json!({"id":"2","name":"Product","price":"12.34"});
    mount(
        &server,
        "PUT",
        "/ecommerce/shops/1/categories/3/products/2",
        None,
        json!({"data":product.clone()}),
        200,
    )
    .await;
    c.assign("3", "2").await.unwrap();
}

#[tokio::test]
async fn unassign_product_from_category() {
    let server = MockServer::start().await;
    let client = client(&server);
    let c = client.ecommerce().shop("1").categories();
    let product = json!({"id":"2","name":"Product","price":"12.34"});
    mount(
        &server,
        "DELETE",
        "/ecommerce/shops/1/categories/3/products/2",
        None,
        json!({"data":product.clone()}),
        200,
    )
    .await;
    c.unassign("3", "2").await.unwrap();
}
