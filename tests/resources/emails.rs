//! HTTP contracts for simple-editor email operations.
#![allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]

use crate::editor_support::{BAD_IDS, assert_no_request, expect_request, fixture, mailerlite};
use mailerlite_rs::{
    Error, Response,
    models::emails::{
        BlockType, BlockWriteResult, BrandMode, EditorText, EmailBlock, EmailDocument, EmailHtml,
        EmailOwnerType, EmailResponseMetadata, EmailTemplate, EmailTemplateType, EmailType,
        EmailVersion, EtagOnly, Position, Rejected, SettingsResult, SocialLinkType,
        VERSION_CONFLICT, Validation, WithEtag,
    },
    pagination::NumberedPage,
    resources::emails::{
        BrandUpdate, EmailSettingsUpdate, Emails, FooterUpdate, LinkStyleUpdate,
        ListVersionsOptions, NewBlock, ParagraphStyleUpdate, Placement, SocialLinkUpdate,
        TextStyleUpdate, TypographyUpdate,
    },
};
use serde_json::json;
use wiremock::{Mock, MockGuard, MockServer, ResponseTemplate, matchers};

#[tokio::test]
async fn find_gets_the_email() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard =
        expect_request(&server, "GET", "/emails/123", None, 200, &fixture("email")).await;

    let response: Response<WithEtag<EmailDocument>, EmailResponseMetadata> =
        mailerlite(&server).emails().get("123").await.unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(response.metadata.http.status, 200);
    assert_eq!(response.data.etag, "42");

    let email: &EmailDocument = &response.data.data;
    assert_eq!(email.id, "123");
    assert_eq!(email.email_type, EmailType::SimpleEditor);
    assert_eq!(email.emailable_type, EmailOwnerType::Campaign);
    assert_eq!(email.emailable_id, "456");

    let brand = email.settings.brand.as_ref().unwrap();
    assert_eq!(brand.mode, Some(BrandMode::Account));
    assert_eq!(brand.background_color.as_deref(), Some("#ffffff"));
    let layout = email.settings.layout.as_ref().unwrap();
    assert_eq!(
        layout.width_mode.as_ref().map(|value| value.as_str()),
        Some("default")
    );
    let typography = email.settings.typography.as_ref().unwrap();
    assert_eq!(
        typography.link.as_ref().unwrap().color.as_deref(),
        Some("#2cb191")
    );
    let heading = typography.heading.as_ref().unwrap();
    assert_eq!(
        heading.font_weight.as_ref().map(|value| value.as_str()),
        Some("700")
    );
    assert_eq!(heading.h1.as_ref().unwrap().size.as_deref(), Some("32"));
    let paragraph = typography.paragraph.as_ref().unwrap();
    assert_eq!(paragraph.p1.as_ref().unwrap().size.as_deref(), Some("16"));
    let button = email.settings.button.as_ref().unwrap();
    assert_eq!(button.border_radius.as_deref(), Some("4"));
    let footer = email.settings.footer.as_ref().unwrap();
    assert_eq!(footer.show_preferences, Some(false));
    assert_eq!(
        serde_json::to_value(&footer.company_name.as_ref().unwrap().editor_content).unwrap()["content"]
            [0]["text"],
        "Acme"
    );
    assert_eq!(footer.unsubscribe_text.as_deref(), Some("Unsubscribe"));
    let social_link = &footer.social_links.as_ref().unwrap()[0];
    assert_eq!(social_link.url.as_deref(), Some("https://example.com"));
    assert_eq!(social_link.link_type, Some(SocialLinkType::Website));
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
    assert!(
        email.blocks[1]
            .editor_content
            .as_object()
            .unwrap()
            .is_empty()
    );
    assert_eq!(email.blocks[1].children[0].id, "3");
}

#[tokio::test]
async fn find_decodes_null_values_and_keeps_unknown_keys() {
    let server: MockServer = MockServer::start().await;
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
    let _mock = expect_request(&server, "GET", "/emails/123", None, 200, &body).await;

    let response: Response<WithEtag<EmailDocument>, EmailResponseMetadata> =
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
    assert_eq!(block.editor_content, json!([]));
}

#[tokio::test]
async fn find_rejects_bad_ids_without_a_request() {
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn html_gets_the_html() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
        "GET",
        "/emails/123/html",
        None,
        200,
        &fixture("email_html"),
    )
    .await;

    let response: Response<WithEtag<EmailHtml>, EmailResponseMetadata> =
        mailerlite(&server).emails().html("123").await.unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(
        response.data.data.html,
        "<!doctype html><html><body><p>Hello</p></body></html>"
    );
    assert_eq!(response.data.etag, "42");
}

#[tokio::test]
async fn html_gives_an_api_error_for_a_503_with_an_html_body() {
    let server: MockServer = MockServer::start().await;
    let _mock = Mock::given(matchers::method("GET"))
        .and(matchers::path("/emails/123/html"))
        .and(|request: &wiremock::Request| request.url.query().is_none())
        .respond_with(
            ResponseTemplate::new(503)
                .insert_header("Content-Type", "text/html")
                .set_body_string("<html><body>Service Unavailable</body></html>"),
        )
        .expect(1)
        .mount_as_scoped(&server)
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
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn find_block_gets_the_block_and_its_placement() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
        "GET",
        "/emails/123/blocks/57",
        None,
        200,
        &fixture("email_block"),
    )
    .await;

    let response: Response<WithEtag<EmailBlock>, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .get_block("123", "57")
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
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
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

