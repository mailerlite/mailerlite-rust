//! HTTP contract regression tests for the migrated simple-editor API.
#![allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]

use crate::editor_support::{expect_request, fixture, mailerlite};
use mailerlite_rs::{Response, models::brand_styles::BrandStyles};
use mockito::{Mock, Server, ServerGuard};

#[tokio::test]
async fn get_gets_the_brand_styles() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "GET",
        "/brand-styles",
        None,
        200,
        &fixture("brand_styles"),
    )
    .await;

    let response: Response<BrandStyles> = mailerlite(&server).brand_styles().get().await.unwrap();

    mock.assert_async().await;
    let styles: &BrandStyles = &response.data;
    let brand = styles.brand.as_ref().unwrap();
    assert_eq!(brand.identifier, "account");
    assert_eq!(brand.name, "Brand styles");
    assert_eq!(brand.primary_color, "#2cb191");
    assert_eq!(styles.default.identifier, "default-brand");
    assert_eq!(styles.default.background_color, "#ffffff");
    assert_eq!(styles.presets.len(), 2);
    assert_eq!(styles.presets[1].identifier, "preset-retro");
    assert_eq!(styles.presets[1].font_family, "Courier New, monospace");
    assert_eq!(styles.presets[1].heading_color, "#451a03");
}

#[tokio::test]
async fn get_decodes_a_null_brand() {
    let mut server: ServerGuard = Server::new_async().await;
    let mut body: serde_json::Value = serde_json::from_str(&fixture("brand_styles")).unwrap();
    body["data"]["brand"] = serde_json::Value::Null;
    expect_request(
        &mut server,
        "GET",
        "/brand-styles",
        None,
        200,
        &body.to_string(),
    )
    .await;

    let response: Response<BrandStyles> = mailerlite(&server).brand_styles().get().await.unwrap();

    assert!(response.data.brand.is_none());
    assert_eq!(response.data.default.identifier, "default-brand");
}
