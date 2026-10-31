//! HTTP contract regression tests for the migrated simple-editor API.
#![allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]

use crate::editor_support::{
    BAD_IDS, assert_no_request, expect_no_request, expect_request, fixture, mailerlite,
};
use mailerlite_rs::{
    Error, Response,
    models::emails::{
        BlockType, BrandMode, EditorText, EmailBlock, EmailDocument, EmailHtml, EmailTemplate,
        EmailVersion, Position, SettingsResult, VERSION_CONFLICT, Validation,
    },
    models::emails::{EtagOnly, WithEtag},
    pagination::NumberedPage,
    resources::emails::{
        BrandUpdate, EmailSettingsUpdate, Emails, FooterUpdate, LinkStyleUpdate,
        ListVersionsOptions, NewBlock, ParagraphStyleUpdate, Placement, SocialLinkUpdate,
        TextStyleUpdate, TypographyUpdate,
    },
};
use mockito::{Matcher, Mock, Server, ServerGuard};
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

    let response: Response<WithEtag<EmailDocument>> =
        mailerlite(&server).emails().get("123").await.unwrap();

    mock.assert_async().await;
    assert_eq!(response.metadata.status, 200);
    assert_eq!(response.data.etag, "42");

    let email: &EmailDocument = &response.data.data;
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

    let response: Response<WithEtag<EmailDocument>> =
        mailerlite(&server).emails().get("123").await.unwrap();

    let settings = &response.data.data.settings;
    let brand = settings.brand.as_ref().unwrap();
    assert_eq!(brand.mode, None);
    assert_eq!(brand.text_color, None);
    assert_eq!(brand.font_family.as_deref(), Some("Inter, sans-serif"));
    assert_eq!(brand.extra["new_key"], "x");
    assert!(settings.layout.is_none());
    assert!(settings.footer.is_none());
    assert_eq!(settings.extra["new_group"], json!({"a": 1}));

    let block = &response.data.data.blocks[0];
    assert_eq!(block.block_type, BlockType::Other("countdown".to_string()));
    assert!(block.editor_content.is_empty());
}

#[tokio::test]
async fn find_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let mailerlite = mailerlite(&server);

    assert!(matches!(
        mailerlite.emails().get("").await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            mailerlite.emails().get(id).await,
            Err(Error::Config(_))
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

    let response: Response<WithEtag<EmailHtml>> =
        mailerlite(&server).emails().html("123").await.unwrap();

    mock.assert_async().await;
    assert_eq!(
        response.data.data.html,
        "<!doctype html><html><body><p>Hello</p></body></html>"
    );
    assert_eq!(response.data.etag, "42");
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

    let error: Error = mailerlite(&server).emails().html("123").await.unwrap_err();

    match error {
        Error::Api(error) => {
            assert_eq!(error.status(), 503);
            assert_eq!(
                String::from_utf8_lossy(error.body()),
                "<html><body>Service Unavailable</body></html>"
            );
            assert_eq!(error.message(), None);
            assert_eq!(error.code(), None);
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
        mailerlite.emails().html("").await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            mailerlite.emails().html(id).await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn find_block_gets_the_block_and_its_placement() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "GET",
        "/emails/123/blocks/57",
        None,
        200,
        &fixture("email_block"),
    )
    .await;

    let response: Response<WithEtag<EmailBlock>> = mailerlite(&server)
        .emails()
        .get_block("123", "57")
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.data.etag, "43");
    assert_eq!(response.data.data.id, "57");
    assert_eq!(response.data.data.block_type, BlockType::Paragraph);
    assert_eq!(
        response.data.data.editor_content["content"][0]["text"],
        "Hello"
    );
    let placement = response.data.data.placement.as_ref().unwrap();
    assert_eq!(placement.after.as_deref(), Some("56"));
    assert_eq!(placement.before, None);
}

#[tokio::test]
async fn find_block_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let mailerlite = mailerlite(&server);

    assert!(matches!(
        mailerlite.emails().get_block("", "57").await,
        Err(Error::Config(_))
    ));
    assert!(matches!(
        mailerlite.emails().get_block("123", "").await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            mailerlite.emails().get_block(id, "57").await,
            Err(Error::Config(_))
        ));
        assert!(matches!(
            mailerlite.emails().get_block("123", id).await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

fn new_paragraph(placement: Option<Placement>) -> NewBlock {
    NewBlock {
        block_type: BlockType::Paragraph,
        editor_content:
            json!({"attrs": {"textAlign": "center"}, "content": [{"type": "text", "text": "Hi"}]})
                .as_object()
                .unwrap()
                .clone(),
        placement,
    }
}

#[tokio::test]
async fn create_block_sends_the_block_and_the_etag() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/blocks",
        Some(json!({
            "type": "paragraph",
            "editor_content": {"attrs": {"textAlign": "center"}, "content": [{"type": "text", "text": "Hi"}]},
            "placement": {"position": "after", "block_id": "57"},
            "etag": "42"
        })),
        201,
        &fixture("email_block_created"),
    )
    .await;

    let response: Response<WithEtag<EmailBlock>> = mailerlite(&server)
        .emails()
        .create_block("123", "42", &new_paragraph(Some(Placement::after("57"))))
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.metadata.status, 201);
    assert_eq!(response.data.etag, "43");
    assert_eq!(response.data.data.id, "58");
    let placement = response.data.data.placement.as_ref().unwrap();
    assert_eq!(placement.after.as_deref(), Some("57"));
    assert_eq!(placement.before, None);
}