/// A fixture with the `rejected` object that the API adds when it removes or replaces values.
fn rejected_body(base: &str, keys: &[&str]) -> serde_json::Value {
    let mut body: serde_json::Value = serde_json::from_str(&fixture(base)).unwrap();
    body["rejected"] = json!({"message": "Some attributes were rejected.", "keys": keys});
    body
}

fn assert_rejected(rejected: &Option<Rejected>, keys: &[&str]) {
    let rejected: &Rejected = rejected.as_ref().unwrap();
    assert_eq!(rejected.message, "Some attributes were rejected.");
    assert_eq!(rejected.keys, keys);
}

fn new_paragraph(placement: Option<Placement>) -> NewBlock {
    NewBlock {
        block_type: BlockType::Paragraph,
        editor_content: json!({"attrs":{"textAlign":"center"},"content":[{"type":"text","text":"Hi"}]}),
        placement,
    }
}

#[tokio::test]
async fn create_block_sends_the_block_and_the_etag() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
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

    let response: Response<BlockWriteResult, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .create_block("123", "42", &new_paragraph(Some(Placement::after("57"))))
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(response.metadata.http.status, 201);
    assert_eq!(response.data.etag, "43");
    assert_eq!(response.data.data.id, "58");
    let placement = response.data.data.placement.as_ref().unwrap();
    assert_eq!(placement.after.as_deref(), Some("57"));
    assert_eq!(placement.before, None);
    assert!(response.data.rejected.is_none());
}

#[tokio::test]
async fn create_block_gives_the_keys_that_the_api_rejected() {
    let server: MockServer = MockServer::start().await;
    let content: serde_json::Value = json!({
        "attrs": {"textAlign": "center", "onclick": "track()"},
        "content": [{"type": "text", "text": "Hi"}]
    });
    let mock: MockGuard = expect_request(
        &server,
        "POST",
        "/emails/123/blocks",
        Some(json!({"type": "paragraph", "editor_content": content, "etag": "42"})),
        201,
        &rejected_body("email_block_created", &["editor_content.attrs.onclick"]).to_string(),
    )
    .await;

    let response: Response<BlockWriteResult, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .create_block("123", "42", &NewBlock::new(BlockType::Paragraph, content))
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(response.data.etag, "43");
    assert_eq!(response.data.data.id, "58");
    assert_rejected(&response.data.rejected, &["editor_content.attrs.onclick"]);
}

#[tokio::test]
async fn create_block_sends_no_placement_when_there_is_none() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
        "POST",
        "/emails/123/blocks",
        Some(json!({"type": "divider", "editor_content": {}, "etag": "42"})),
        201,
        &fixture("email_block_created"),
    )
    .await;

    let block: NewBlock = NewBlock {
        block_type: BlockType::Divider,
        editor_content: json!({}),
        placement: None,
    };
    mailerlite(&server)
        .emails()
        .create_block("123", "42", &block)
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
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
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
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
async fn placement_combinations_are_validated_by_the_api() {
    let server = MockServer::start().await;
    for (position, block_id) in [
        (Position::Before, None),
        (Position::After, None),
        (Position::Start, Some("57".into())),
        (Position::End, Some("57".into())),
        (Position::Other("before".into()), None),
    ] {
        let placement = Placement { position, block_id };
        let sent = serde_json::to_value(&placement).unwrap();
        let block = new_paragraph(Some(placement.clone()));
        let mut body = serde_json::to_value(&block).unwrap();
        body["etag"] = json!("42");
        let create = expect_request(
            &server,
            "POST",
            "/emails/123/blocks",
            Some(body),
            422,
            &invalid_placement(),
        )
        .await;
        let move_mock = expect_request(
            &server,
            "POST",
            "/emails/123/blocks/57/move",
            Some(json!({"placement":sent,"etag":"42"})),
            422,
            &invalid_placement(),
        )
        .await;
        let validate = expect_request(
            &server,
            "POST",
            "/emails/blocks/validate",
            Some(serde_json::to_value(&block).unwrap()),
            200,
            &fixture("email_block_validation_invalid"),
        )
        .await;
        let client = mailerlite(&server);
        assert!(
            matches!(client.emails().create_block("123", "42", &block).await, Err(Error::Api(e)) if e.status()==422)
        );
        assert!(
            matches!(client.emails().move_block("123", "57", "42", &placement).await, Err(Error::Api(e)) if e.status()==422)
        );
        assert!(
            !client
                .emails()
                .validate_block(&block)
                .await
                .unwrap()
                .data
                .valid
        );
        assert_eq!(create.received_requests().await.len(), 1);
        assert_eq!(move_mock.received_requests().await.len(), 1);
        assert_eq!(validate.received_requests().await.len(), 1);
    }
}

