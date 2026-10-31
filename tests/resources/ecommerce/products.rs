//! Independently runnable e-commerce contract scenarios.
use super::*;

#[tokio::test]
async fn list_returns_numbered_page() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").products();
    let fixture = json!({"id":"1","name":"Product","price":"12.34"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/products",
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
    let api = client.ecommerce().shop("1").products();
    let fixture = json!({"id":"1","name":"Product","price":"12.34"});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/products/1",
        None,
        json!({"data":fixture.clone()}),
        200,
    )
    .await;
    let response = api.get("1").await.unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.name, "Product");
    assert_eq!(response.data.price, Some(Decimal::new(1234, 2)));
}

#[tokio::test]
async fn create_sends_documented_payload() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").products();
    let fixture = json!({"id":"1","name":"Product","price":"12.34"});
    mount(
        &server,
        "POST",
        "/ecommerce/shops/1/products",
        Some(json!({"name":"Product"})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    let response = api.create(SaveProduct::new("Product")).await.unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.name, "Product");
    assert_eq!(response.data.price, Some(Decimal::new(1234, 2)));
}

#[tokio::test]
async fn update_accepts_created_status() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").products();
    let fixture = json!({"id":"1","name":"Product","price":"12.34"});
    mount(
        &server,
        "PUT",
        "/ecommerce/shops/1/products/1",
        Some(json!({"name":"Product"})),
        json!({"data":fixture.clone()}),
        201,
    )
    .await;
    let response = api.update("1", SaveProduct::new("Product")).await.unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.name, "Product");
    assert_eq!(response.data.price, Some(Decimal::new(1234, 2)));
}

#[tokio::test]
async fn delete_decodes_response_body() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").products();
    let fixture = json!({"id":"1","name":"Product","price":"12.34"});
    mount(
        &server,
        "DELETE",
        "/ecommerce/shops/1/products/1",
        None,
        json!({"data":fixture.clone()}),
        200,
    )
    .await;
    let response = api.delete("1").await.unwrap();

    assert_eq!(response.data.id.as_str(), "1");
    assert_eq!(response.data.name, "Product");
    assert_eq!(response.data.price, Some(Decimal::new(1234, 2)));
}

