//! Independently runnable e-commerce contract scenarios.
use super::*;

#[tokio::test]
async fn shop_stats_preserve_required_date_range() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();
    let stats = json!({"ecommerce":{"orders_count":2,"revenue":40,"average_order_value":20,"conversion_rate":"2%"}});

    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/reports/stats"))
        .and(query_param("start_date", "2026-01-01"))
        .and(query_param("end_date", "2026-02-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":stats.clone()})))
        .expect(1)
        .mount(&server)
        .await;
    let response = api
        .shop("1")
        .stats("2026-01-01", "2026-02-01")
        .await
        .unwrap();

    assert_eq!(response.data.ecommerce.orders_count, 2);
    assert_eq!(response.data.ecommerce.revenue, Decimal::from(40));
    assert_eq!(
        response.data.ecommerce.average_order_value,
        Decimal::from(20)
    );
    assert_eq!(
        response.data.ecommerce.conversion_rate.as_deref(),
        Some("2%")
    );
}

#[tokio::test]
async fn campaign_stats_preserve_shop_filter() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();
    let stats = json!({"ecommerce":{"orders_count":2,"revenue":40,"average_order_value":20,"conversion_rate":"2%"}});

    Mock::given(method("GET"))
        .and(path("/api/campaigns/2/reports/ecommerce"))
        .and(query_param("shop_id", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":stats})))
        .expect(1)
        .mount(&server)
        .await;
    let response = api.campaign_stats("2", "1").await.unwrap();

    assert_eq!(response.data.ecommerce.orders_count, 2);
    assert_eq!(response.data.ecommerce.revenue, Decimal::from(40));
    assert_eq!(
        response.data.ecommerce.average_order_value,
        Decimal::from(20)
    );
    assert_eq!(
        response.data.ecommerce.conversion_rate.as_deref(),
        Some("2%")
    );
}

#[tokio::test]
async fn campaign_orders_preserve_shop_and_page() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();

    Mock::given(method("GET"))
        .and(path("/api/campaigns/2/reports/ecommerce/orders"))
        .and(query_param("shop_id", "1"))
        .and(query_param("page", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"subscriber":{"id":"3","email":"a@example.com"},"total":"12.34","updated_at":"2026-01-01 00:00:00","order":"1 X Product"}]})))
        .expect(1).mount(&server).await;
    let orders = api.campaign_orders("2", "1").page(2).send().await.unwrap();
    assert_eq!(orders.data.data[0].total, Decimal::new(1234, 2));
}

#[tokio::test]
async fn subscriber_stats_preserve_shop_filter() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();
    let stats = json!({"ecommerce":{"orders_count":2,"revenue":40,"average_order_value":20,"conversion_rate":"2%"}});

    Mock::given(method("GET"))
        .and(path("/api/subscribers/2/reports/ecommerce"))
        .and(query_param("shop_id", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":stats})))
        .expect(1)
        .mount(&server)
        .await;
    let response = api.subscriber_stats("2", "1").await.unwrap();

    assert_eq!(response.data.ecommerce.orders_count, 2);
    assert_eq!(response.data.ecommerce.revenue, Decimal::from(40));
    assert_eq!(
        response.data.ecommerce.average_order_value,
        Decimal::from(20)
    );
    assert_eq!(
        response.data.ecommerce.conversion_rate.as_deref(),
        Some("2%")
    );
}

#[tokio::test]
async fn subscriber_orders_preserve_shop_and_page() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();

    Mock::given(method("GET"))
        .and(path("/api/subscribers/2/reports/ecommerce/orders"))
        .and(query_param("shop_id", "1"))
        .and(query_param("page", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"subscriber":{"id":"3","email":"a@example.com"},"total":"12.34","updated_at":"2026-01-01 00:00:00","order":"1 X Product"}]})))
        .expect(1).mount(&server).await;
    let orders = api
        .subscriber_orders("2", "1")
        .page(2)
        .send()
        .await
        .unwrap();
    assert_eq!(orders.data.data[0].total, Decimal::new(1234, 2));
}

#[tokio::test]
async fn shop_stats_accept_missing_conversion_rate() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/ecommerce/shops/1/reports/stats"))
        .and(query_param("start_date", "2026-01-01"))
        .and(query_param("end_date", "2026-02-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "ecommerce": {"orders_count": 2, "revenue": "40.50", "average_order_value": "20.25"},
                "series": []
            }
        })))
        .expect(1)
        .mount(&server)
        .await;
    let response = client(&server)
        .ecommerce()
        .shop("1")
        .stats("2026-01-01", "2026-02-01")
        .await
        .unwrap();

    assert!(response.data.ecommerce.conversion_rate.is_none());
    assert_eq!(response.data.ecommerce.orders_count, 2);
    assert_eq!(response.data.ecommerce.revenue, Decimal::new(4050, 2));
    assert_eq!(
        response.data.ecommerce.average_order_value,
        Decimal::new(2025, 2)
    );
    assert!(response.data.series.unwrap().is_empty());
}

#[tokio::test]
async fn subscriber_stats_accept_missing_conversion_rate() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/subscribers/2/reports/ecommerce"))
        .and(query_param("shop_id", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {"ecommerce": {"orders_count": 0, "revenue": 0, "average_order_value": 0}}
        })))
        .expect(1)
        .mount(&server)
        .await;
    let response = client(&server)
        .ecommerce()
        .subscriber_stats("2", "1")
        .await
        .unwrap();

    assert!(response.data.ecommerce.conversion_rate.is_none());
    assert_eq!(response.data.ecommerce.orders_count, 0);
    assert_eq!(response.data.ecommerce.revenue, Decimal::ZERO);
}