#[tokio::test]
async fn create_block_sends_the_placement_block_id_as_given() {
    let server: MockServer = MockServer::start().await;
    let mut mocks: Vec<MockGuard> = Vec::new();
    for block_id in ["", "1/2"] {
        mocks.push(
            expect_request(
                &server,
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
        assert_eq!(mock.received_requests().await.len(), 1);
    }
}

fn hello_again() -> serde_json::Value {
    json!({"content": [{"type":"text", "text":"Hello again"}]})
}

#[tokio::test]
async fn update_block_patches_the_content() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
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

    let response: Response<BlockWriteResult, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .update_block("123", "57", "43", &hello_again())
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(response.data.etag, "44");
    assert_eq!(
        response.data.data.editor_content["content"][0]["text"],
        "Hello again"
    );
    let placement = response.data.data.placement.as_ref().unwrap();
    assert_eq!(placement.after.as_deref(), Some("56"));
    assert_eq!(placement.before.as_deref(), Some("58"));
    assert!(response.data.rejected.is_none());
}

#[tokio::test]
async fn update_block_gives_the_keys_that_the_api_rejected() {
    let server: MockServer = MockServer::start().await;
    let content: serde_json::Value = json!({
        "mobile": {"onclick": "track()"},
        "content": [{"type": "text", "text": "Hello again"}]
    });
    let mut reply: serde_json::Value =
        rejected_body("email_block_updated", &["editor_content.mobile.onclick"]);
    // The API keeps the emptied `mobile` object and sends it back as `{}`.
    reply["data"]["editor_content"]["mobile"] = json!({});
    let mock: MockGuard = expect_request(
        &server,
        "PATCH",
        "/emails/123/blocks/57",
        Some(json!({"editor_content": content, "etag": "43"})),
        200,
        &reply.to_string(),
    )
    .await;

    let response: Response<BlockWriteResult, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .update_block("123", "57", "43", &content)
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(response.data.etag, "44");
    assert_eq!(response.data.data.id, "57");
    assert_rejected(&response.data.rejected, &["editor_content.mobile.onclick"]);
}

#[tokio::test]
async fn update_block_gives_the_version_conflict() {
    let server: MockServer = MockServer::start().await;
    let response = ResponseTemplate::new(409)
        .insert_header("Content-Type", "application/json")
        .set_body_string(r#"{"message": "The email was changed by another request.", "code": "version_conflict", "etag": "50"}"#);
    let _mock = Mock::given(matchers::method("PATCH"))
        .and(matchers::path("/emails/123/blocks/57"))
        .and(|request: &wiremock::Request| request.url.query().is_none())
        .respond_with(response)
        .expect(1)
        .mount_as_scoped(&server)
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
    let server: MockServer = MockServer::start().await;
    let response = ResponseTemplate::new(422)
        .insert_header("Content-Type", "application/json")
        .set_body_string(r#"{"message": "The editor content is invalid.", "errors": {"editor_content.content": ["The editor content is invalid."]}}"#);
    let _mock = Mock::given(matchers::method("PATCH"))
        .and(matchers::path("/emails/123/blocks/57"))
        .and(|request: &wiremock::Request| request.url.query().is_none())
        .respond_with(response)
        .expect(1)
        .mount_as_scoped(&server)
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
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn delete_block_sends_the_etag_in_the_body() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
        "DELETE",
        "/emails/123/blocks/57",
        Some(json!({"etag": "44"})),
        200,
        &fixture("email_etag"),
    )
    .await;

    let response: Response<EtagOnly, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .delete_block("123", "57", "44")
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(response.metadata.http.status, 200);
    assert_eq!(response.data.etag, "45");
}

#[tokio::test]
async fn delete_block_rejects_bad_ids_without_a_request() {
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn move_block_sends_the_placement_and_the_etag() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
        "POST",
        "/emails/123/blocks/57/move",
        Some(json!({"placement": {"position": "before", "block_id": "1"}, "etag": "45"})),
        200,
        &fixture("email_block_moved"),
    )
    .await;

    let response: Response<WithEtag<EmailBlock>, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .move_block("123", "57", "45", &Placement::before("1"))
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(response.data.etag, "46");
    let placement = response.data.data.placement.as_ref().unwrap();
    assert_eq!(placement.after, None);
    assert_eq!(placement.before.as_deref(), Some("1"));
}

#[tokio::test]
async fn move_block_rejects_bad_ids_without_a_request() {
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn placement_at_start_or_end_omits_the_adjacent_block_id() {
    let server: MockServer = MockServer::start().await;
    let at_end: MockGuard = expect_request(
        &server,
        "POST",
        "/emails/123/blocks/57/move",
        Some(json!({"placement": {"position": "end"}, "etag": "45"})),
        200,
        &fixture("email_block_moved"),
    )
    .await;
    let at_start: MockGuard = expect_request(
        &server,
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
    let client = mailerlite(&server);
    let email = client.emails();

    let end = Placement::end();
    email.move_block("123", "57", "45", &end).await.unwrap();

    let start = Placement::start();
    email
        .create_block("123", "42", &new_paragraph(Some(start)))
        .await
        .unwrap();

    assert_eq!(at_end.received_requests().await.len(), 1);
    assert_eq!(at_start.received_requests().await.len(), 1);
}

#[tokio::test]
async fn move_block_sends_the_placement_as_given_and_gives_the_api_error() {
    let server: MockServer = MockServer::start().await;
    let placements: [(Placement, serde_json::Value); 3] = [
        (
            Placement::after(""),
            json!({"position": "after", "block_id": ""}),
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
        let mock: MockGuard = expect_request(
            &server,
            "POST",
            "/emails/123/blocks/57/move",
            Some(json!({"placement": sent, "etag": "45"})),
            422,
            &invalid_placement(),
        )
        .await;

        let result = email.move_block("123", "57", "45", &placement).await;

        assert_eq!(mock.received_requests().await.len(), 1);
        assert!(
            matches!(&result, Err(Error::Api(error)) if error.status() == 422),
            "expected Error::Api for {placement:?}, got {result:?}"
        );
        drop(mock);
    }
}

#[tokio::test]
async fn placement_with_an_unknown_position_sends_its_block_id() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
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

    assert_eq!(mock.received_requests().await.len(), 1);
}

#[tokio::test]
async fn validate_block_sends_the_block_without_an_etag() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
        "POST",
        "/emails/blocks/validate",
        Some(json!({"type": "divider", "editor_content": {}})),
        200,
        &fixture("email_block_validation_valid"),
    )
    .await;

    let block: NewBlock = NewBlock {
        block_type: BlockType::Divider,
        editor_content: json!({}),
        placement: None,
    };
    let response: Response<Validation, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .validate_block(&block)
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert!(response.data.valid);
    assert!(response.data.errors.is_empty());
    assert_eq!(response.data.schema.version.as_deref(), Some("2.73.2"));
    assert_eq!(response.data.schema.hash.as_deref(), Some("0f3a9c"));
    assert!(response.data.rejected.is_none());
}

#[tokio::test]
async fn validate_block_gives_the_keys_that_the_api_rejected() {
    let server: MockServer = MockServer::start().await;
    let content: serde_json::Value = json!({
        "attrs": {"textAlign": "center", "onclick": "track()"},
        "content": [{"type": "text", "text": "Hi"}]
    });
    let mock: MockGuard = expect_request(
        &server,
        "POST",
        "/emails/blocks/validate",
        Some(json!({"type": "paragraph", "editor_content": content})),
        200,
        &rejected_body(
            "email_block_validation_valid",
            &["editor_content.attrs.onclick"],
        )
        .to_string(),
    )
    .await;

    let response: Response<Validation, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .validate_block(&NewBlock::new(BlockType::Paragraph, content))
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert!(response.data.valid);
    assert_rejected(&response.data.rejected, &["editor_content.attrs.onclick"]);
}

#[tokio::test]
async fn validate_block_gives_the_errors_of_an_invalid_block() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
        "POST",
        "/emails/blocks/validate",
        Some(json!({"type": "countdown", "editor_content": {}})),
        200,
        &fixture("email_block_validation_invalid"),
    )
    .await;

    let block: NewBlock = NewBlock {
        block_type: BlockType::Other("countdown".into()),
        editor_content: json!({}),
        placement: None,
    };
    let response: Response<Validation, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .validate_block(&block)
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert!(!response.data.valid);
    assert_eq!(
        response.data.errors["type"],
        vec!["The selected type is invalid."]
    );
}

