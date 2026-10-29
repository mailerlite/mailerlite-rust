mod common;

use common::{assert_no_request, expect_no_request, expect_request, fixture, mailerlite, BAD_IDS};
use mailerlite_rs::{
    api_response::{ApiResponse, DataOnly, EtagOnly, WithEtag},
    email::{
        BlockType, BrandMode, BrandUpdate, EditorText, Email, EmailBlock, EmailDocument, EmailHtml,
        EmailSettingsUpdate, EmailTemplate, EmailVersion, FooterUpdate, ListVersionsOptions,
        NewBlock, ParagraphStyleUpdate, Placement, Position, SettingsResult, SocialLinkUpdate,
        TextStyleUpdate, TypographyUpdate, Validation, VERSION_CONFLICT,
    },
    error::Error,
    pagination::Paginated,
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
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
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
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
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

    let response: ApiResponse<WithEtag<EmailBlock>> = mailerlite(&server)
        .email()
        .find_block("123", "57")
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.etag, "43");
    assert_eq!(response.data.id, "57");
    assert_eq!(response.data.block_type, BlockType::Paragraph);
    assert_eq!(response.data.editor_content["content"][0]["text"], "Hello");
    let placement = response.data.placement.as_ref().unwrap();
    assert_eq!(placement.after.as_deref(), Some("56"));
    assert_eq!(placement.before, None);
}

