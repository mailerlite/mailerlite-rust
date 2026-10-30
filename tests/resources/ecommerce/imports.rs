//! Independently runnable e-commerce contract scenarios.
use super::*;

#[tokio::test]
async fn categories_preserve_large_integer_identifiers() {
    let server = MockServer::start().await;
    let client = client(&server);
    let c = client.ecommerce().shop("1").categories();
    mount(
        &server,
        "POST",
        "/ecommerce/shops/1/categories/import",
        Some(json!([{"name":"Category"}])),
        json!({"data":[{"id":46919150796276752u64,"name":"Category"}]}),
        200,
    )
    .await;
    let category = c.import(vec![SaveCategory::new("Category")]).await.unwrap();
    assert_eq!(category.data[0].id.as_ref(), "46919150796276752");
}

#[tokio::test]
async fn products_send_categories_in_array_payload() {
    let server = MockServer::start().await;
    let client = client(&server);
    let p = client.ecommerce().shop("1").products();
    let product = json!({"id":"2","name":"Product","price":"12.34"});
    mount(
        &server,
        "POST",
        "/ecommerce/shops/1/products/import",
        Some(json!([{"name":"Product","categories":["3"]}])),
        json!({"data":[product]}),
        200,
    )
    .await;
    p.import(vec![
        ImportProduct::new(SaveProduct::new("Product")).categories(vec!["3".into()]),
    ])
    .await
    .unwrap();
}

#[tokio::test]
async fn customers_decode_accepted_input_records() {
    let server = MockServer::start().await;
    let client = client(&server);
    let shop = client.ecommerce().shop("1");
    mount(&server,"POST","/ecommerce/shops/1/customers/import",Some(json!([{"email":"a@example.com","orders_count":3}])),json!({"data":[{"email":"a@example.com","resource_id":"customer-1","create_subscriber":true}]}),200).await;
    let accepted = shop
        .customers()
        .import(vec![
            ImportCustomer::new(SaveCustomer::new("a@example.com")).orders_count(3),
        ])
        .await
        .unwrap();
    assert_eq!(accepted.data[0].email, "a@example.com");
}

#[tokio::test]
async fn orders_send_orders_wrapper_with_nested_cart_items() {
    let server = MockServer::start().await;
    let client = client(&server);
    let shop = client.ecommerce().shop("1");
    mount(
        &server,
        "POST",
        "/ecommerce/shops/1/orders/import",
        Some(json!({"orders":[{"customer":{"email":"a@example.com"},"cart":{"items":[{"ecommerce_product_id":"2"}]}}]})),
        json!({"data":[{"id":46920756608631854u64,"status":"pending"}]}),
        200,
    )
    .await;
    let response = shop
        .orders()
        .import(vec![CreateOrder::new(
            SaveCustomer::new("a@example.com"),
            OrderCart::new(vec![SaveCartItem::new("2")]),
        )])
        .await
        .unwrap();
    assert_eq!(response.data[0].id.as_ref(), "46920756608631854");
}

#[tokio::test]
async fn customers_preserve_account_limit_error_without_retrying() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/ecommerce/shops/1/customers/import"))
        .and(body_json(json!([{"email":"a@example.test","create_subscriber":true}])))
        .respond_with(ResponseTemplate::new(413).set_body_json(json!({"message":"The customers you are going to import might exceed the current subscriber limit."})))
        .expect(1).mount(&server).await;
    let mailerlite_rs::Error::Api(error) = crate::support::client(&server)
        .ecommerce()
        .shop("1")
        .customers()
        .import(vec![ImportCustomer::new(
            SaveCustomer::new("a@example.test").create_subscriber(true),
        )])
        .await
        .unwrap_err()
    else {
        panic!("expected account-limit API error")
    };
    assert_eq!(error.status().as_u16(), 413);
    assert!(error.message().unwrap().contains("subscriber limit"));
}

#[tokio::test]
async fn products_preserve_plan_limit_error_without_retrying() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/ecommerce/shops/1/products/import"))
        .and(body_json(json!([{"name":"Guide"}])))
        .respond_with(ResponseTemplate::new(413).set_body_json(json!({"message":"The products you are going to import might exceed the current product limit."})))
        .expect(1).mount(&server).await;
    let mailerlite_rs::Error::Api(error) = crate::support::client(&server)
        .ecommerce()
        .shop("1")
        .products()
        .import(vec![ImportProduct::new(SaveProduct::new("Guide"))])
        .await
        .unwrap_err()
    else {
        panic!("expected plan-limit API error")
    };
    assert_eq!(error.status().as_u16(), 413);
    assert!(error.message().unwrap().contains("product limit"));
}