#[tokio::test]
async fn validate_block_accepts_an_empty_array_for_the_errors() {
    let server: MockServer = MockServer::start().await;
    let _mock = expect_request(
        &server,
        "POST",
        "/emails/blocks/validate",
        Some(json!({"type": "divider", "editor_content": {}})),
        200,
        r#"{"valid": true, "errors": [], "schema": {"version": "2.73.2", "hash": "0f3a9c"}}"#,
    )
    .await;

    let block: NewBlock = NewBlock {
        block_type: BlockType::Divider,
        editor_content: json!({}),
        placement: None,
    };
    let response: Response<Validation, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .validate_block(&block)
        .await
        .unwrap();

    assert!(response.data.valid);
    assert!(response.data.errors.is_empty());
}

#[tokio::test]
async fn validate_block_sends_a_bad_placement_and_gives_the_errors() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
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
    let response: Response<Validation, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .validate_block(&block)
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert!(!response.data.valid);
}

fn settings_update() -> EmailSettingsUpdate {
    let mut extra: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    extra.insert("new_group".to_string(), json!({"a": "1"}));

    EmailSettingsUpdate {
        brand: Some(BrandUpdate {
            mode: Some(BrandMode::Custom),
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
                link_type: Some(SocialLinkType::Website),
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
    let server: MockServer = MockServer::start().await;
    let mut body: serde_json::Value = settings_update_json();
    body["etag"] = json!("46");
    let mock: MockGuard = expect_request(
        &server,
        "PATCH",
        "/emails/123/settings",
        Some(body),
        200,
        &fixture("email_settings"),
    )
    .await;

    let response: Response<SettingsResult, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .update_settings("123", "46", &settings_update())
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(response.data.etag, "47");
    let brand = response.data.data.brand.as_ref().unwrap();
    assert_eq!(brand.mode, Some(BrandMode::Custom));
    assert_eq!(brand.background_color.as_deref(), Some("#f4f4f4"));
    assert_rejected(&response.data.rejected, &["footer.social_links[0].url"]);
}

#[tokio::test]
async fn update_settings_sends_an_empty_object_for_empty_settings() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
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

    assert_eq!(mock.received_requests().await.len(), 1);
}

#[tokio::test]
async fn update_settings_checks_the_input_without_a_request() {
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn validate_settings_sends_the_settings_without_an_etag() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
        "POST",
        "/emails/123/settings/validate",
        Some(settings_update_json()),
        200,
        &fixture("email_settings_validation"),
    )
    .await;

    let response: Response<Validation, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .validate_settings("123", &settings_update())
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert!(!response.data.valid);
    assert_eq!(
        response.data.errors["layout.width"],
        vec!["The layout.width field cannot be changed."]
    );
    assert_eq!(response.data.schema.version.as_deref(), Some("2.73.2"));
    assert!(response.data.rejected.is_none());
}

#[tokio::test]
async fn validate_settings_gives_the_keys_that_the_api_would_reject() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
        "POST",
        "/emails/123/settings/validate",
        Some(json!({
            "footer": {
                "social_links": [
                    {"url": "javascript:alert(1)", "type": "facebook", "label": "Facebook"}
                ]
            }
        })),
        200,
        &rejected_body(
            "email_block_validation_valid",
            &["footer.social_links[0].url"],
        )
        .to_string(),
    )
    .await;

    let settings: EmailSettingsUpdate = EmailSettingsUpdate {
        footer: Some(FooterUpdate {
            social_links: Some(vec![SocialLinkUpdate::new(
                "javascript:alert(1)",
                SocialLinkType::Facebook,
                "Facebook",
            )]),
            ..Default::default()
        }),
        ..Default::default()
    };
    let response: Response<Validation, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .validate_settings("123", &settings)
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert!(response.data.valid);
    assert!(response.data.errors.is_empty());
    assert_rejected(&response.data.rejected, &["footer.social_links[0].url"]);
}