#[tokio::test]
async fn find_block_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let mailerlite = mailerlite(&server);

    assert!(matches!(
        mailerlite.email().find_block("", "57").await,
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
    ));
    assert!(matches!(
        mailerlite.email().find_block("123", "").await,
        Err(Error::EmptyId {
            name: "block_id",
            ..
        })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            mailerlite.email().find_block(id, "57").await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
        ));
        assert!(matches!(
            mailerlite.email().find_block("123", id).await,
            Err(Error::InvalidId {
                name: "block_id",
                ..
            })
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

    let response: ApiResponse<WithEtag<EmailBlock>> = mailerlite(&server)
        .email()
        .create_block("123", "42", &new_paragraph(Some(Placement::after("57"))))
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.status_code, 201);
    assert_eq!(response.etag, "43");
    assert_eq!(response.data.id, "58");
    let placement = response.data.placement.as_ref().unwrap();
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
        ..Default::default()
    };
    mailerlite(&server)
        .email()
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
async fn create_block_checks_the_input_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let email = mailerlite(&server).email();

    assert!(matches!(
        email.create_block("123", "", &new_paragraph(None)).await,
        Err(Error::EmptyEtag)
    ));
    assert!(matches!(
        email.create_block("", "42", &new_paragraph(None)).await,
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
    ));
    assert!(matches!(
        email
            .create_block("123", "42", &new_paragraph(Some(Placement::after(""))))
            .await,
        Err(Error::EmptyId {
            name: "placement.block_id",
            ..
        })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.create_block(id, "42", &new_paragraph(None)).await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
        ));
        assert!(matches!(
            email
                .create_block("123", "42", &new_paragraph(Some(Placement::before(id))))
                .await,
            Err(Error::InvalidId {
                name: "placement.block_id",
                ..
            })
        ));
    }

    assert_no_request(&mocks).await;
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

    let response: ApiResponse<WithEtag<EmailBlock>> = mailerlite(&server)
        .email()
        .update_block("123", "57", "43", &hello_again())
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.etag, "44");
    assert_eq!(
        response.data.editor_content["content"][0]["text"],
        "Hello again"
    );
    let placement = response.data.placement.as_ref().unwrap();
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
        .email()
        .update_block("123", "57", "43", &hello_again())
        .await
        .unwrap_err();

    match error {
        Error::Api(error) => {
            assert_eq!(error.status_code, 409);
            assert_eq!(error.code.as_deref(), Some(VERSION_CONFLICT));
            assert_eq!(error.etag.as_deref(), Some("50"));
            assert_eq!(
                error.message.as_deref(),
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
        .email()
        .update_block("123", "57", "43", &hello_again())
        .await
        .unwrap_err();

    match error {
        Error::Api(error) => {
            assert_eq!(error.status_code, 422);
            assert_eq!(
                error.errors.unwrap()["editor_content.content"],
                vec!["The editor content is invalid."]
            );
            assert_eq!(error.code, None);
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn update_block_checks_the_input_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let email = mailerlite(&server).email();

    assert!(matches!(
        email.update_block("123", "57", "", &hello_again()).await,
        Err(Error::EmptyEtag)
    ));
    assert!(matches!(
        email.update_block("", "57", "43", &hello_again()).await,
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
    ));
    assert!(matches!(
        email.update_block("123", "", "43", &hello_again()).await,
        Err(Error::EmptyId {
            name: "block_id",
            ..
        })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.update_block(id, "57", "43", &hello_again()).await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
        ));
        assert!(matches!(
            email.update_block("123", id, "43", &hello_again()).await,
            Err(Error::InvalidId {
                name: "block_id",
                ..
            })
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

    let response: ApiResponse<EtagOnly> = mailerlite(&server)
        .email()
        .delete_block("123", "57", "44")
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.status_code, 200);
    assert_eq!(response.etag, "45");
}

#[tokio::test]
async fn delete_block_checks_the_input_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let email = mailerlite(&server).email();

    assert!(matches!(
        email.delete_block("123", "57", "").await,
        Err(Error::EmptyEtag)
    ));
    assert!(matches!(
        email.delete_block("", "57", "44").await,
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
    ));
    assert!(matches!(
        email.delete_block("123", "", "44").await,
        Err(Error::EmptyId {
            name: "block_id",
            ..
        })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.delete_block(id, "57", "44").await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
        ));
        assert!(matches!(
            email.delete_block("123", id, "44").await,
            Err(Error::InvalidId {
                name: "block_id",
                ..
            })
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

    let response: ApiResponse<WithEtag<EmailBlock>> = mailerlite(&server)
        .email()
        .move_block("123", "57", "45", &Placement::before("1"))
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.etag, "46");
    let placement = response.data.placement.as_ref().unwrap();
    assert_eq!(placement.after, None);
    assert_eq!(placement.before.as_deref(), Some("1"));
}

#[tokio::test]
async fn move_block_checks_the_input_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let email = mailerlite(&server).email();

    assert!(matches!(
        email.move_block("123", "57", "", &Placement::start()).await,
        Err(Error::EmptyEtag)
    ));
    assert!(matches!(
        email.move_block("", "57", "45", &Placement::start()).await,
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
    ));
    assert!(matches!(
        email.move_block("123", "", "45", &Placement::start()).await,
        Err(Error::EmptyId {
            name: "block_id",
            ..
        })
    ));
    assert!(matches!(
        email
            .move_block("123", "57", "45", &Placement::after(""))
            .await,
        Err(Error::EmptyId {
            name: "placement.block_id",
            ..
        })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.move_block(id, "57", "45", &Placement::start()).await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
        ));
        assert!(matches!(
            email.move_block("123", id, "45", &Placement::start()).await,
            Err(Error::InvalidId {
                name: "block_id",
                ..
            })
        ));
        assert!(matches!(
            email
                .move_block("123", "57", "45", &Placement::after(id))
                .await,
            Err(Error::InvalidId {
                name: "placement.block_id",
                ..
            })
        ));
    }

    assert_no_request(&mocks).await;
}

#[tokio::test]
async fn placement_at_start_or_end_sends_no_block_id_and_has_no_check() {
    let mut server: ServerGuard = Server::new_async().await;
    let at_end: Mock = expect_request(
        &mut server,
        "POST",
        "/emails/123/blocks/57/move",
        Some(json!({"placement": {"position": "end"}, "etag": "45"})),
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
            "placement": {"position": "start"},
            "etag": "42"
        })),
        201,
        &fixture("email_block_created"),
    )
    .await;
    let email = mailerlite(&server).email();

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
async fn placement_before_or_after_needs_a_valid_block_id() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let email = mailerlite(&server).email();

    for block_id in [None, Some(String::new())] {
        let after: Placement = Placement {
            position: Position::After,
            block_id,
        };
        assert!(matches!(
            email.move_block("123", "57", "45", &after).await,
            Err(Error::EmptyId {
                name: "placement.block_id",
                ..
            })
        ));
    }
    let before: Placement = Placement {
        position: Position::Before,
        block_id: Some("1/2".to_string()),
    };
    assert!(matches!(
        email.move_block("123", "57", "45", &before).await,
        Err(Error::InvalidId {
            name: "placement.block_id",
            ..
        })
    ));
    let other: Placement = Placement {
        position: Position::Other("next_to".to_string()),
        block_id: Some("1/2".to_string()),
    };
    assert!(matches!(
        email.move_block("123", "57", "45", &other).await,
        Err(Error::InvalidId {
            name: "placement.block_id",
            ..
        })
    ));

    assert_no_request(&mocks).await;
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
        .email()
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
        ..Default::default()
    };
    let response: ApiResponse<Validation> = mailerlite(&server)
        .email()
        .validate_block(&block)
        .await
        .unwrap();

    mock.assert_async().await;
    assert!(response.valid);
    assert!(response.errors.is_empty());
    assert_eq!(response.schema.version.as_deref(), Some("2.73.2"));
    assert_eq!(response.schema.hash.as_deref(), Some("0f3a9c"));
    assert!(response.rejected.is_none());
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
        ..Default::default()
    };
    let response: ApiResponse<Validation> = mailerlite(&server)
        .email()
        .validate_block(&block)
        .await
        .unwrap();

    mock.assert_async().await;
    assert!(!response.valid);
    assert_eq!(
        response.errors["type"],
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
        ..Default::default()
    };
    let response: ApiResponse<Validation> = mailerlite(&server)
        .email()
        .validate_block(&block)
        .await
        .unwrap();

    assert!(response.valid);
    assert!(response.errors.is_empty());
}