#[tokio::test]
async fn create_block_sends_no_placement_when_there_is_none() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/blocks",
        Some(json!({"type": "divider", "editor_content": {}, "etag": "42"})),
        201,
        &fixture("email_block_created"),
    )
    .await;

    let block: NewBlock = NewBlock {
        block_type: BlockType::Divider,
        editor_content: serde_json::Map::new(),
        placement: None,
    };
    mailerlite(&server)
        .emails()
        .create_block("123", "42", &block)
        .await
        .unwrap();

    mock.assert_async().await;
}

#[test]
fn placement_constructors_give_the_position_and_the_block_id() {
    assert_eq!(
        serde_json::to_value(Placement::start()).unwrap(),
        json!({"position": "start"})
    );
    assert_eq!(
        serde_json::to_value(Placement::end()).unwrap(),
        json!({"position": "end"})
    );
    assert_eq!(
        serde_json::to_value(Placement::before("1")).unwrap(),
        json!({"position": "before", "block_id": "1"})
    );
    assert_eq!(
        serde_json::to_value(Placement::after("2")).unwrap(),
        json!({"position": "after", "block_id": "2"})
    );
}

#[tokio::test]
async fn create_block_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let client = mailerlite(&server);
    let email = client.emails();

    assert!(matches!(
        email.create_block("", "42", &new_paragraph(None)).await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.create_block(id, "42", &new_paragraph(None)).await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

fn invalid_placement() -> String {
    json!({
        "message": "The placement.block_id field is required when placement.position is after.",
        "errors": {
            "placement.block_id": [
                "The placement.block_id field is required when placement.position is after."
            ]
        }
    })
    .to_string()
}

#[tokio::test]
async fn create_block_sends_an_after_placement_without_a_block_id_and_gives_the_api_error() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/blocks",
        Some(json!({
            "type": "paragraph",
            "editor_content": {"attrs": {"textAlign": "center"}, "content": [{"type": "text", "text": "Hi"}]},
            "placement": {"position": "after"},
            "etag": "42"
        })),
        422,
        &invalid_placement(),
    )
    .await;

    let after: Placement = Placement {
        position: Position::After,
        block_id: None,
    };
    let error: Error = mailerlite(&server)
        .emails()
        .create_block("123", "42", &new_paragraph(Some(after)))
        .await
        .unwrap_err();

    mock.assert_async().await;
    match error {
        Error::Api(error) => {
            assert_eq!(error.status(), 422);
            assert!(error.validation_errors().contains_key("placement.block_id"));
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn create_block_sends_the_placement_block_id_as_given() {
    let mut server: ServerGuard = Server::new_async().await;
    let mut mocks: Vec<Mock> = Vec::new();
    for block_id in ["", "1/2"] {
        mocks.push(
            expect_request(
                &mut server,
                "POST",
                "/emails/123/blocks",
                Some(json!({
                    "type": "paragraph",
                    "editor_content": {"attrs": {"textAlign": "center"}, "content": [{"type": "text", "text": "Hi"}]},
                    "placement": {"position": "before", "block_id": block_id},
                    "etag": "42"
                })),
                422,
                &invalid_placement(),
            )
            .await,
        );
    }
    let client = mailerlite(&server);
    let email = client.emails();

    for block_id in ["", "1/2"] {
        let result = email
            .create_block(
                "123",
                "42",
                &new_paragraph(Some(Placement::before(block_id))),
            )
            .await;
        assert!(
            matches!(&result, Err(Error::Api(error)) if error.status() == 422),
            "expected Error::Api for {block_id:?}, got {result:?}"
        );
    }

    for mock in mocks {
        mock.assert_async().await;
    }
}

fn hello_again() -> serde_json::Map<String, serde_json::Value> {
    json!({"content": [{"type": "text", "text": "Hello again"}]})
        .as_object()
        .unwrap()
        .clone()
}

#[tokio::test]
async fn update_block_patches_the_content() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "PATCH",
        "/emails/123/blocks/57",
        Some(json!({
            "editor_content": {"content": [{"type": "text", "text": "Hello again"}]},
            "etag": "43"
        })),
        200,
        &fixture("email_block_updated"),
    )
    .await;

    let response: Response<WithEtag<EmailBlock>> = mailerlite(&server)
        .emails()
        .update_block("123", "57", "43", &hello_again())
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.data.etag, "44");
    assert_eq!(
        response.data.data.editor_content["content"][0]["text"],
        "Hello again"
    );
    let placement = response.data.data.placement.as_ref().unwrap();
    assert_eq!(placement.after.as_deref(), Some("56"));
    assert_eq!(placement.before.as_deref(), Some("58"));
}

