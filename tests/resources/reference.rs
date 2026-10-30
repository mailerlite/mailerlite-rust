//! Timezone and language reference contracts.
use crate::support::{default_client, resource_mock};
use mailerlite_rs::Error;
use serde_json::json;
use wiremock::MockServer;

#[tokio::test]
async fn timezones_decode_offsets_in_minutes() {
    let server = MockServer::start().await;
    resource_mock(&server, "GET", "/api/timezones", 200,
        Some(json!({"data":[{"id":"370","name":"Europe/Vilnius","name_for_humans":"Europe/Vilnius (+03:00)","offset_name":"+03:00","offset":180}]})), None).await;
    assert_eq!(
        default_client(&server)
            .reference()
            .timezones()
            .await
            .unwrap()
            .data[0]
            .offset,
        180
    );
}

#[tokio::test]
async fn campaign_languages_preserve_string_identifiers() {
    let server = MockServer::start().await;
    resource_mock(&server, "GET", "/api/campaigns/languages", 200,
        Some(json!({"data":[{"id":"4","shortcode":"en","iso639":"en-US","name":"English","direction":"ltr"}]})), None).await;
    assert_eq!(
        default_client(&server)
            .reference()
            .campaign_languages()
            .await
            .unwrap()
            .data[0]
            .id,
        "4"
    );
}

#[tokio::test]
async fn timezones_accept_negative_fractional_hour_offsets() {
    let server = MockServer::start().await;
    resource_mock(&server, "GET", "/api/timezones", 200,
        Some(json!({"data":[{"id":"1","name":"America/St_Johns","name_for_humans":"St Johns","offset_name":"-03:30","offset":-210}]})), None).await;
    assert_eq!(
        default_client(&server)
            .reference()
            .timezones()
            .await
            .unwrap()
            .data[0]
            .offset,
        -210
    );
}

#[tokio::test]
async fn campaign_languages_accept_an_empty_collection() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/campaigns/languages",
        200,
        Some(json!({"data":[]})),
        None,
    )
    .await;
    assert!(
        default_client(&server)
            .reference()
            .campaign_languages()
            .await
            .unwrap()
            .data
            .is_empty()
    );
}

#[tokio::test]
async fn timezones_retain_diagnostics_for_a_non_integer_offset() {
    let server = MockServer::start().await;
    resource_mock(&server, "GET", "/api/timezones", 200,
        Some(json!({"data":[{"id":"1","name":"UTC","name_for_humans":"UTC","offset_name":"+00:00","offset":"invalid"}]})), None).await;
    let Error::Decode(error) = default_client(&server)
        .reference()
        .timezones()
        .await
        .unwrap_err()
    else {
        panic!("expected decoding error")
    };
    assert_eq!(error.status().as_u16(), 200);
    assert!(error.detail().contains("offset"));
}