#[tokio::test]
async fn validate_block_checks_the_placement_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;

    let block: NewBlock = new_paragraph(Some(Placement::after("1/2")));
    assert!(matches!(
        mailerlite(&server).email().validate_block(&block).await,
        Err(Error::InvalidId {
            name: "placement.block_id",
            ..
        })
    ));

    assert_no_request(&mocks).await;
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

    let response: ApiResponse<SettingsResult> = mailerlite(&server)
        .email()
        .update_settings("123", "46", &settings_update())
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.etag, "47");
    let brand = response.data.brand.as_ref().unwrap();
    assert_eq!(brand.mode.as_deref(), Some("custom"));
    assert_eq!(brand.background_color.as_deref(), Some("#f4f4f4"));
    let rejected = response.rejected.as_ref().unwrap();
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
        .email()
        .update_settings("123", "46", &EmailSettingsUpdate::default())
        .await
        .unwrap();

    mock.assert_async().await;
}

#[tokio::test]
async fn update_settings_checks_the_input_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let email = mailerlite(&server).email();

    let mut with_etag: EmailSettingsUpdate = settings_update();
    with_etag.extra.insert("etag".to_string(), json!("1"));
    assert!(matches!(
        email.update_settings("123", "46", &with_etag).await,
        Err(Error::EtagInSettings)
    ));
    assert!(matches!(
        email.update_settings("123", "", &settings_update()).await,
        Err(Error::EmptyEtag)
    ));
    assert!(matches!(
        email.update_settings("", "46", &settings_update()).await,
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.update_settings(id, "46", &settings_update()).await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
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

    let response: ApiResponse<Validation> = mailerlite(&server)
        .email()
        .validate_settings("123", &settings_update())
        .await
        .unwrap();

    mock.assert_async().await;
    assert!(!response.valid);
    assert_eq!(
        response.errors["layout.width"],
        vec!["The layout.width field cannot be changed."]
    );
    assert_eq!(response.schema.version.as_deref(), Some("2.73.2"));
    assert_eq!(
        response.rejected.as_ref().unwrap().keys,
        vec!["footer.social_links[0].url"]
    );
}

#[tokio::test]
async fn validate_settings_checks_the_input_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let email = mailerlite(&server).email();

    let mut with_etag: EmailSettingsUpdate = settings_update();
    with_etag.extra.insert("etag".to_string(), json!("1"));
    assert!(matches!(
        email.validate_settings("123", &with_etag).await,
        Err(Error::EtagInSettings)
    ));
    assert!(matches!(
        email.validate_settings("", &settings_update()).await,
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.validate_settings(id, &settings_update()).await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
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

    let response: ApiResponse<DataOnly<EmailTemplate>> = mailerlite(&server)
        .email()
        .create_template("123", "Newsletter")
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.status_code, 201);
    assert_eq!(response.data.id, "789");
    assert_eq!(response.data.name, "Newsletter");
    assert_eq!(response.data.template_type, "email");
    assert_eq!(response.data.created_at, "2026-10-28 12:00:00");
}

#[tokio::test]
async fn create_template_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let email = mailerlite(&server).email();

    assert!(matches!(
        email.create_template("", "Newsletter").await,
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.create_template(id, "Newsletter").await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
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
    let response: ApiResponse<Paginated<EmailVersion>> = mailerlite(&server)
        .email()
        .versions("123", options)
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].id, "902");
    assert_eq!(response.data[0].created_at, "2026-10-28 12:05:00");
    assert!(response.data[0].current);
    assert!(!response.data[1].current);
    assert_eq!(response.links.prev, None);
    assert_eq!(response.links.next, None);
    assert_eq!(response.meta.from, Some(1));
    assert_eq!(response.meta.to, Some(2));
    assert_eq!(response.meta.total, 2);
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
        .email()
        .versions("123", Default::default())
        .await
        .unwrap();

    mock.assert_async().await;
}