#[tokio::test]
async fn update_block_gives_the_version_conflict() {
    let mut server: ServerGuard = Server::new_async().await;
    server
        .mock("PATCH", "/emails/123/blocks/57")
        .with_status(409)
        .with_header("Content-Type", "application/json")
        .with_body(r#"{"message": "The email was changed by another request.", "code": "version_conflict", "etag": "50"}"#)
        .create_async()
        .await;

    let error: Error = mailerlite(&server)
        .emails()
        .update_block("123", "57", "43", &hello_again())
        .await
        .unwrap_err();

    match error {
        Error::Api(error) => {
            assert_eq!(error.status(), 409);
            assert_eq!(error.code(), Some(VERSION_CONFLICT));
            assert_eq!(error.etag(), Some("50"));
            assert_eq!(
                error.message(),
                Some("The email was changed by another request.")
            );
            assert!(error.is_version_conflict());
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn update_block_gives_the_field_errors() {
    let mut server: ServerGuard = Server::new_async().await;
    server
        .mock("PATCH", "/emails/123/blocks/57")
        .with_status(422)
        .with_header("Content-Type", "application/json")
        .with_body(r#"{"message": "The editor content is invalid.", "errors": {"editor_content.content": ["The editor content is invalid."]}}"#)
        .create_async()
        .await;

    let error: Error = mailerlite(&server)
        .emails()
        .update_block("123", "57", "43", &hello_again())
        .await
        .unwrap_err();

    match error {
        Error::Api(error) => {
            assert_eq!(error.status(), 422);
            assert_eq!(
                error.validation_errors()["editor_content.content"],
                vec!["The editor content is invalid."]
            );
            assert_eq!(error.code(), None);
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn update_block_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let client = mailerlite(&server);
    let email = client.emails();

    assert!(matches!(
        email.update_block("", "57", "43", &hello_again()).await,
        Err(Error::Config(_))
    ));
    assert!(matches!(
        email.update_block("123", "", "43", &hello_again()).await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.update_block(id, "57", "43", &hello_again()).await,
            Err(Error::Config(_))
        ));
        assert!(matches!(
            email.update_block("123", id, "43", &hello_again()).await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn delete_block_sends_the_etag_in_the_body() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "DELETE",
        "/emails/123/blocks/57",
        Some(json!({"etag": "44"})),
        200,
        &fixture("email_etag"),
    )
    .await;

    let response: Response<EtagOnly> = mailerlite(&server)
        .emails()
        .delete_block("123", "57", "44")
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.metadata.status, 200);
    assert_eq!(response.data.etag, "45");
}

#[tokio::test]
async fn delete_block_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let client = mailerlite(&server);
    let email = client.emails();

    assert!(matches!(
        email.delete_block("", "57", "44").await,
        Err(Error::Config(_))
    ));
    assert!(matches!(
        email.delete_block("123", "", "44").await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.delete_block(id, "57", "44").await,
            Err(Error::Config(_))
        ));
        assert!(matches!(
            email.delete_block("123", id, "44").await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn move_block_sends_the_placement_and_the_etag() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/blocks/57/move",
        Some(json!({"placement": {"position": "before", "block_id": "1"}, "etag": "45"})),
        200,
        &fixture("email_block_moved"),
    )
    .await;

    let response: Response<WithEtag<EmailBlock>> = mailerlite(&server)
        .emails()
        .move_block("123", "57", "45", &Placement::before("1"))
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.data.etag, "46");
    let placement = response.data.data.placement.as_ref().unwrap();
    assert_eq!(placement.after, None);
    assert_eq!(placement.before.as_deref(), Some("1"));
}

#[tokio::test]
async fn move_block_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let client = mailerlite(&server);
    let email = client.emails();

    assert!(matches!(
        email.move_block("", "57", "45", &Placement::start()).await,
        Err(Error::Config(_))
    ));
    assert!(matches!(
        email.move_block("123", "", "45", &Placement::start()).await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.move_block(id, "57", "45", &Placement::start()).await,
            Err(Error::Config(_))
        ));
        assert!(matches!(
            email.move_block("123", id, "45", &Placement::start()).await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn placement_at_start_or_end_sends_the_block_id_as_given() {
    let mut server: ServerGuard = Server::new_async().await;
    let at_end: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/blocks/57/move",
        Some(json!({"placement": {"position": "end", "block_id": ""}, "etag": "45"})),
        200,
        &fixture("email_block_moved"),
    )
    .await;
    let at_start: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/blocks",
        Some(json!({
            "type": "paragraph",
            "editor_content": {"attrs": {"textAlign": "center"}, "content": [{"type": "text", "text": "Hi"}]},
            "placement": {"position": "start", "block_id": "1/2"},
            "etag": "42"
        })),
        201,
        &fixture("email_block_created"),
    )
    .await;
    let client = mailerlite(&server);
    let email = client.emails();

    let end: Placement = Placement {
        position: Position::End,
        block_id: Some(String::new()),
    };
    email.move_block("123", "57", "45", &end).await.unwrap();

    let start: Placement = Placement {
        position: Position::Start,
        block_id: Some("1/2".to_string()),
    };
    email
        .create_block("123", "42", &new_paragraph(Some(start)))
        .await
        .unwrap();

    at_end.assert_async().await;
    at_start.assert_async().await;
}

#[tokio::test]
async fn move_block_sends_the_placement_as_given_and_gives_the_api_error() {
    let mut server: ServerGuard = Server::new_async().await;
    let placements: [(Placement, serde_json::Value); 3] = [
        (
            Placement {
                position: Position::After,
                block_id: None,
            },
            json!({"position": "after"}),
        ),
        (
            Placement::before("1/2"),
            json!({"position": "before", "block_id": "1/2"}),
        ),
        (
            Placement {
                position: Position::Other("next_to".to_string()),
                block_id: Some("1/2".to_string()),
            },
            json!({"position": "next_to", "block_id": "1/2"}),
        ),
    ];
    let client = mailerlite(&server);
    let email = client.emails();

    for (placement, sent) in placements {
        let mock: Mock = expect_request(
            &mut server,
            "POST",
            "/emails/123/blocks/57/move",
            Some(json!({"placement": sent, "etag": "45"})),
            422,
            &invalid_placement(),
        )
        .await;

        let result = email.move_block("123", "57", "45", &placement).await;

        mock.assert_async().await;
        assert!(
            matches!(&result, Err(Error::Api(error)) if error.status() == 422),
            "expected Error::Api for {placement:?}, got {result:?}"
        );
        mock.remove_async().await;
    }
}

#[tokio::test]
async fn placement_with_an_unknown_position_sends_its_block_id() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/blocks/57/move",
        Some(json!({"placement": {"position": "next_to", "block_id": "1"}, "etag": "45"})),
        200,
        &fixture("email_block_moved"),
    )
    .await;

    let other: Placement = Placement {
        position: Position::Other("next_to".to_string()),
        block_id: Some("1".to_string()),
    };
    mailerlite(&server)
        .emails()
        .move_block("123", "57", "45", &other)
        .await
        .unwrap();

    mock.assert_async().await;
}

#[tokio::test]
async fn validate_block_sends_the_block_without_an_etag() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/blocks/validate",
        Some(json!({"type": "divider", "editor_content": {}})),
        200,
        &fixture("email_block_validation_valid"),
    )
    .await;

    let block: NewBlock = NewBlock {
        block_type: BlockType::Divider,
        editor_content: serde_json::Map::new(),
        placement: None,
    };
    let response: Response<Validation> = mailerlite(&server)
        .emails()
        .validate_block(&block)
        .await
        .unwrap();

    mock.assert_async().await;
    assert!(response.data.valid);
    assert!(response.data.errors.is_empty());
    assert_eq!(response.data.schema.version.as_deref(), Some("2.73.2"));
    assert_eq!(response.data.schema.hash.as_deref(), Some("0f3a9c"));
    assert!(response.data.rejected.is_none());
}

#[tokio::test]
async fn validate_block_gives_the_errors_of_an_invalid_block() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/blocks/validate",
        Some(json!({"type": "countdown", "editor_content": {}})),
        200,
        &fixture("email_block_validation_invalid"),
    )
    .await;

    let block: NewBlock = NewBlock {
        block_type: BlockType::Other("countdown".to_string()),
        editor_content: serde_json::Map::new(),
        placement: None,
    };
    let response: Response<Validation> = mailerlite(&server)
        .emails()
        .validate_block(&block)
        .await
        .unwrap();

    mock.assert_async().await;
    assert!(!response.data.valid);
    assert_eq!(
        response.data.errors["type"],
        vec!["The selected type is invalid."]
    );
}