#[tokio::test]
async fn orders_preserve_account_limit_error_without_retrying() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/ecommerce/shops/1/orders/import"))
        .and(body_json(json!({"orders":[{"customer":{"email":"a@example.test","create_subscriber":true},"cart":{"items":[{"ecommerce_product_id":"2"}]}}]})))
        .respond_with(ResponseTemplate::new(413).set_body_json(json!({"message":"The orders you are going to import might exceed the current subscriber limit."})))
        .expect(1).mount(&server).await;
    let mailerlite_rs::Error::Api(error) = crate::support::client(&server)
        .ecommerce()
        .shop("1")
        .orders()
        .import(vec![CreateOrder::new(
            SaveCustomer::new("a@example.test").create_subscriber(true),
            OrderCart::new(vec![SaveCartItem::new("2")]),
        )])
        .await
        .unwrap_err()
    else {
        panic!("expected account-limit API error")
    };
    assert_eq!(error.status().as_u16(), 413);
    assert!(error.message().unwrap().contains("subscriber limit"));
}

#[tokio::test]
async fn products_preserve_nested_variant_validation_errors() {
    let server = MockServer::start().await;
    mount(&server, "POST", "/ecommerce/shops/1/products/import",
        Some(json!([{"name":"Shirt","variants":[{"name":"Blue","attributes":{},"price_type":"fixed"}]}])),
        json!({"message":"The given data was invalid.","errors":{"0.variants.0.price":["The price field is required."]}}), 422).await;
    let mailerlite_rs::Error::Api(error) = client(&server)
        .ecommerce()
        .shop("1")
        .products()
        .import(vec![ImportProduct::new(
            SaveProduct::new("Shirt").variants(vec![
                SaveVariant::new("Blue", BTreeMap::new()).price_type(PriceType::Fixed),
            ]),
        )])
        .await
        .unwrap_err()
    else {
        panic!("expected validation error")
    };
    assert!(error.is_validation());
    assert_eq!(
        error.validation_errors()["0.variants.0.price"],
        ["The price field is required."]
    );
}

#[tokio::test]
async fn orders_preserve_wrapped_validation_paths_without_retrying() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/ecommerce/shops/1/orders/import"))
        .and(body_json(
            json!({"orders":[{"customer":{"email":"a@example.test"},"cart":{"items":[]}}]}),
        ))
        .respond_with(ResponseTemplate::new(422).set_body_json(json!({
            "message":"The given data was invalid.",
            "errors":{"orders.0.cart.items":["The orders.0.cart.items field is required."]}
        })))
        .expect(1)
        .mount(&server)
        .await;
    let mailerlite_rs::Error::Api(error) = crate::support::client(&server)
        .ecommerce()
        .shop("1")
        .orders()
        .import(vec![CreateOrder::new(
            SaveCustomer::new("a@example.test"),
            OrderCart::new(vec![]),
        )])
        .await
        .unwrap_err()
    else {
        panic!("expected wrapped order validation error")
    };
    assert!(error.is_validation());
    assert_eq!(
        error.validation_errors()["orders.0.cart.items"],
        ["The orders.0.cart.items field is required."]
    );
}

#[tokio::test]
async fn customers_return_only_accepted_records_when_account_capacity_is_partial() {
    let server = MockServer::start().await;
    mount(&server, "POST", "/ecommerce/shops/1/customers/import",
        Some(json!([{"email":"a@example.test"},{"email":"b@example.test"}])),
        json!({"data":[{"resource_id":"","email":"a@example.test","create_subscriber":false,"accepts_marketing":false}]}), 200).await;
    let response = client(&server)
        .ecommerce()
        .shop("1")
        .customers()
        .import(vec![
            ImportCustomer::new(SaveCustomer::new("a@example.test")),
            ImportCustomer::new(SaveCustomer::new("b@example.test")),
        ])
        .await
        .unwrap();
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].email, "a@example.test");
}