#[tokio::test]
async fn versions_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let email = mailerlite(&server).email();

    assert!(matches!(
        email.versions("", Default::default()).await,
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.versions(id, Default::default()).await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
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

        let response: ApiResponse<DataOnly<EmailVersion>> = mailerlite(&server)
            .email()
            .create_version("123")
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(response.status_code.as_u16() as usize, status);
        assert_eq!(response.data.id, "903");
        assert!(response.data.current);
    }
}

#[tokio::test]
async fn create_version_rejects_bad_ids_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let email = mailerlite(&server).email();

    assert!(matches!(
        email.create_version("").await,
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.create_version(id).await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
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

    let response: ApiResponse<WithEtag<EmailDocument>> = mailerlite(&server)
        .email()
        .restore_version("123", "901", "41")
        .await
        .unwrap();

    mock.assert_async().await;
    assert_eq!(response.etag, "42");
    assert_eq!(response.data.id, "123");
    assert_eq!(response.data.blocks.len(), 2);
}

#[tokio::test]
async fn restore_version_checks_the_input_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let email = mailerlite(&server).email();

    assert!(matches!(
        email.restore_version("123", "901", "").await,
        Err(Error::EmptyEtag)
    ));
    assert!(matches!(
        email.restore_version("", "901", "41").await,
        Err(Error::EmptyId {
            name: "email_id",
            ..
        })
    ));
    assert!(matches!(
        email.restore_version("123", "", "41").await,
        Err(Error::EmptyId {
            name: "version_id",
            ..
        })
    ));
    for id in BAD_IDS {
        assert!(matches!(
            email.restore_version(id, "901", "41").await,
            Err(Error::InvalidId {
                name: "email_id",
                ..
            })
        ));
        assert!(matches!(
            email.restore_version("123", id, "41").await,
            Err(Error::InvalidId {
                name: "version_id",
                ..
            })
        ));
    }

    assert_no_request(&mocks).await;
}

/// Sends each of the 8 writes that accept an idempotency key, with valid input.
async fn send_all_writes(email: &Email) -> Vec<Result<(), Error>> {
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

/// Starts one mock for each of the 8 writes. Each mock expects one request with the header matcher.
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
                .with_status(200)
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
    let email: Email = mailerlite(&server).email().with_idempotency_key("key-1");

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
    let email: Email = mailerlite(&server).email();

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
    let email: Email = mailerlite(&server).email().with_idempotency_key("key-1");

    email.find("123").await.unwrap();
    email.html("123").await.unwrap();
    email.find_block("123", "57").await.unwrap();
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
async fn bad_idempotency_keys_stop_the_writes_without_a_request() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;
    let too_long: String = "k".repeat(256);

    for key in [
        "",
        too_long.as_str(),
        "key\n",
        "key\t",
        "clé",
        "key\x7f",
        " key",
        "key ",
    ] {
        let email: Email = mailerlite(&server).email().with_idempotency_key(key);
        for result in send_all_writes(&email).await {
            assert!(
                matches!(result, Err(Error::InvalidIdempotencyKey)),
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
        .email()
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

    let response: ApiResponse<DataOnly<EmailVersion>> = mailerlite(&server)
        .email()
        .with_idempotency_key("key-1")
        .create_version("123")
        .await
        .unwrap();

    assert!(response.is_idempotent_replay());
    assert_eq!(response.status_code, 201);
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
        ..Default::default()
    };
    let response: ApiResponse<Validation> = mailerlite(&server)
        .email()
        .validate_block(&block)
        .await
        .unwrap();

    assert!(!response.valid);
    assert_eq!(
        response.errors["type"],
        vec!["The selected type is invalid."]
    );
    assert_eq!(response.schema.version, None);
    assert_eq!(response.schema.hash, None);
}

#[tokio::test]
async fn an_other_position_with_a_known_value_follows_the_rules_of_that_value() {
    let mut server: ServerGuard = Server::new_async().await;
    let mocks: Vec<Mock> = expect_no_request(&mut server).await;

    let before: Placement = Placement {
        position: Position::Other("before".to_string()),
        block_id: None,
    };
    let result = mailerlite(&server)
        .email()
        .move_block("123", "57", "45", &before)
        .await;
    assert!(
        matches!(
            result,
            Err(Error::EmptyId {
                name: "placement.block_id",
                ..
            })
        ),
        "got {result:?}"
    );
    assert_no_request(&mocks).await;

    let end: Placement = Placement {
        position: Position::Other("end".to_string()),
        block_id: Some("12".to_string()),
    };
    assert_eq!(
        serde_json::to_value(&end).unwrap(),
        json!({"position": "end"})
    );
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