#[tokio::test]
async fn validate_block_accepts_an_empty_array_for_the_errors() {
    let mut server: ServerGuard = Server::new_async().await;
    expect_request(
        &mut server,
        "POST",
        "/emails/blocks/validate",
        Some(json!({"type": "divider", "editor_content": {}})),
        200,
        r#"{"valid": true, "errors": [], "schema": {"version": "2.73.2", "hash": "0f3a9c"}}"#,
    )
    .await;

    let block: NewBlock = NewBlock {
        block_type: BlockType::Divider,
        editor_content: serde_json::Map::new(),
        placement: None,
    };
    let response: Response<Validation> = mailerlite(&server)
        .emails()
        .validate_block(&block)
        .await
        .unwrap();

    assert!(response.data.valid);
    assert!(response.data.errors.is_empty());
}

#[tokio::test]
async fn validate_block_sends_a_bad_placement_and_gives_the_errors() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/blocks/validate",
        Some(json!({
            "type": "paragraph",
            "editor_content": {"attrs": {"textAlign": "center"}, "content": [{"type": "text", "text": "Hi"}]},
            "placement": {"position": "after", "block_id": "1/2"}
        })),
        200,
        &fixture("email_block_validation_invalid"),
    )
    .await;

    let block: NewBlock = new_paragraph(Some(Placement::after("1/2")));
    let response: Response<Validation> = mailerlite(&server)
        .emails()
        .validate_block(&block)
        .await
        .unwrap();

    mock.assert_async().await;
    assert!(!response.data.valid);
}