#[tokio::test]
async fn validate_settings_sends_an_etag_key_of_the_extra_map() {
    let server: MockServer = MockServer::start().await;
    let mut sent: serde_json::Value = settings_update_json();
    sent["etag"] = json!("1");
    let mock: MockGuard = expect_request(
        &server,
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

    assert_eq!(mock.received_requests().await.len(), 1);
}

#[tokio::test]
async fn validate_settings_rejects_bad_ids_without_a_request() {
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn create_template_sends_the_name() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
        "POST",
        "/emails/123/templates",
        Some(json!({"name": "Newsletter"})),
        201,
        &fixture("email_template"),
    )
    .await;

    let response: Response<EmailTemplate, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .create_template("123", "Newsletter")
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(response.metadata.http.status, 201);
    assert_eq!(response.data.id, "789");
    assert_eq!(response.data.name, "Newsletter");
    assert_eq!(response.data.template_type, EmailTemplateType::Email);
    assert_eq!(response.data.created_at, "2026-10-28 12:00:00");
}

#[tokio::test]
async fn create_template_rejects_bad_ids_without_a_request() {
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn versions_lists_the_versions_with_the_page_options() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
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
    let response: Response<NumberedPage<EmailVersion>, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .versions("123", options)
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
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
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
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

    assert_eq!(mock.received_requests().await.len(), 1);
}

