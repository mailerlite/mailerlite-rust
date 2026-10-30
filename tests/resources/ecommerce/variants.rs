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
    variants.list().await.unwrap();
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
    variants
        .create(vec![SaveVariant::new("Black", BTreeMap::new())])
        .await
        .unwrap();
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
    variants
        .update("4", SaveVariant::new("Black", BTreeMap::new()))
        .await
        .unwrap();
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
    variants.delete("4").await.unwrap();
}
