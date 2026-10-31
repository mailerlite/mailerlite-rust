//! Independently runnable e-commerce contract scenarios.
use super::*;

#[tokio::test]
async fn get_decodes_tracking_settings() {
    let server = MockServer::start().await;
    let client = client(&server);
    let api = client.ecommerce();
    mount(&server,"GET","/ecommerce/settings",None,json!({"data":{"ecommerce_tracking_in_campaigns":true,"ecommerce_tracking_in_automations":false,"ecommerce_attribution_window":30}}),200).await;
    let response = api.settings().await.unwrap();

    assert!(response.data.ecommerce_tracking_in_campaigns);
    assert!(!response.data.ecommerce_tracking_in_automations);
    assert_eq!(response.data.ecommerce_attribution_window, 30);
}
