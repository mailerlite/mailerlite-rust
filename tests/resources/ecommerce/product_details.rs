//! Backend-verified internal-product schemas and their JSON write parameters.
use super::*;
use mailerlite_rs::{Error, models::ecommerce::ProductAssetType};

#[tokio::test]
async fn get_decodes_assets_booking_settings_and_exact_order_totals() {
    let server = MockServer::start().await;
    mount(&server, "GET", "/ecommerce/shops/1/products/2", None, json!({"data":{
        "id":"2","name":"Workshop",
        "assets":[{"id":"0007","name":"Join","type":"url","url":"https://example.test/join","variant_id":9007199254740993u64}],
        "settings":{"action":"move_to_group","group_id":"0008","checkout_type":"stripe","enable_tax_id":false,"enable_terms":true,
            "free":false,"duration":"0.25","duration_unit":"hours","timezone_id":3,
            "timezone":{"id":3,"name":"UTC","offset":0,"offset_name":"UTC"},
            "calendar_integration_type":"google","booking_type":"group","booking_window_days":"30",
            "booking_group_max_participants":1000,"booking_group_display_remaining_slots":true,
            "custom_availability_days":{"monday":["09:00","17:00"],"sunday":null},
            "booking_event_id":"event-1","booking_attendees_count":5},
        "order_stats":{"total_orders":"9007199254740993","total_amount":"123456789.123456789123456789","currency":"€"}
    }}), 200).await;
    let product = client(&server)
        .ecommerce()
        .shop("1")
        .products()
        .get("2")
        .await
        .unwrap()
        .data;
    let asset = &product.assets.unwrap()[0];
    assert_eq!(asset.id, "0007");
    assert_eq!(asset.kind, ProductAssetType::Url);
    assert_eq!(asset.variant_id.as_deref(), Some("9007199254740993"));
    let settings = product.settings.unwrap();
    assert_eq!(settings.group_id.as_deref(), Some("0008"));
    assert_eq!(settings.duration, Some(Decimal::new(25, 2)));
    assert_eq!(settings.timezone_id.as_deref(), Some("3"));
    assert_eq!(settings.timezone.unwrap().name, "UTC");
    assert_eq!(settings.booking_window_days, Some(30));
    assert_eq!(settings.booking_group_max_participants, Some(1000));
    assert_eq!(settings.enable_tax_id, Some(false));
    assert_eq!(settings.booking_attendees_count, Some(5));
    let days = settings.custom_availability_days.unwrap();
    assert_eq!(days["monday"], Some(["09:00".into(), "17:00".into()]));
    assert!(days["sunday"].is_none());
    let stats = product.order_stats.unwrap();
    assert_eq!(stats.total_orders, 9007199254740993);
    assert_eq!(
        stats.total_amount.to_string(),
        "123456789.123456789123456789"
    );
    assert_eq!(stats.currency, "€");
}

#[tokio::test]
async fn get_rejects_fractional_order_count_and_retains_response_diagnostics() {
    let server = MockServer::start().await;
    mount(&server, "GET", "/ecommerce/shops/1/products/2", None, json!({"data":{
        "id":"2","name":"Invalid","order_stats":{"total_orders":"1.5","total_amount":"1.00","currency":"$"}
    }}), 200).await;
    let Error::Decode(error) = client(&server)
        .ecommerce()
        .shop("1")
        .products()
        .get("2")
        .await
        .unwrap_err()
    else {
        panic!("expected decode error")
    };
    assert_eq!(error.status().as_u16(), 200);
    assert!(error.detail().contains("total_orders"));
    assert!(!error.body().is_empty());
}

#[tokio::test]
async fn create_internal_product_sends_typed_url_assets_and_checkout_settings() {
    let server = MockServer::start().await;
    mount(&server, "POST", "/ecommerce/shops/1/products", Some(json!({
        "name":"Guide","price":10,"currency":"USD","settings":{"action":"move_to_group","group_id":"3","checkout_type":"stripe","enable_tax_id":false,"enable_terms":true},
        "assets":[{"name":"Download","type":"url","url":"https://example.test/guide"}]
    })), json!({"data":{"id":"2","name":"Guide"}}), 201).await;
    client(&server)
        .ecommerce()
        .shop("1")
        .products()
        .create(
            SaveProduct::mailerlite(
                "Guide",
                Decimal::TEN,
                "USD",
                ProductSettings::new(ProductAction::MoveToGroup)
                    .group_id("3")
                    .checkout_type("stripe")
                    .enable_tax_id(false)
                    .enable_terms(true),
            )
            .assets(vec![SaveProductAsset::url(
                "Download",
                "https://example.test/guide",
            )]),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn create_external_product_sends_categories_and_variants() {
    let server = MockServer::start().await;
    mount(&server, "POST", "/ecommerce/shops/1/products", Some(json!({
        "name":"Shirt","categories":["3"],"variants":[{"name":"Blue","attributes":{"color":"blue"},"price_type":"increment","price":2.5}]
    })), json!({"data":{"id":"2","name":"Shirt"}}), 201).await;
    client(&server)
        .ecommerce()
        .shop("1")
        .products()
        .create(
            SaveProduct::new("Shirt")
                .categories(vec!["3".into()])
                .variants(vec![
                    SaveVariant::new("Blue", BTreeMap::from([("color".into(), "blue".into())]))
                        .price_type(PriceType::Increment)
                        .price(Decimal::new(25, 1)),
                ]),
        )
        .await
        .unwrap();
}
