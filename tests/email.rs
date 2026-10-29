mod common;

use common::{assert_no_request, expect_no_request, expect_request, fixture, mailerlite, BAD_IDS};
use mailerlite_rs::{
    api_response::{ApiResponse, WithEtag},
    email::{BlockType, EmailDocument, EmailHtml},
    error::Error,
};
use mockito::{Mock, Server, ServerGuard};
use serde_json::json;

#[tokio::test]
async fn find_gets_the_email() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "GET",
        "/emails/123",
        None,
        200,
        &fixture("email"),
    )
    .await;

    let response: ApiResponse<WithEtag<EmailDocument>> =
        mailerlite(&server).email().find("123").await.unwrap();

    mock.assert_async().await;
    assert_eq!(response.status_code, 200);
    assert_eq!(response.etag, "42");

    let email: &EmailDocument = &response.data;
    assert_eq!(email.id, "123");
    assert_eq!(email.email_type, "builder_inline_email");
    assert_eq!(email.emailable_type, "campaigns");
    assert_eq!(email.emailable_id, "456");

    let brand = email.settings.brand.as_ref().unwrap();
    assert_eq!(brand.mode.as_deref(), Some("account"));
    assert_eq!(brand.background_color.as_deref(), Some("#ffffff"));
    let layout = email.settings.layout.as_ref().unwrap();
    assert_eq!(layout.width_mode.as_deref(), Some("default"));
    let typography = email.settings.typography.as_ref().unwrap();
    assert_eq!(
        typography.link.as_ref().unwrap().color.as_deref(),
        Some("#2cb191")
    );
    let heading = typography.heading.as_ref().unwrap();
    assert_eq!(heading.font_weight.as_deref(), Some("700"));
    assert_eq!(heading.h1.as_ref().unwrap().size.as_deref(), Some("32"));
    let paragraph = typography.paragraph.as_ref().unwrap();
    assert_eq!(paragraph.p1.as_ref().unwrap().size.as_deref(), Some("16"));
    let button = email.settings.button.as_ref().unwrap();
    assert_eq!(button.border_radius.as_deref(), Some("4"));
    let footer = email.settings.footer.as_ref().unwrap();
    assert_eq!(footer.show_preferences, Some(false));
    assert_eq!(
        footer.company_name.as_ref().unwrap().editor_content["content"][0]["text"],
        "Acme"
    );
    assert_eq!(footer.unsubscribe_text.as_deref(), Some("Unsubscribe"));
    let social_link = &footer.social_links.as_ref().unwrap()[0];
    assert_eq!(social_link.url.as_deref(), Some("https://example.com"));
    assert_eq!(social_link.link_type.as_deref(), Some("website"));
    assert_eq!(social_link.link_id, Some(7));

    assert_eq!(email.blocks.len(), 2);
    assert_eq!(email.blocks[0].id, "1");
    assert_eq!(email.blocks[0].block_type, BlockType::Paragraph);
    assert_eq!(
        email.blocks[0].editor_content["attrs"],
        json!({"textAlign": "left"})
    );
    assert!(email.blocks[0].children.is_empty());
    assert!(email.blocks[0].placement.is_none());
    assert_eq!(email.blocks[1].block_type, BlockType::BulletedList);
    assert!(email.blocks[1].editor_content.is_empty());
    assert_eq!(email.blocks[1].children[0].id, "3");
}

#[tokio::test]
async fn find_decodes_null_values_and_keeps_unknown_keys() {
    let mut server: ServerGuard = Server::new_async().await;
    let body: String = json!({
        "data": {
            "id": "123",
            "type": "builder_inline_email",
            "emailable_type": "campaigns",
            "emailable_id": "456",
            "settings": {
                "brand": {"mode": null, "font_family": "Inter, sans-serif", "new_key": "x"},
                "layout": null,
                "new_group": {"a": 1}
            },
            "blocks": [{"id": "9", "type": "countdown", "editor_content": []}]
        },
        "etag": "42"
    })
    .to_string();
    expect_request(&mut server, "GET", "/emails/123", None, 200, &body).await;

    let response: ApiResponse<WithEtag<EmailDocument>> =
        mailerlite(&server).email().find("123").await.unwrap();

    let settings = &response.data.settings;
    let brand = settings.brand.as_ref().unwrap();
    assert_eq!(brand.mode, None);
    assert_eq!(brand.text_color, None);
    assert_eq!(brand.font_family.as_deref(), Some("Inter, sans-serif"));
    assert_eq!(brand.extra["new_key"], "x");
    assert!(settings.layout.is_none());
    assert!(settings.footer.is_none());
    assert_eq!(settings.extra["new_group"], json!({"a": 1}));

    let block = &response.data.blocks[0];
    assert_eq!(block.block_type, BlockType::Other("countdown".to_string()));
    assert!(block.editor_content.is_empty());
}

#[tokio::test]
async fn find_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let mailerlite = mailerlite(&server);

    assert!(matches!(
        mailerlite.email().find("").await,
        Err(Error::EmptyId { name: "email_id" })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            mailerlite.email().find(id).await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn html_gets_the_html() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "GET",
        "/emails/123/html",
        None,
        200,
        &fixture("email_html"),
    )
    .await;

    let response: ApiResponse<WithEtag<EmailHtml>> =
        mailerlite(&server).email().html("123").await.unwrap();

    mock.assert_async().await;
    assert_eq!(
        response.data.html,
        "<!doctype html><html><body><p>Hello</p></body></html>"
    );
    assert_eq!(response.etag, "42");
}

#[tokio::test]
async fn html_gives_an_api_error_for_a_503_with_an_html_body() {
    let mut server: ServerGuard = Server::new_async().await;
    server
        .mock("GET", "/emails/123/html")
        .with_status(503)
        .with_header("Content-Type", "text/html")
        .with_body("<html><body>Service Unavailable</body></html>")
        .create_async()
        .await;

    let error: Error = mailerlite(&server).email().html("123").await.unwrap_err();

    match error {
        Error::Api(error) => {
            assert_eq!(error.status_code, 503);
            assert_eq!(error.body, "<html><body>Service Unavailable</body></html>");
            assert_eq!(error.message, None);
            assert_eq!(error.code, None);
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn html_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let mailerlite = mailerlite(&server);

    assert!(matches!(
        mailerlite.email().html("").await,
        Err(Error::EmptyId { name: "email_id" })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            mailerlite.email().html(id).await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
        ));
    }

    assert_no_request(&mocks).await;
}