fn settings_update() -> EmailSettingsUpdate {
    let mut extra: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    extra.insert("new_group".to_string(), json!({"a": "1"}));

    EmailSettingsUpdate {
        brand: Some(BrandUpdate {
            mode: Some(BrandMode::Custom.into()),
            style: Some("preset-retro".to_string()),
            background_color: Some("#f4f4f4".to_string()),
            text_color: Some(String::new()),
            ..Default::default()
        }),
        typography: Some(TypographyUpdate {
            paragraph: Some(ParagraphStyleUpdate {
                p1: Some(TextStyleUpdate {
                    size: Some("18".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }),
        footer: Some(FooterUpdate {
            show_preferences: Some(true),
            info_text: Some(EditorText::default()),
            social_links: Some(vec![SocialLinkUpdate {
                url: Some("https://example.com".to_string()),
                link_type: Some("website".to_string()),
                label: Some("Website".to_string()),
                link_id: Some(7),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        extra,
        ..Default::default()
    }
}

fn settings_update_json() -> serde_json::Value {
    json!({
        "brand": {
            "mode": "custom",
            "style": "preset-retro",
            "background_color": "#f4f4f4",
            "text_color": ""
        },
        "typography": {"paragraph": {"p1": {"size": "18"}}},
        "footer": {
            "show_preferences": true,
            "info_text": {"editor_content": {}},
            "social_links": [
                {"url": "https://example.com", "type": "website", "label": "Website", "link_id": 7}
            ]
        },
        "new_group": {"a": "1"}
    })
}

#[tokio::test]
async fn update_settings_sends_only_the_keys_that_are_set() {
    let mut server: ServerGuard = Server::new_async().await;
    let mut body: serde_json::Value = settings_update_json();
    body["etag"] = json!("46");
    let mock: Mock = expect_request(
        &mut server,
        "PATCH",
        "/emails/123/settings",
        Some(body),
        200,
        &fixture("email_settings"),
    )
    .await;

    let response: Response<SettingsResult> = mailerlite(&server)
        .emails()
        .update_settings("123", "46", &settings_update())
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.data.etag, "47");
    let brand = response.data.data.brand.as_ref().unwrap();
    assert_eq!(brand.mode.as_deref(), Some("custom"));
    assert_eq!(brand.background_color.as_deref(), Some("#f4f4f4"));
    let rejected = response.data.rejected.as_ref().unwrap();
    assert_eq!(rejected.message, "Some attributes were rejected.");
    assert_eq!(rejected.keys, vec!["footer.social_links[0].url"]);
}

#[tokio::test]
async fn update_settings_sends_an_empty_object_for_empty_settings() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "PATCH",
        "/emails/123/settings",
        Some(json!({"etag": "46"})),
        200,
        &fixture("email_settings"),
    )
    .await;

    mailerlite(&server)
        .emails()
        .update_settings("123", "46", &EmailSettingsUpdate::default())
        .await
        .unwrap();

    mock.assert_async().await;
}

#[tokio::test]
async fn update_settings_checks_the_input_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let client = mailerlite(&server);
    let email = client.emails();

    let mut with_etag: EmailSettingsUpdate = settings_update();
    with_etag.extra.insert("etag".to_string(), json!("1"));
    assert!(matches!(
        email.update_settings("123", "46", &with_etag).await,
        Err(Error::Config(_))
    ));
    assert!(matches!(
        email.update_settings("", "46", &settings_update()).await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.update_settings(id, "46", &settings_update()).await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn validate_settings_sends_the_settings_without_an_etag() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/settings/validate",
        Some(settings_update_json()),
        200,
        &fixture("email_settings_validation"),
    )
    .await;

    let response: Response<Validation> = mailerlite(&server)
        .emails()
        .validate_settings("123", &settings_update())
        .await
        .unwrap();

    mock.assert_async().await;
    assert!(!response.data.valid);
    assert_eq!(
        response.data.errors["layout.width"],
        vec!["The layout.width field cannot be changed."]
    );
    assert_eq!(response.data.schema.version.as_deref(), Some("2.73.2"));
    assert_eq!(
        response.data.rejected.as_ref().unwrap().keys,
        vec!["footer.social_links[0].url"]
    );
}

#[tokio::test]
async fn validate_settings_sends_an_etag_key_of_the_extra_map() {
    let mut server: ServerGuard = Server::new_async().await;
    let mut sent: serde_json::Value = settings_update_json();
    sent["etag"] = json!("1");
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/settings/validate",
        Some(sent),
        200,
        &fixture("email_settings_validation"),
    )
    .await;

    let mut with_etag: EmailSettingsUpdate = settings_update();
    with_etag.extra.insert("etag".to_string(), json!("1"));
    mailerlite(&server)
        .emails()
        .validate_settings("123", &with_etag)
        .await
        .unwrap();

    mock.assert_async().await;
}

#[tokio::test]
async fn validate_settings_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let client = mailerlite(&server);
    let email = client.emails();

    assert!(matches!(
        email.validate_settings("", &settings_update()).await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.validate_settings(id, &settings_update()).await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn create_template_sends_the_name() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/templates",
        Some(json!({"name": "Newsletter"})),
        201,
        &fixture("email_template"),
    )
    .await;

    let response: Response<EmailTemplate> = mailerlite(&server)
        .emails()
        .create_template("123", "Newsletter")
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.metadata.status, 201);
    assert_eq!(response.data.id, "789");
    assert_eq!(response.data.name, "Newsletter");
    assert_eq!(response.data.template_type, "email");
    assert_eq!(response.data.created_at, "2026-10-28 12:00:00");
}

#[tokio::test]
async fn create_template_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let client = mailerlite(&server);
    let email = client.emails();

    assert!(matches!(
        email.create_template("", "Newsletter").await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.create_template(id, "Newsletter").await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn versions_lists_the_versions_with_the_page_options() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "GET",
        "/emails/123/versions?page=2&limit=10",
        None,
        200,
        &fixture("email_versions"),
    )
    .await;

    let options: ListVersionsOptions = ListVersionsOptions {
        page: Some(2),
        limit: Some(10),
    };
    let response: Response<NumberedPage<EmailVersion>> = mailerlite(&server)
        .emails()
        .versions("123", options)
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.data.data.len(), 2);
    assert_eq!(response.data.data[0].id, "902");
    assert_eq!(response.data.data[0].created_at, "2026-10-28 12:05:00");
    assert!(response.data.data[0].current);
    assert!(!response.data.data[1].current);
    assert_eq!(response.data.links.prev, None);
    assert_eq!(response.data.links.next, None);
    assert_eq!(response.data.meta.from, Some(1));
    assert_eq!(response.data.meta.to, Some(2));
    assert_eq!(response.data.meta.total, Some(2));
}

#[tokio::test]
async fn versions_sends_no_query_without_options() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "GET",
        "/emails/123/versions",
        None,
        200,
        &fixture("email_versions"),
    )
    .await;

    mailerlite(&server)
        .emails()
        .versions("123", Default::default())
        .await
        .unwrap();

    mock.assert_async().await;
}

#[tokio::test]
async fn versions_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let client = mailerlite(&server);
    let email = client.emails();

    assert!(matches!(
        email.versions("", Default::default()).await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.versions(id, Default::default()).await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn create_version_sends_no_body_and_gives_the_status_code() {
    for status in [201, 200] {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = expect_request(
            &mut server,
            "POST",
            "/emails/123/versions",
            None,
            status,
            &fixture("email_version"),
        )
        .await;

        let response: Response<EmailVersion> = mailerlite(&server)
            .emails()
            .create_version("123")
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(response.metadata.status.as_u16() as usize, status);
        assert_eq!(response.data.id, "903");
        assert!(response.data.current);
    }
}

#[tokio::test]
async fn create_version_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let client = mailerlite(&server);
    let email = client.emails();

    assert!(matches!(
        email.create_version("").await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.create_version(id).await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn restore_version_sends_the_etag_and_gives_the_email() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/versions/901/restore",
        Some(json!({"etag": "41"})),
        200,
        &fixture("email"),
    )
    .await;

    let response: Response<WithEtag<EmailDocument>> = mailerlite(&server)
        .emails()
        .restore_version("123", "901", "41")
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.data.etag, "42");
    assert_eq!(response.data.data.id, "123");
    assert_eq!(response.data.data.blocks.len(), 2);
}

#[tokio::test]
async fn restore_version_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let client = mailerlite(&server);
    let email = client.emails();

    assert!(matches!(
        email.restore_version("", "901", "41").await,
        Err(Error::Config(_))
    ));
    assert!(matches!(
        email.restore_version("123", "", "41").await,
        Err(Error::Config(_))
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.restore_version(id, "901", "41").await,
            Err(Error::Config(_))
        ));
        assert!(matches!(
            email.restore_version("123", id, "41").await,
            Err(Error::Config(_))
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn the_6_writes_with_an_etag_send_an_empty_etag_and_give_the_api_error() {
    let mut server: ServerGuard = Server::new_async().await;
    let writes: [(&str, &str); 6] = [
        ("POST", "/emails/123/blocks"),
        ("PATCH", "/emails/123/blocks/57"),
        ("DELETE", "/emails/123/blocks/57"),
        ("POST", "/emails/123/blocks/57/move"),
        ("PATCH", "/emails/123/settings"),
        ("POST", "/emails/123/versions/901/restore"),
    ];
    let mut mocks: Vec<Mock> = Vec::new();
    for (method, path) in writes {
        mocks.push(
            server
                .mock(method, path)
                .match_body(Matcher::PartialJson(json!({"etag": ""})))
                .with_status(422)
                .with_header("Content-Type", "application/json")
                .with_body(r#"{"message": "The etag field is required.", "errors": {"etag": ["The etag field is required."]}}"#)
                .expect(1)
                .create_async()
                .await,
        );
    }
    let client = mailerlite(&server);
    let email = client.emails();

    let results: Vec<Result<(), Error>> = vec![
        email
            .create_block("123", "", &new_paragraph(None))
            .await
            .map(|_| ()),
        email
            .update_block("123", "57", "", &hello_again())
            .await
            .map(|_| ()),
        email.delete_block("123", "57", "").await.map(|_| ()),
        email
            .move_block("123", "57", "", &Placement::start())
            .await
            .map(|_| ()),
        email
            .update_settings("123", "", &settings_update())
            .await
            .map(|_| ()),
        email.restore_version("123", "901", "").await.map(|_| ()),
    ];

    for mock in mocks {
        mock.assert_async().await;
    }
    for result in results {
        match result {
            Err(Error::Api(error)) => {
                assert_eq!(error.status(), 422);
                assert!(error.validation_errors().contains_key("etag"));
            }
            other => panic!("expected Error::Api, got {other:?}"),
        }
    }
}

/// Sends each of the 8 writes that accept an idempotency key, with valid input.
async fn send_all_writes(email: &Emails<'_>) -> Vec<Result<(), Error>> {
    let block: NewBlock = new_paragraph(None);
    vec![
        email.create_block("123", "42", &block).await.map(|_| ()),
        email
            .update_block("123", "57", "43", &hello_again())
            .await
            .map(|_| ()),
        email.delete_block("123", "57", "44").await.map(|_| ()),
        email
            .move_block("123", "57", "45", &Placement::start())
            .await
            .map(|_| ()),
        email
            .update_settings("123", "46", &settings_update())
            .await
            .map(|_| ()),
        email.create_template("123", "Newsletter").await.map(|_| ()),
        email.create_version("123").await.map(|_| ()),
        email.restore_version("123", "901", "41").await.map(|_| ()),
    ]
}

async fn mock_all_writes(server: &mut ServerGuard, key: Matcher) -> Vec<Mock> {
    let writes: [(&str, &str, &str); 8] = [
        ("POST", "/emails/123/blocks", "email_block_created"),
        ("PATCH", "/emails/123/blocks/57", "email_block_updated"),
        ("DELETE", "/emails/123/blocks/57", "email_etag"),
        ("POST", "/emails/123/blocks/57/move", "email_block_moved"),
        ("PATCH", "/emails/123/settings", "email_settings"),
        ("POST", "/emails/123/templates", "email_template"),
        ("POST", "/emails/123/versions", "email_version"),
        ("POST", "/emails/123/versions/901/restore", "email"),
    ];
    let mut mocks: Vec<Mock> = Vec::new();
    for (method, path, response) in writes {
        mocks.push(
            server
                .mock(method, path)
                .match_header("Idempotency-Key", key.clone())
                .with_status(
                    if path.ends_with("/blocks") || path.ends_with("/templates") {
                        201
                    } else {
                        200
                    },
                )
                .with_body(fixture(response))
                .expect(1)
                .create_async()
                .await,
        );
    }

    mocks
}

#[tokio::test]
async fn the_8_writes_send_the_idempotency_key() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = mock_all_writes(&mut server, Matcher::Exact("key-1".to_string())).await;
    let client = mailerlite(&server);
    let email: Emails = client.emails().with_idempotency_key("key-1");

    for result in send_all_writes(&email).await {
        result.unwrap();
    }

    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn the_8_writes_send_no_idempotency_key_without_a_key() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = mock_all_writes(&mut server, Matcher::Missing).await;
    let client = mailerlite(&server);
    let email: Emails = client.emails();

    for result in send_all_writes(&email).await {
        result.unwrap();
    }

    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn reads_and_validations_ignore_the_idempotency_key() {
    let mut server: ServerGuard = Server::new_async().await;
    let reads: [(&str, &str, &str); 6] = [
        ("GET", "/emails/123", "email"),
        ("GET", "/emails/123/html", "email_html"),
        ("GET", "/emails/123/blocks/57", "email_block"),
        ("GET", "/emails/123/versions", "email_versions"),
        (
            "POST",
            "/emails/blocks/validate",
            "email_block_validation_valid",
        ),
        (
            "POST",
            "/emails/123/settings/validate",
            "email_settings_validation",
        ),
    ];
    let mut mocks: Vec<Mock> = Vec::new();
    for (method, path, response) in reads {
        mocks.push(
            server
                .mock(method, path)
                .match_header("Idempotency-Key", Matcher::Missing)
                .with_status(200)
                .with_body(fixture(response))
                .expect(1)
                .create_async()
                .await,
        );
    }
    let client = mailerlite(&server);
    let email: Emails = client.emails().with_idempotency_key("key-1");

    email.get("123").await.unwrap();
    email.html("123").await.unwrap();
    email.get_block("123", "57").await.unwrap();
    email.versions("123", Default::default()).await.unwrap();
    email.validate_block(&new_paragraph(None)).await.unwrap();
    email
        .validate_settings("123", &settings_update())
        .await
        .unwrap();

    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn the_8_writes_send_the_idempotency_key_as_given() {
    let mut server: ServerGuard = Server::new_async().await;
    let too_long: String = "k".repeat(256);

    // HTTP removes a space at the start or the end of a header value, so the API gets "key".
    for (key, received) in [
        ("", ""),
        (too_long.as_str(), too_long.as_str()),
        ("key\tkey", "key\tkey"),
        (" key", "key"),
        ("key ", "key"),
    ] {
        let mocks: Vec<Mock> =
            mock_all_writes(&mut server, Matcher::Exact(received.to_string())).await;
        let client = mailerlite(&server);
        let email: Emails = client.emails().with_idempotency_key(key);

        for result in send_all_writes(&email).await {
            assert!(
                result.is_ok(),
                "expected a request for {key:?}, got {result:?}"
            );
        }
        for mock in mocks {
            mock.assert_async().await;
            mock.remove_async().await;
        }
    }
}

#[tokio::test]
async fn a_key_that_http_cannot_carry_stops_the_writes_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;

    for key in ["key\n", "key\r", "key\x7f"] {
        let client = mailerlite(&server);
        let email: Emails = client.emails().with_idempotency_key(key);
        for result in send_all_writes(&email).await {
            assert!(
                matches!(result, Err(Error::Config(_))),
                "expected InvalidIdempotencyKey for {key:?}"
            );
        }
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn a_255_character_key_is_sent() {
    let mut server: ServerGuard = Server::new_async().await;
    let key: String = "k".repeat(255);
    let mock: Mock = server
        .mock("POST", "/emails/123/versions")
        .match_header("Idempotency-Key", key.as_str())
        .with_status(201)
        .with_body(fixture("email_version"))
        .create_async()
        .await;

    mailerlite(&server)
        .emails()
        .with_idempotency_key(key.clone())
        .create_version("123")
        .await
        .unwrap();

    mock.assert_async().await;
}

#[tokio::test]
async fn a_replayed_write_has_the_replay_flag() {
    let mut server: ServerGuard = Server::new_async().await;
    server
        .mock("POST", "/emails/123/versions")
        .match_header("Idempotency-Key", "key-1")
        .with_status(201)
        .with_header("Idempotent-Replayed", "True")
        .with_body(fixture("email_version"))
        .create_async()
        .await;

    let response: Response<EmailVersion> = mailerlite(&server)
        .emails()
        .with_idempotency_key("key-1")
        .create_version("123")
        .await
        .unwrap();

    assert!(response.metadata.is_idempotent_replay());
    assert_eq!(response.metadata.status, 201);
}

#[tokio::test]
async fn validate_block_decodes_a_null_schema() {
    let mut server: ServerGuard = Server::new_async().await;
    expect_request(
        &mut server,
        "POST",
        "/emails/blocks/validate",
        Some(json!({"type": "divider", "editor_content": {}})),
        200,
        r#"{"valid": false, "errors": {"type": ["The selected type is invalid."]}, "schema": {"version": null, "hash": null}}"#,
    )
    .await;

    let block: NewBlock = NewBlock {
        block_type: BlockType::Divider,
        editor_content: serde_json::Map::new(),
        placement: None,
    };
    let response: Response<Validation> = mailerlite(&server)
        .emails()
        .validate_block(&block)
        .await
        .unwrap();

    assert!(!response.data.valid);
    assert_eq!(
        response.data.errors["type"],
        vec!["The selected type is invalid."]
    );
    assert_eq!(response.data.schema.version, None);
    assert_eq!(response.data.schema.hash, None);
}

#[test]
fn an_other_value_equals_the_known_variant_with_the_same_value() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash(value: &BlockType) -> u64 {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    let other: BlockType = BlockType::Other("paragraph".to_string());
    assert_eq!(other, BlockType::Paragraph);
    assert_eq!(hash(&other), hash(&BlockType::Paragraph));
    assert_ne!(
        BlockType::Other("countdown".to_string()),
        BlockType::Paragraph
    );
    assert_eq!(Position::Other("end".to_string()), Position::End);
    assert!(matches!(
        BlockType::from("paragraph".to_string()),
        BlockType::Paragraph
    ));
}

#[tokio::test]
async fn settings_with_an_extra_key_that_is_also_a_field_send_no_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let client = mailerlite(&server);
    let email = client.emails();

    let mut top: EmailSettingsUpdate = settings_update();
    top.extra
        .insert("brand".to_string(), json!({"mode": "account"}));

    let mut nested: EmailSettingsUpdate = settings_update();
    let typography: &mut TypographyUpdate = nested.typography.as_mut().unwrap();
    typography.link = Some(LinkStyleUpdate {
        color: Some("#000000".to_string()),
        ..Default::default()
    });
    typography
        .extra
        .insert("link".to_string(), json!({"color": "#ffffff"}));

    let mut social: EmailSettingsUpdate = settings_update();
    social
        .footer
        .as_mut()
        .unwrap()
        .social_links
        .as_mut()
        .unwrap()[0]
        .extra
        .insert("type".to_string(), json!("x"));

    for (settings, expected) in [
        (&top, "brand"),
        (&nested, "typography.link"),
        (&social, "footer.social_links[0].type"),
    ] {
        for result in [
            email
                .update_settings("123", "46", settings)
                .await
                .map(|_| ()),
            email.validate_settings("123", settings).await.map(|_| ()),
        ] {
            match result {
                Err(Error::Config(message)) => assert!(message.contains(expected)),
                other => panic!("expected DuplicateSettingsKey for {expected}, got {other:?}"),
            }
        }
    }

    let mut with_etag: EmailSettingsUpdate = settings_update();
    with_etag.extra.insert("etag".to_string(), json!("1"));
    assert!(matches!(
        email.update_settings("123", "46", &with_etag).await,
        Err(Error::Config(_))
    ));

    assert_no_request(&mocks).await;
}
