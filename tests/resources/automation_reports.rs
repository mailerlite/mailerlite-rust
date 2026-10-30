//! Automation report query and response contracts.
use crate::support;
use mailerlite_rs::{Error, resources::automation_actions::*};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string, header, method, path, query_param},
};

#[tokio::test]
async fn ecommerce_orders_rejects_zero_page_without_http() {
    let server = MockServer::start().await;
    let result = support::default_client(&server)
        .automations()
        .ecommerce_orders(
            "001",
            AutomationOrderQuery::new(AutomationOrderTimeFrame::AllTime).page(0),
        )
        .await;
    assert!(matches!(result, Err(Error::Config(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn ecommerce_orders_rejects_zero_limit_without_http() {
    let server = MockServer::start().await;
    let result = support::default_client(&server)
        .automations()
        .ecommerce_orders(
            "001",
            AutomationOrderQuery::new(AutomationOrderTimeFrame::AllTime).limit(0),
        )
        .await;
    assert!(matches!(result, Err(Error::Config(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn email_stats_rejects_an_end_date_without_a_start_date() {
    let server = MockServer::start().await;
    let result = support::default_client(&server)
        .automations()
        .email_stats("001", AutomationReportQuery::new().end_date("2026-01-31"))
        .await;
    assert!(matches!(result, Err(Error::Config(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn ecommerce_orders_custom_dates_accept_an_empty_terminal_page() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/automations/001/order-activity"))
        .and(query_param("time_frame", "custom"))
        .and(query_param("time_frame_start", "2026-01-01"))
        .and(query_param("time_frame_end", "2026-01-31"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":[],"meta":{"current_page":1,"last_page":1}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = support::default_client(&server)
        .automations()
        .ecommerce_orders(
            "001",
            AutomationOrderQuery::new(AutomationOrderTimeFrame::Custom)
                .time_frame_start("2026-01-01")
                .time_frame_end("2026-01-31"),
        )
        .await
        .unwrap();
    assert!(response.data.data.is_empty());
    assert_eq!(response.data.next_page(), None);
}

#[tokio::test]
async fn ecommerce_stats_preserves_decimal_strings_and_nullable_metrics() {
    let server = MockServer::start().await;
    support::resource_mock(&server, "GET", "/api/automations/001/monthly-totals/ecommerce", 200,
        Some(json!({"data":{"ecommerce":{"orders_count":0,"revenue":"0.00","average_order_value":"0.00","conversion_rate":null},"best_performing_email":null}})), None).await;
    let response = support::default_client(&server)
        .automations()
        .ecommerce_stats("001", AutomationReportQuery::new())
        .await
        .unwrap();
    assert_eq!(response.data.ecommerce.revenue.to_string(), "0.00");
    assert!(response.data.ecommerce.conversion_rate.is_none());
    assert!(response.data.best_performing_email.is_none());
}

#[tokio::test]
async fn ecommerce_stats_rejects_invalid_money_with_response_diagnostics() {
    let server = MockServer::start().await;
    support::resource_mock(&server, "GET", "/api/automations/001/monthly-totals/ecommerce", 200,
        Some(json!({"data":{"ecommerce":{"orders_count":1,"revenue":"invalid","average_order_value":"1.00"}}})), None).await;
    let Error::Decode(error) = support::default_client(&server)
        .automations()
        .ecommerce_stats("001", AutomationReportQuery::new())
        .await
        .unwrap_err()
    else {
        panic!("expected decoding error")
    };
    assert_eq!(error.status().as_u16(), 200);
    assert!(error.detail().contains("revenue"));
    assert!(String::from_utf8_lossy(error.body()).contains("invalid"));
}

#[tokio::test]
async fn email_stats_retains_rate_limit_timing_without_default_retry() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/automations/001/monthly-totals/email"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "7")
                .set_body_json(json!({"message":"Too many requests"})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let Error::Api(error) = support::default_client(&server)
        .automations()
        .email_stats("001", AutomationReportQuery::new())
        .await
        .unwrap_err()
    else {
        panic!("expected API error")
    };
    assert!(error.is_rate_limited());
    assert_eq!(
        error.metadata().retry_after,
        Some(std::time::Duration::from_secs(7))
    );
    assert_eq!(error.message(), Some("Too many requests"));
}

#[tokio::test]
async fn email_report_stats_sends_date_range_and_decodes_formatted_rate() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/automations/001/emails/002/report-stats"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(body_string(""))
        .and(query_param("start_date", "2026-01-01"))
        .and(query_param("end_date", "2026-01-31"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"engagement":{"sent":0,"opens_count":0,"clicks_count":0,"total_opens_count":0,"total_clicks_count":0,"forwards_count":0,"social_interactions_count":0,"open_rate":{"string":"0%"},"click_rate":{"string":"0%"},"click_to_open_rate":{"string":"0%"},"forward_rate":{"string":"0%"},"social_interaction_rate":{"string":"0%"}},"deliverability":{"sent":0,"delivered_count":0,"hard_bounces_count":0,"soft_bounces_count":0,"spam_count":0,"unsubscribes_count":0,"delivered_rate":{"string":"0%"},"hard_bounce_rate":{"string":"0%"},"soft_bounce_rate":{"string":"0%"},"spam_rate":{"string":"0%"},"unsubscribe_rate":{"string":"0%"}}}})))
        .expect(1).mount(&server).await;
    let response = support::client(&server)
        .automations()
        .email_report_stats(
            "001",
            "002",
            AutomationReportQuery::new()
                .start_date("2026-01-01")
                .end_date("2026-01-31"),
        )
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
    assert_eq!(response.data.engagement.open_rate.string, "0%");
}

#[tokio::test]
async fn email_unsubscribe_reasons_decodes_collection() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/automations/001/emails/002/unsubscribe-reasons"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(body_string(""))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"data":[{"label":"Too many emails","count":2,"percentage":100}]}),
        ))
        .expect(1)
        .mount(&server)
        .await;
    let response = support::client(&server)
        .automations()
        .email_unsubscribe_reasons("001", "002")
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
}

#[tokio::test]
async fn email_stats_sends_date_range_and_decodes_formatted_rate() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/automations/001/monthly-totals/email"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(body_string(""))
        .and(query_param("start_date", "2026-01-01"))
        .and(query_param("end_date", "2026-01-31"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"engagement":{"sent":0,"opens_count":0,"clicks_count":0,"total_opens_count":0,"total_clicks_count":0,"forwards_count":0,"social_interactions_count":0,"open_rate":{"string":"0%"},"click_rate":{"string":"0%"},"click_to_open_rate":{"string":"0%"},"forward_rate":{"string":"0%"},"social_interaction_rate":{"string":"0%"}},"deliverability":{"sent":0,"delivered_count":0,"hard_bounces_count":0,"soft_bounces_count":0,"spam_count":0,"unsubscribes_count":0,"delivered_rate":{"string":"0%"},"hard_bounce_rate":{"string":"0%"},"soft_bounce_rate":{"string":"0%"},"spam_rate":{"string":"0%"},"unsubscribe_rate":{"string":"0%"}}}})))
        .expect(1).mount(&server).await;
    let response = support::client(&server)
        .automations()
        .email_stats(
            "001",
            AutomationReportQuery::new()
                .start_date("2026-01-01")
                .end_date("2026-01-31"),
        )
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
    assert_eq!(response.data.engagement.open_rate.string, "0%");
}

#[tokio::test]
async fn ecommerce_stats_sends_shop_and_date_range() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/automations/001/monthly-totals/ecommerce"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(body_string(""))
        .and(query_param("start_date", "2026-01-01"))
        .and(query_param("end_date", "2026-01-31"))
        .and(query_param("shop_id", "005"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"ecommerce":{"orders_count":2,"revenue":39.98,"average_order_value":19.99,"conversion_rate":"20%"},"best_performing_email":null}})))
        .expect(1).mount(&server).await;
    let response = support::client(&server)
        .automations()
        .ecommerce_stats(
            "001",
            AutomationReportQuery::new()
                .start_date("2026-01-01")
                .end_date("2026-01-31")
                .shop_id("005"),
        )
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
    assert_eq!(response.data.ecommerce.revenue.to_string(), "39.98");
}

#[tokio::test]
async fn email_ecommerce_stats_sends_shop_filter() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/automations/001/emails/002/ecommerce-overview-stats"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(body_string(""))
        .and(query_param("shop_id", "005"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"ecommerce":{"orders_count":2,"revenue":39.98,"average_order_value":19.99,"conversion_rate":"20%"}}})))
        .expect(1).mount(&server).await;
    let response = support::client(&server)
        .automations()
        .email_ecommerce_stats("001", "002", AutomationReportQuery::new().shop_id("005"))
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
    assert_eq!(response.data.ecommerce.revenue.to_string(), "39.98");
}