#[tokio::test]
async fn versions_rejects_bad_ids_without_a_request() {
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn create_version_sends_no_body_and_gives_the_status_code() {
    for status in [201, 200] {
        let server: MockServer = MockServer::start().await;
        let mock: MockGuard = expect_request(
            &server,
            "POST",
            "/emails/123/versions",
            None,
            status,
            &fixture("email_version"),
        )
        .await;

        let response: Response<EmailVersion, EmailResponseMetadata> = mailerlite(&server)
            .emails()
            .create_version("123")
            .await
            .unwrap();

        assert_eq!(mock.received_requests().await.len(), 1);
        assert_eq!(response.metadata.http.status.as_u16(), status);
        assert_eq!(response.data.id, "903");
        assert!(response.data.current);
    }
}

#[tokio::test]
async fn create_version_rejects_bad_ids_without_a_request() {
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn restore_version_sends_the_etag_and_gives_the_email() {
    let server: MockServer = MockServer::start().await;
    let mock: MockGuard = expect_request(
        &server,
        "POST",
        "/emails/123/versions/901/restore",
        Some(json!({"etag": "41"})),
        200,
        &fixture("email"),
    )
    .await;

    let response: Response<WithEtag<EmailDocument>, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .restore_version("123", "901", "41")
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
    assert_eq!(response.data.etag, "42");
    assert_eq!(response.data.data.id, "123");
    assert_eq!(response.data.data.blocks.len(), 2);
}

#[tokio::test]
async fn restore_version_rejects_bad_ids_without_a_request() {
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn the_6_writes_with_an_etag_send_an_empty_etag_and_give_the_api_error() {
    let server: MockServer = MockServer::start().await;
    let writes: [(&str, &str); 6] = [
        ("POST", "/emails/123/blocks"),
        ("PATCH", "/emails/123/blocks/57"),
        ("DELETE", "/emails/123/blocks/57"),
        ("POST", "/emails/123/blocks/57/move"),
        ("PATCH", "/emails/123/settings"),
        ("POST", "/emails/123/versions/901/restore"),
    ];
    let mut mocks: Vec<MockGuard> = Vec::new();
    let response = ResponseTemplate::new(422).set_body_json(json!({
        "message": "The etag field is required.",
        "errors": {"etag": ["The etag field is required."]}
    }));
    for (method, path) in writes {
        let mock = Mock::given(matchers::method(method))
            .and(matchers::path(path))
            .and(|request: &wiremock::Request| request.url.query().is_none())
            .and(matchers::body_partial_json(json!({"etag": ""})))
            .respond_with(response.clone())
            .expect(1)
            .mount_as_scoped(&server)
            .await;
        mocks.push(mock);
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
        assert_eq!(mock.received_requests().await.len(), 1);
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

async fn mock_all_writes(server: &MockServer, key: Option<&str>) -> Vec<MockGuard> {
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
    let mut mocks: Vec<MockGuard> = Vec::new();
    for (method, path, response) in writes {
        mocks.push(
            Mock::given(matchers::method(method))
                .and(matchers::path(path))
                .and(|request: &wiremock::Request| request.url.query().is_none())
                .and({
                    let expected = key.map(str::to_owned);
                    move |request: &wiremock::Request| {
                        request
                            .headers
                            .get("Idempotency-Key")
                            .and_then(|value| value.to_str().ok())
                            == expected.as_deref()
                    }
                })
                .respond_with(
                    ResponseTemplate::new(
                        if path.ends_with("/blocks") || path.ends_with("/templates") {
                            201
                        } else {
                            200
                        },
                    )
                    .set_body_string(fixture(response)),
                )
                .expect(1)
                .mount_as_scoped(server)
                .await,
        );
    }

    mocks
}

#[tokio::test]
async fn the_8_writes_send_the_idempotency_key() {
    let server: MockServer = MockServer::start().await;
    let mocks: Vec<MockGuard> = mock_all_writes(&server, Some("key-1")).await;
    let client = mailerlite(&server);
    let email: Emails = client.emails().with_idempotency_key("key-1");

    for result in send_all_writes(&email).await {
        result.unwrap();
    }

    for mock in mocks {
        assert_eq!(mock.received_requests().await.len(), 1);
    }
}

#[tokio::test]
async fn the_8_writes_send_no_idempotency_key_without_a_key() {
    let server: MockServer = MockServer::start().await;
    let mocks: Vec<MockGuard> = mock_all_writes(&server, None).await;
    let client = mailerlite(&server);
    let email: Emails = client.emails();

    for result in send_all_writes(&email).await {
        result.unwrap();
    }

    for mock in mocks {
        assert_eq!(mock.received_requests().await.len(), 1);
    }
}

#[tokio::test]
async fn reads_and_validations_ignore_the_idempotency_key() {
    let server: MockServer = MockServer::start().await;
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
    let mut mocks: Vec<MockGuard> = Vec::new();
    for (method, path, response) in reads {
        mocks.push(
            Mock::given(matchers::method(method))
                .and(matchers::path(path))
                .and(|request: &wiremock::Request| request.url.query().is_none())
                .and(|request: &wiremock::Request| !request.headers.contains_key("Idempotency-Key"))
                .respond_with(ResponseTemplate::new(200).set_body_string(fixture(response)))
                .expect(1)
                .mount_as_scoped(&server)
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
        assert_eq!(mock.received_requests().await.len(), 1);
    }
}

#[tokio::test]
async fn the_8_writes_send_the_idempotency_key_as_given() {
    let server: MockServer = MockServer::start().await;
    let too_long: String = "k".repeat(256);

    // HTTP removes a space at the start or the end of a header value, so the API gets "key".
    for (key, received) in [
        ("", ""),
        (too_long.as_str(), too_long.as_str()),
        ("key\tkey", "key\tkey"),
        (" key", "key"),
        ("key ", "key"),
    ] {
        let mocks: Vec<MockGuard> = mock_all_writes(&server, Some(received)).await;
        let client = mailerlite(&server);
        let email: Emails = client.emails().with_idempotency_key(key);

        for result in send_all_writes(&email).await {
            assert!(
                result.is_ok(),
                "expected a request for {key:?}, got {result:?}"
            );
        }
        for mock in mocks {
            assert_eq!(mock.received_requests().await.len(), 1);
            drop(mock);
        }
    }
}

#[tokio::test]
async fn a_key_that_http_cannot_carry_stops_the_writes_without_a_request() {
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn a_255_character_key_is_sent() {
    let server: MockServer = MockServer::start().await;
    let key: String = "k".repeat(255);
    let mock: MockGuard = Mock::given(matchers::method("POST"))
        .and(matchers::path("/emails/123/versions"))
        .and(|request: &wiremock::Request| request.url.query().is_none())
        .and(matchers::header("Idempotency-Key", key.as_str()))
        .respond_with(ResponseTemplate::new(201).set_body_string(fixture("email_version")))
        .expect(1)
        .mount_as_scoped(&server)
        .await;

    mailerlite(&server)
        .emails()
        .with_idempotency_key(key.clone())
        .create_version("123")
        .await
        .unwrap();

    assert_eq!(mock.received_requests().await.len(), 1);
}

#[tokio::test]
async fn a_replayed_write_has_the_replay_flag() {
    let server: MockServer = MockServer::start().await;
    let _mock = Mock::given(matchers::method("POST"))
        .and(matchers::path("/emails/123/versions"))
        .and(|request: &wiremock::Request| request.url.query().is_none())
        .and(matchers::header("Idempotency-Key", "key-1"))
        .respond_with(
            ResponseTemplate::new(201)
                .insert_header("Idempotent-Replayed", "True")
                .set_body_string(fixture("email_version")),
        )
        .expect(1)
        .mount_as_scoped(&server)
        .await;

    let response: Response<EmailVersion, EmailResponseMetadata> = mailerlite(&server)
        .emails()
        .with_idempotency_key("key-1")
        .create_version("123")
        .await
        .unwrap();

    assert!(response.metadata.is_idempotent_replay());
    assert_eq!(response.metadata.http.status, 201);
}

#[tokio::test]
async fn validate_block_decodes_a_null_schema() {
    let server: MockServer = MockServer::start().await;
    let _mock = expect_request(
        &server,
        "POST",
        "/emails/blocks/validate",
        Some(json!({"type": "divider", "editor_content": {}})),
        200,
        r#"{"valid": false, "errors": {"type": ["The selected type is invalid."]}, "schema": {"version": null, "hash": null}}"#,
    )
    .await;

    let block: NewBlock = NewBlock {
        block_type: BlockType::Divider,
        editor_content: json!({}),
        placement: None,
    };
    let response: Response<Validation, EmailResponseMetadata> = mailerlite(&server)
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

#[tokio::test]
async fn settings_with_an_extra_key_that_is_also_a_field_send_no_request() {
    let server: MockServer = MockServer::start().await;

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

    assert_no_request(&server).await;
}

#[tokio::test]
async fn update_block_by_id_leaves_stored_kind_validation_to_the_server() {
    let server = MockServer::start().await;
    let get = Mock::given(matchers::method("GET"))
        .and(matchers::path("/emails/123/blocks/57"))
        .and(|request: &wiremock::Request| request.url.query().is_none())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount_as_scoped(&server)
        .await;
    let patch = expect_request(
        &server, "PATCH", "/emails/123/blocks/57",
        Some(json!({"editor_content":{"content":[{"type":"text","text":"Wrong kind"}]},"etag":"43"})),
        422,
        r#"{"message":"Invalid content for the stored block type.","errors":{"editor_content":["Invalid content for the stored block type."]}}"#,
    )
        .await;
    let error = mailerlite(&server)
        .emails()
        .update_block(
            "123",
            "57",
            "43",
            &json!({"content":[{"type":"text","text":"Wrong kind"}]}),
        )
        .await
        .unwrap_err();
    assert!(matches!(&error, Error::Api(error) if error.status().as_u16() == 422));
    assert!(get.received_requests().await.is_empty());
    assert_eq!(patch.received_requests().await.len(), 1);
}

#[tokio::test]
async fn placeholder_images_decode_on_reads_and_writes() {
    let server = MockServer::start().await;
    let image = json!({"id":"57","type":"image","editor_content":{
        "attrs":{"src":null,"url":null,"alt":null,"caption":null,"linkType":null,"width":300.5,"height":150.25},
        "content":null,"mobile":null,"visibility":null
    }});
    let body = json!({"data":image,"etag":"43"}).to_string();
    let get = expect_request(&server, "GET", "/emails/123/blocks/57", None, 200, &body).await;
    let create = expect_request(
        &server,
        "POST",
        "/emails/123/blocks",
        Some(json!({"type":"image","editor_content":{},"etag":"42"})),
        201,
        &body,
    )
    .await;
    let update = expect_request(
        &server,
        "PATCH",
        "/emails/123/blocks/57",
        Some(json!({"editor_content":image["editor_content"],"etag":"43"})),
        200,
        &body,
    )
    .await;
    let client = mailerlite(&server);
    let fetched = client.emails().get_block("123", "57").await.unwrap();
    let block = NewBlock::new(BlockType::Image, json!({}));
    let created = client
        .emails()
        .create_block("123", "42", &block)
        .await
        .unwrap();
    let updated = client
        .emails()
        .update_block(
            "123",
            &fetched.data.data.id,
            &fetched.data.etag,
            &fetched.data.data.editor_content,
        )
        .await
        .unwrap();
    for block in [fetched.data.data, created.data.data, updated.data.data] {
        assert_eq!(block.block_type, BlockType::Image);
        assert_eq!(block.editor_content, image["editor_content"]);
    }
    assert_eq!(get.received_requests().await.len(), 1);
    assert_eq!(create.received_requests().await.len(), 1);
    assert_eq!(update.received_requests().await.len(), 1);

    let mut document: serde_json::Value = serde_json::from_str(&fixture("email")).unwrap();
    document["data"]["blocks"] = json!([
        {"id":"1","type":"paragraph","editor_content":{"attrs":null,"content":null,"mobile":null,"visibility":null}},
        {"id":"2","type":"bulleted_list","editor_content":{},"children":[image]},
        {"id":"3","type":"paragraph","editor_content":null,"children":null}
    ]);
    let get = expect_request(
        &server,
        "GET",
        "/emails/123",
        None,
        200,
        &document.to_string(),
    )
    .await;
    let response = client.emails().get("123").await.unwrap();
    assert_eq!(response.data.data.blocks[1].children[0].id, "57");
    assert_eq!(
        response.data.data.blocks[2].editor_content,
        serde_json::Value::Null
    );
    assert!(response.data.data.blocks[2].children.is_empty());
    assert_eq!(get.received_requests().await.len(), 1);
}

#[tokio::test]
async fn stored_string_widths_and_footer_documents_survive_email_reads() {
    let server = MockServer::start().await;
    let mut document: serde_json::Value = serde_json::from_str(&fixture("email")).unwrap();
    document["data"]["blocks"] = json!([
        {"id":"1","type":"paragraph","editor_content":{}},
        {"id":"2","type":"bulleted_list","editor_content":{},"children":[
            {"id":"bad-image-57","type":"image","editor_content":{"attrs":{"width":"600","future":{"schema":99}}}}
        ]}
    ]);
    document["data"]["settings"]["footer"]["info_text"] =
        json!({"editor_content":{"content":[{"type":"future_node","text":42}]}});
    let mock = Mock::given(matchers::method("GET"))
        .and(matchers::path("/emails/123"))
        .and(|request: &wiremock::Request| request.url.query().is_none())
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .insert_header("x-request-id", "decode-review")
                .set_body_string(document.to_string()),
        )
        .expect(1)
        .mount_as_scoped(&server)
        .await;
    let response = mailerlite(&server).emails().get("123").await.unwrap();
    assert_eq!(
        response.data.data.blocks[1].children[0].editor_content,
        document["data"]["blocks"][1]["children"][0]["editor_content"]
    );
    assert_eq!(
        response
            .data
            .data
            .settings
            .footer
            .unwrap()
            .info_text
            .unwrap()
            .editor_content,
        document["data"]["settings"]["footer"]["info_text"]["editor_content"]
    );
    assert_eq!(mock.received_requests().await.len(), 1);
}

#[tokio::test]
async fn stored_inline_nodes_and_marks_are_not_validated_on_reads() {
    let server = MockServer::start().await;
    let body = json!({"data":{"id":"57","type":"paragraph","editor_content":{"content":[
        {"type":"text","text":"Hello","marks":[{"type":"link","attrs":{"href":false}}]}
    ]}},"etag":"43"});
    let mock = expect_request(
        &server,
        "GET",
        "/emails/123/blocks/57",
        None,
        200,
        &body.to_string(),
    )
    .await;
    let response = mailerlite(&server)
        .emails()
        .get_block("123", "57")
        .await
        .unwrap();
    assert_eq!(
        response.data.data.editor_content,
        body["data"]["editor_content"]
    );
    assert_eq!(mock.received_requests().await.len(), 1);
}

#[tokio::test]
async fn update_block_accepts_unknown_content_by_id() {
    let server = MockServer::start().await;
    let content = json!({"value":2});
    let mock = expect_request(
        &server,
        "PATCH",
        "/emails/123/blocks/57",
        Some(json!({"editor_content":{"value":2},"etag":"43"})),
        200,
        &json!({"data":{"id":"57","type":"future_block","editor_content":{"value":2}},"etag":"44"})
            .to_string(),
    )
    .await;
    let response = mailerlite(&server)
        .emails()
        .update_block("123", "57", "43", &content)
        .await
        .unwrap();
    assert_eq!(response.data.data.editor_content, content);
    assert_eq!(mock.received_requests().await.len(), 1);
}

#[tokio::test]
async fn editor_documents_pass_unchanged_through_create_validate_and_update() {
    let server = MockServer::start().await;
    let client = mailerlite(&server);
    for content in [
        json!({"attrs":{"width":"600","future":{"schema":99}},"content":[{"type":"future_node","marks":[{"type":"link","attrs":{"href":false}}]}]}),
        serde_json::Value::Null,
        json!([]),
    ] {
        let block = NewBlock::new(BlockType::Image, content.clone());
        let response =
            json!({"data":{"id":"57","type":"image","editor_content":content},"etag":"43"})
                .to_string();
        let create = expect_request(
            &server,
            "POST",
            "/emails/123/blocks",
            Some(json!({"type":"image","editor_content":content,"etag":"42"})),
            201,
            &response,
        )
        .await;
        let validation = expect_request(
            &server,
            "POST",
            "/emails/blocks/validate",
            Some(json!({"type":"image","editor_content":content})),
            200,
            &fixture("email_block_validation_invalid"),
        )
        .await;
        let update = expect_request(
            &server,
            "PATCH",
            "/emails/123/blocks/57",
            Some(json!({"editor_content":content,"etag":"43"})),
            200,
            &response,
        )
        .await;
        assert_eq!(
            client
                .emails()
                .create_block("123", "42", &block)
                .await
                .unwrap()
                .data
                .data
                .editor_content,
            content
        );
        assert!(
            !client
                .emails()
                .validate_block(&block)
                .await
                .unwrap()
                .data
                .valid
        );
        assert_eq!(
            client
                .emails()
                .update_block("123", "57", "43", &content)
                .await
                .unwrap()
                .data
                .data
                .editor_content,
            content
        );
        assert_eq!(create.received_requests().await.len(), 1);
        assert_eq!(validation.received_requests().await.len(), 1);
        assert_eq!(update.received_requests().await.len(), 1);
    }
}

#[tokio::test]
async fn footer_editor_documents_pass_unchanged_to_settings_endpoints() {
    let server = MockServer::start().await;
    let content =
        json!({"attrs":{"future":[null,42]},"content":[{"type":"future_node","text":false}]});
    let settings = EmailSettingsUpdate {
        footer: Some(FooterUpdate {
            info_text: Some(EditorText {
                editor_content: content.clone(),
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    let validation = expect_request(
        &server,
        "POST",
        "/emails/123/settings/validate",
        Some(json!({"footer":{"info_text":{"editor_content":content}}})),
        200,
        &fixture("email_block_validation_invalid"),
    )
    .await;
    let update = expect_request(
        &server,
        "PATCH",
        "/emails/123/settings",
        Some(json!({"footer":{"info_text":{"editor_content":content}},"etag":"42"})),
        422,
        r#"{"message":"Invalid document","errors":{"footer.info_text":["Invalid document"]}}"#,
    )
    .await;
    let client = mailerlite(&server);
    assert!(
        !client
            .emails()
            .validate_settings("123", &settings)
            .await
            .unwrap()
            .data
            .valid
    );
    assert!(
        matches!(client.emails().update_settings("123", "42", &settings).await, Err(Error::Api(e)) if e.status()==422)
    );
    assert_eq!(validation.received_requests().await.len(), 1);
    assert_eq!(update.received_requests().await.len(), 1);
}
