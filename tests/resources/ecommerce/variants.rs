//! Independently runnable e-commerce contract scenarios.
use super::*;

#[tokio::test]
async fn list_decodes_attributes_and_numeric_price() {
    let server = MockServer::start().await;
    let client = client(&server);
    let variants = client.ecommerce().shop("1").products().variants("2");
    mount(
        &server,
        "GET",
        "/ecommerce/shops/1/products/2/variants",
        None,
        json!({"data":[{"id":"4","name":"Black","attributes":{"color":"Black"},"price":10}]}),
        200,
    )
    .await;
    let response = variants.list().await.unwrap();

    assert_eq!(response.data.len(), 1);
    let variant = &response.data[0];
    assert_eq!(variant.id, "4");
    assert_eq!(variant.name, "Black");
    assert_eq!(variant.attributes["color"], "Black");
    assert_eq!(variant.price, Some(Decimal::from(10)));
}

#[tokio::test]
async fn create_sends_variant_array_and_returns_product() {
    let server = MockServer::start().await;
    let client = client(&server);
    let variants = client.ecommerce().shop("1").products().variants("2");
    let product = json!({"id":"2","name":"Product","price":"12.34"});
    mount(
        &server,
        "POST",
        "/ecommerce/shops/1/products/2/variants",
        Some(json!([{"name":"Black","attributes":{}}])),
        json!({"data":product.clone()}),
        200,
    )
    .await;
    let response = variants
        .create(vec![SaveVariant::new("Black", BTreeMap::new())])
        .await
        .unwrap();

    assert_eq!(response.data.id.as_str(), "2");
    assert_eq!(response.data.name, "Product");
    assert_eq!(response.data.price, Some(Decimal::new(1234, 2)));
}

#[tokio::test]
async fn update_returns_parent_product() {
    let server = MockServer::start().await;
    let client = client(&server);
    let variants = client.ecommerce().shop("1").products().variants("2");
    let product = json!({"id":"2","name":"Product","price":"12.34"});
    mount(
        &server,
        "PUT",
        "/ecommerce/shops/1/products/2/variants/4",
        Some(json!({"name":"Black","attributes":{}})),
        json!({"data":product.clone()}),
        200,
    )
    .await;
    let response = variants
        .update("4", SaveVariant::new("Black", BTreeMap::new()))
        .await
        .unwrap();

    assert_eq!(response.data.id.as_str(), "2");
    assert_eq!(response.data.name, "Product");
    assert_eq!(response.data.price, Some(Decimal::new(1234, 2)));
}

#[tokio::test]
async fn delete_returns_parent_product() {
    let server = MockServer::start().await;
    let client = client(&server);
    let variants = client.ecommerce().shop("1").products().variants("2");
    let product = json!({"id":"2","name":"Product","price":"12.34"});
    mount(
        &server,
        "DELETE",
        "/ecommerce/shops/1/products/2/variants/4",
        None,
        json!({"data":product.clone()}),
        200,
    )
    .await;
    let response = variants.delete("4").await.unwrap();

    assert_eq!(response.data.id.as_str(), "2");
    assert_eq!(response.data.name, "Product");
    assert_eq!(response.data.price, Some(Decimal::new(1234, 2)));
}