#[tokio::test]
async fn count_uses_limit_zero() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce().shop("1").products();
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/products"))
        .and(query_param("limit", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total":42})))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(api.count().await.unwrap().data.total, 42);
}

#[tokio::test]
async fn set_enabled_sends_boolean_status() {
    let server = MockServer::start().await;
    let client = client(&server);
    let p = client.ecommerce().shop("1").products();
    let product = json!({"id":"2","name":"Product","price":"12.34"});
    mount(
        &server,
        "PUT",
        "/ecommerce/shops/1/products/2/change-status",
        Some(json!({"status":false})),
        json!({"data":product.clone()}),
        200,
    )
    .await;
    p.set_enabled("2", false).await.unwrap();
}

#[tokio::test]
async fn get_encodes_external_identifier_and_sends_resource_id_flag() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/products/external%2Fproduct"))
        .and(query_param("with_resource_id", ""))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"data":{"id":"2","name":"Product"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    api.shop("1")
        .with_resource_ids()
        .products()
        .get("external/product")
        .await
        .unwrap();
}

#[tokio::test]
async fn list_preserves_product_filters_and_sort() {
    let server = MockServer::start().await;
    let client = client(&server);
    let shop = client.ecommerce().shop("1");
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/products"))
        .and(query_param("filter[name]", "hat"))
        .and(query_param("filter[enabled]", "true"))
        .and(query_param("filter[category]", "2"))
        .and(query_param("sort", "-name"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .expect(1)
        .mount(&server)
        .await;
    shop.products()
        .list()
        .name("hat")
        .enabled(true)
        .category("2")
        .sort(Sort::NameDescending)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn zero_list_limit_fails_before_network() {
    let server = MockServer::start().await;
    let client = client(&server);
    assert!(matches!(
        client
            .ecommerce()
            .shop("1")
            .products()
            .list()
            .limit(0)
            .send()
            .await,
        Err(mailerlite_rs::Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn stream_is_send_and_preserves_external_ids() {
    use futures_util::TryStreamExt;
    let server = MockServer::start().await;
    let client = client(&server);
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/products"))
        .and(query_param("with_resource_id", ""))
        .and(query_param("filter[enabled]", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"data":[{"id":"2","name":"Product"}],"meta":{"current_page":1,"last_page":1}}),
        ))
        .expect(1)
        .mount(&server)
        .await;
    let stream = client
        .ecommerce()
        .shop("1")
        .with_resource_ids()
        .products()
        .list()
        .enabled(true)
        .into_stream();
    fn assert_send<T: Send>(_: &T) {}
    assert_send(&stream);
    tokio::pin!(stream);
    assert_eq!(stream.try_next().await.unwrap().unwrap().id.as_ref(), "2");
    assert!(stream.try_next().await.unwrap().is_none());
}

#[tokio::test]
async fn get_not_found_retains_api_message() {
    let server = MockServer::start().await;
    let client = client(&server);
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/products/missing",
        None,
        json!({"message":"Product not found"}),
        404,
    )
    .await;
    let error = client
        .ecommerce()
        .shop("1")
        .products()
        .get("missing")
        .await
        .unwrap_err();
    let mailerlite_rs::Error::Api(error) = error else {
        panic!("expected API error")
    };
    assert_eq!(error.status(), 404);
    assert_eq!(error.message(), Some("Product not found"));
}

#[tokio::test]
async fn create_validation_error_retains_field_messages() {
    let server = MockServer::start().await;
    let client = client(&server);
    mount(&server, "POST", "/ecommerce/shops/1/products", Some(json!({"name":""})),
        json!({"message":"The given data was invalid.","errors":{"name":["The name field is required."]}}), 422).await;
    let error = client
        .ecommerce()
        .shop("1")
        .products()
        .create(SaveProduct::new(""))
        .await
        .unwrap_err();
    let mailerlite_rs::Error::Api(error) = error else {
        panic!("expected API error")
    };
    assert_eq!(error.status(), 422);
    assert_eq!(
        error.validation_errors()["name"],
        ["The name field is required."]
    );
}

#[tokio::test]
async fn create_never_retries_even_when_safe_read_retries_are_enabled() {
    let server = MockServer::start().await;
    let client = crate::support::client(&server);
    Mock::given(method("POST"))
        .and(path("/api/ecommerce/shops/1/products"))
        .and(body_json(json!({"name":"Product"})))
        .respond_with(ResponseTemplate::new(503).set_body_json(json!({"message":"Unavailable"})))
        .expect(1)
        .mount(&server)
        .await;
    let error = client
        .ecommerce()
        .shop("1")
        .products()
        .create(SaveProduct::new("Product"))
        .await
        .unwrap_err();
    let mailerlite_rs::Error::Api(error) = error else {
        panic!("expected API error")
    };
    assert_eq!(error.status(), 503);
}

#[tokio::test]
async fn delete_empty_success_is_a_decode_error() {
    let server = MockServer::start().await;
    let client = client(&server);
    Mock::given(method("DELETE"))
        .and(path("/api/ecommerce/shops/1/products/1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let error = client
        .ecommerce()
        .shop("1")
        .products()
        .delete("1")
        .await
        .unwrap_err();
    let mailerlite_rs::Error::Decode(error) = error else {
        panic!("expected decode error")
    };
    assert_eq!(error.status(), 204);
    assert!(error.body().is_empty());
}

#[tokio::test]
async fn malformed_price_retains_response_diagnostics() {
    let server = MockServer::start().await;
    let client = client(&server);
    let body = json!({"data":{"id":"1","name":"Product","price":"invalid-price"}});
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/products/1",
        None,
        body.clone(),
        200,
    )
    .await;
    let error = client
        .ecommerce()
        .shop("1")
        .products()
        .get("1")
        .await
        .unwrap_err();
    let mailerlite_rs::Error::Decode(error) = error else {
        panic!("expected decode error")
    };
    assert_eq!(error.status(), 200);
    assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), body);
    assert!(!error.detail().is_empty());
}

#[tokio::test]
async fn null_price_is_accepted_as_absent() {
    let server = MockServer::start().await;
    let client = client(&server);
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/products/1",
        None,
        json!({"data":{"id":"1","name":"Product","price":null}}),
        200,
    )
    .await;
    let product = client
        .ecommerce()
        .shop("1")
        .products()
        .get("1")
        .await
        .unwrap();
    assert!(product.data.price.is_none());
}

#[tokio::test]
async fn zero_page_fails_before_network() {
    let server = MockServer::start().await;
    let client = client(&server);
    assert!(matches!(
        client
            .ecommerce()
            .shop("1")
            .products()
            .list()
            .page(0)
            .send()
            .await,
        Err(mailerlite_rs::Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn repeated_list_setters_replace_values_without_duplicate_query_keys() {
    let server = MockServer::start().await;
    let client = client(&server);
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/products"))
        .and(query_param("page", "2"))
        .and(query_param("limit", "5"))
        .and(query_param("filter[name]", "updated"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data":[],"meta":{"current_page":2,"last_page":2,"per_page":5,"total":5}
        })))
        .expect(1)
        .mount(&server)
        .await;
    client
        .ecommerce()
        .shop("1")
        .products()
        .list()
        .page(1)
        .page(2)
        .limit(10)
        .limit(5)
        .name("old")
        .name("updated")
        .send()
        .await
        .unwrap();
    let requests = server.received_requests().await.unwrap();
    let query: BTreeMap<_, _> = requests[0].url.query_pairs().collect();
    assert_eq!(requests[0].url.query_pairs().count(), 3);
    assert_eq!(query["page"], "2");
    assert_eq!(query["limit"], "5");
    assert_eq!(query["filter[name]"], "updated");
}

#[tokio::test]
async fn empty_list_retains_page_metadata() {
    let server = MockServer::start().await;
    let client = client(&server);
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/products",
        None,
        json!({"data":[],"meta":{"current_page":1,"last_page":1,"per_page":5,"total":0}}),
        200,
    )
    .await;
    let response = client
        .ecommerce()
        .shop("1")
        .products()
        .list()
        .send()
        .await
        .unwrap();
    assert!(response.data.data.is_empty());
    assert_eq!(response.data.meta.current_page, Some(1));
    assert_eq!(response.data.meta.last_page, Some(1));
    assert_eq!(response.data.meta.total, Some(0));
    assert_eq!(response.data.meta.per_page, Some(5));
}