#[tokio::test]
async fn ecommerce_orders_sends_page_filters_and_decodes_nullable_order() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/automations/001/order-activity"))
        .and(header("authorization", "Bearer private-test-token"))
        .and(body_string(""))
        .and(query_param("time_frame", "all_time"))
        .and(query_param("ecommerce_shop_id", "005"))
        .and(query_param("page", "2"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"subscriber":{"id":"009","email":"reader@example.test"},"total":19.99,"updated_at":"2026-01-01 12:00:00","order":null,"email":null}],"meta":{"current_page":2,"last_page":2,"per_page":10,"total":11}})))
        .expect(1).mount(&server).await;
    let response = support::client(&server)
        .automations()
        .ecommerce_orders(
            "001",
            AutomationOrderQuery::new(AutomationOrderTimeFrame::AllTime)
                .shop_id("005")
                .page(2)
                .limit(10),
        )
        .await
        .unwrap();
    assert_eq!(response.metadata.status.as_u16(), 200);
    assert!(response.data.data[0].order.is_none());
    assert_eq!(response.data.data[0].total.to_string(), "19.99");
}

#[tokio::test]
async fn email_stats_requires_both_date_bounds() {
    let server = MockServer::start().await;
    let sdk = support::default_client(&server);
    assert!(matches!(
        sdk.automations()
            .email_stats("1", AutomationReportQuery::new().start_date("2026-01-01"))
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn ecommerce_orders_requires_custom_date_bounds() {
    let server = MockServer::start().await;
    let sdk = support::default_client(&server);
    assert!(matches!(
        sdk.automations()
            .ecommerce_orders(
                "1",
                AutomationOrderQuery::new(AutomationOrderTimeFrame::Custom)
            )
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn ecommerce_stats_decodes_populated_best_performing_email() {
    let server = MockServer::start().await;
    support::resource_mock(&server, "GET", "/api/automations/001/monthly-totals/ecommerce", 200,
        Some(json!({"data":{
            "ecommerce":{"orders_count":3,"revenue":37.35,"average_order_value":12.45,"conversion_rate":"12.5%"},
            "best_performing_email":{"email_id":"900719925474099312345","orders_count":3,"revenue":37.35,"sent_count":24,"conversion_rate":0.125,"average_order_value":12.45,"screenshot_url":"https://example.test/screenshot.png","subject":"The best email","created_at":"2026-01-02 09:05:00"}
        }})), None).await;
    let response = support::default_client(&server)
        .automations()
        .ecommerce_stats("001", AutomationReportQuery::new())
        .await
        .unwrap();
    let best = response.data.best_performing_email.unwrap();
    assert_eq!(best.email_id, "900719925474099312345");
    assert_eq!(best.orders_count, 3);
    assert_eq!(best.sent_count, 24);
    assert_eq!(best.revenue.to_string(), "37.35");
    assert_eq!(best.average_order_value.to_string(), "12.45");
    assert_eq!(best.conversion_rate, 0.125);
    assert_eq!(
        best.screenshot_url.as_deref(),
        Some("https://example.test/screenshot.png")
    );
    assert_eq!(best.subject.as_deref(), Some("The best email"));
    assert_eq!(best.created_at.as_deref(), Some("2026-01-02 09:05:00"));
}

#[tokio::test]
async fn ecommerce_stats_preserves_nullable_email_metadata_and_zero_delivery_ratio() {
    let server = MockServer::start().await;
    support::resource_mock(&server, "GET", "/api/automations/001/monthly-totals/ecommerce", 200,
        Some(json!({"data":{
            "ecommerce":{"orders_count":1,"revenue":9.99,"average_order_value":9.99},
            "best_performing_email":{"email_id":"002","orders_count":1,"revenue":9.99,"sent_count":0,"conversion_rate":0,"average_order_value":9.99,"screenshot_url":null,"subject":null,"created_at":null}
        }})), None).await;
    let response = support::default_client(&server)
        .automations()
        .ecommerce_stats("001", AutomationReportQuery::new())
        .await
        .unwrap();
    let best = response.data.best_performing_email.unwrap();
    assert_eq!(best.email_id, "002");
    assert_eq!(best.sent_count, 0);
    assert_eq!(best.conversion_rate, 0.0);
    assert!(best.screenshot_url.is_none());
    assert!(best.subject.is_none());
    assert!(best.created_at.is_none());
    assert!(response.data.ecommerce.conversion_rate.is_none());
}

#[tokio::test]
async fn ecommerce_stats_rejects_malformed_nested_best_email_without_retrying() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/automations/001/monthly-totals/ecommerce"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{
            "ecommerce":{"orders_count":1,"revenue":9.99,"average_order_value":9.99},
            "best_performing_email":{"email_id":"002","orders_count":1,"revenue":"invalid-price","sent_count":2,"conversion_rate":0.5,"average_order_value":9.99}
        }}))).expect(1).mount(&server).await;
    let Error::Decode(error) = support::client(&server)
        .automations()
        .ecommerce_stats("001", AutomationReportQuery::new())
        .await
        .unwrap_err()
    else {
        panic!("expected nested decode error")
    };
    assert_eq!(error.status().as_u16(), 200);
    assert!(error.detail().contains("best_performing_email.revenue"));
    assert!(String::from_utf8_lossy(error.body()).contains("invalid-price"));
}
