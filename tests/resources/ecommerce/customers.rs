//! Independently runnable e-commerce contract scenarios.
use super::*;

#[tokio::test]
async fn list_returns_numbered_page() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").customers();
    let fixture = json!({"id":"1","email":"a@example.com"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/customers",
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
    let api = client.ecommerce().shop("1").customers();
    let fixture = json!({"id":"1","email":"a@example.com"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/customers/1",
        None,
        json!({"data":fixture.clone()}),
        200,
    )
    .await;
    let response = api.get("1").await.unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.email, "a@example.com");
}

#[tokio::test]
async fn create_sends_documented_payload() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").customers();
    let fixture = json!({"id":"1","email":"a@example.com"});
    mount(
        &server,
        "POST",
        "/ecommerce/shops/1/customers",
        Some(json!({"email":"a@example.com"})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    api.create(SaveCustomer::new("a@example.com"))
        .await
        .unwrap();
}

#[tokio::test]
async fn update_accepts_created_status() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").customers();
    let fixture = json!({"id":"1","email":"a@example.com"});
    mount(
        &server,
        "PUT",
        "/ecommerce/shops/1/customers/1",
        Some(json!({"email":"a@example.com"})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    api.update("1", SaveCustomer::new("a@example.com"))
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_decodes_response_body() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").customers();
    let fixture = json!({"id":"1","email":"a@example.com"});
    mount(
        &server,
        "DELETE",
        "/ecommerce/shops/1/customers/1",
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
    let api = client.ecommerce().shop("1").customers();
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/customers"))
        .and(query_param("limit", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":42})))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(api.count().await.unwrap().data.total, 42);
}
