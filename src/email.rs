//! Emails built with the simple editor: blocks, design settings, templates and versions.
//!
//! To make such an email, create a campaign with `editor` set to `campaign::EDITOR_SIMPLE`.
//! Each write needs the etag of the last response for the email, and returns a new etag.

use crate::{
    api_response::{object_or_empty_array, ApiResponse, DataOnly, EtagOnly, WithEtag},
    error::Error,
    http::{self, Method, NO_BODY},
    pagination::Paginated,
    validate, MailerLite,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

const END_POINT: &str = "emails";

/// The `code` of a `409` error when the email changed after you read the etag.
/// Then `ApiError::etag` has the current etag.
pub const VERSION_CONFLICT: &str = "version_conflict";

open_enum! {
    /// The type of a block.
    pub enum BlockType {
        Blockquote => "blockquote",
        BulletedList => "bulleted_list",
        Button => "button",
        Code => "code",
        Divider => "divider",
        Heading => "heading",
        Image => "image",
        NumberedList => "numbered_list",
        Paragraph => "paragraph",
        SocialIcon => "social_icon",
        /// The title of the email. You cannot insert a title block.
        Title => "title",
        Video => "video",
    }
}

open_enum! {
    /// The position of a block in a placement.
    pub enum Position {
        Start => "start",
        End => "end",
        /// Before the block with the `block_id` of the placement.
        Before => "before",
        /// After the block with the `block_id` of the placement.
        After => "after",
    }
}

open_enum! {
    /// The source of the brand settings of an email.
    pub enum BrandMode {
        Account => "account",
        Template => "template",
        Custom => "custom",
    }
}

/// An email built with the simple editor.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EmailDocument {
    pub id: String,
    /// The type of the email, for example `"builder_inline_email"`.
    #[serde(rename = "type")]
    pub email_type: String,
    pub emailable_type: String,
    pub emailable_id: String,
    pub settings: EmailSettings,
    #[serde(default)]
    pub blocks: Vec<EmailBlock>,
}

/// A content block of an email.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EmailBlock {
    pub id: String,
    #[serde(rename = "type")]
    pub block_type: BlockType,
    /// The editor document of the block. Its keys are camelCase, for example `textAlign`.
    #[serde(default, deserialize_with = "object_or_empty_array")]
    pub editor_content: Map<String, Value>,
    /// The blocks in this block, for example the items of a list.
    #[serde(default)]
    pub children: Vec<EmailBlock>,
    /// The neighbors of the block. Only the responses for one block have it.
    #[serde(default)]
    pub placement: Option<BlockPlacement>,
}

/// The IDs of the blocks after which and before which a block is. `None` when there is no such block.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct BlockPlacement {
    pub after: Option<String>,
    pub before: Option<String>,
}

/// The design settings of an email, as the API returns them.
///
/// A value is `None` when the API sent `null` or no key. Each `extra` map keeps the keys that this version
/// of the SDK does not know. To change the settings, use `EmailSettingsUpdate`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EmailSettings {
    pub brand: Option<Brand>,
    pub layout: Option<Layout>,
    pub typography: Option<Typography>,
    pub button: Option<Button>,
    pub footer: Option<Footer>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// The brand settings. `mode` is one of the `BrandMode` values.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Brand {
    pub mode: Option<String>,
    pub font_family: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub border_color: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// The layout settings.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Layout {
    pub width_mode: Option<String>,
    pub alignment: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// The typography settings.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Typography {
    pub link: Option<LinkStyle>,
    pub heading: Option<HeadingStyle>,
    pub paragraph: Option<ParagraphStyle>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// The style of links.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct LinkStyle {
    pub color: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// The style of headings.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct HeadingStyle {
    pub font_family: Option<String>,
    pub font_weight: Option<String>,
    pub line_height: Option<String>,
    pub letter_spacing: Option<String>,
    pub h1: Option<TextStyle>,
    pub h2: Option<TextStyle>,
    pub h3: Option<TextStyle>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// The style of paragraphs.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ParagraphStyle {
    pub font_family: Option<String>,
    pub font_weight: Option<String>,
    pub line_height: Option<String>,
    pub letter_spacing: Option<String>,
    pub p1: Option<TextStyle>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// The color and the size of a text level.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct TextStyle {
    pub color: Option<String>,
    pub size: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// The button settings.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Button {
    pub font_family: Option<String>,
    pub font_weight: Option<String>,
    pub letter_spacing: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub border_radius: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// The footer settings.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Footer {
    pub show_preferences: Option<bool>,
    pub company_name: Option<EditorText>,
    pub address: Option<EditorText>,
    pub info_text: Option<EditorText>,
    pub unsubscribe_text: Option<String>,
    pub preferences_text: Option<String>,
    pub social_links: Option<Vec<SocialLink>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A footer text as an editor document. In an update, an empty `editor_content` unsets the text.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EditorText {
    #[serde(default, deserialize_with = "object_or_empty_array")]
    pub editor_content: Map<String, Value>,
}

/// A social link of the footer. `link_id` keeps the click history of the link.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct SocialLink {
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub link_type: Option<String>,
    pub label: Option<String>,
    pub link_id: Option<u64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// The HTML of an email.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EmailHtml {
    pub html: String,
}

/// The default block type is `Paragraph`.
impl Default for BlockType {
    fn default() -> Self {
        BlockType::Paragraph
    }
}

/// The default position is `End`, as in the API.
impl Default for Position {
    fn default() -> Self {
        Position::End
    }
}

/// A new block for `Email::create_block` and `Email::validate_block`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct NewBlock {
    #[serde(rename = "type")]
    pub block_type: BlockType,
    /// The editor document of the block. Its keys are camelCase. Use an empty map for a block with no
    /// content, for example a divider.
    pub editor_content: Map<String, Value>,
    /// Where to put the block. Without a placement, the API puts the block at the end of the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
}

/// Where to put a block. `block_id` is necessary for the positions `Before` and `After`.
///
/// For `Start` and `End`, the API ignores `block_id`, so the SDK does not check it and does not send it.
#[derive(Debug, Clone, Default)]
pub struct Placement {
    pub position: Position,
    pub block_id: Option<String>,
}

impl Serialize for Placement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            position: &'a Position,
            #[serde(skip_serializing_if = "Option::is_none")]
            block_id: Option<&'a str>,
        }

        let block_id: Option<&str> = match self.position {
            Position::Start | Position::End => None,
            _ => self.block_id.as_deref(),
        };

        Body {
            position: &self.position,
            block_id,
        }
        .serialize(serializer)
    }
}

impl Placement {
    /// At the start of the email.
    pub fn start() -> Self {
        Self {
            position: Position::Start,
            block_id: None,
        }
    }

    /// At the end of the email.
    pub fn end() -> Self {
        Self {
            position: Position::End,
            block_id: None,
        }
    }

    /// Before the block with the ID `block_id`.
    pub fn before(block_id: impl Into<String>) -> Self {
        Self {
            position: Position::Before,
            block_id: Some(block_id.into()),
        }
    }

    /// After the block with the ID `block_id`.
    pub fn after(block_id: impl Into<String>) -> Self {
        Self {
            position: Position::After,
            block_id: Some(block_id.into()),
        }
    }

    /// Checks `block_id` for `Before` and `After`, where it is necessary. For an unknown position, the SDK
    /// checks `block_id` only when it is set.
    fn check(&self) -> Result<(), Error> {
        match (&self.position, self.block_id.as_deref()) {
            (Position::Start | Position::End, _) => Ok(()),
            (Position::Before | Position::After, block_id) => {
                validate::path_id("placement.block_id", block_id.unwrap_or_default())
            }
            (Position::Other(_), Some(block_id)) => {
                validate::path_id("placement.block_id", block_id)
            }
            (Position::Other(_), None) => Ok(()),
        }
    }
}

/// A request body with the etag of the email.
#[derive(Serialize)]
struct EtagBody<'a, T: Serialize> {
    #[serde(flatten)]
    body: &'a T,
    etag: &'a str,
}

/// The request body of `Email::update_block`.
#[derive(Serialize)]
struct EditorContentBody<'a> {
    editor_content: &'a Map<String, Value>,
    etag: &'a str,
}

/// The request body of `Email::move_block`.
#[derive(Serialize)]
struct MoveBody<'a> {
    placement: &'a Placement,
    etag: &'a str,
}

/// The result of a validation. An invalid input is not an error: `valid` is `false`, and `errors` has
/// the messages for each field.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Validation {
    pub valid: bool,
    /// The messages for each field path. Empty when the input is valid.
    #[serde(default, deserialize_with = "object_or_empty_array")]
    pub errors: BTreeMap<String, Vec<String>>,
    pub schema: ValidationSchema,
    /// The keys of the unsafe values that the API removed. Only the settings validation has it.
    #[serde(default)]
    pub rejected: Option<Rejected>,
}

/// The version of the rules that the validation used.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ValidationSchema {
    pub version: String,
    pub hash: String,
}

/// The unsafe values that the API removed or replaced.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Rejected {
    pub message: String,
    /// The paths of the keys, for example `footer.social_links[0].url`.
    #[serde(default)]
    pub keys: Vec<String>,
}

/// A change to the design settings of an email, for `Email::update_settings` and `Email::validate_settings`.
///
/// Each value has three states:
/// - `None`: the SDK does not send the key, and the value does not change;
/// - `Some(value)`: the API stores the value;
/// - `Some(String::new())`: the API removes the value, and the email uses the value of the account again.
///
/// Set only the values to change. The API stores each value that you send, also a value that is the same as
/// the value of the account. Then the email no longer follows the account for that value. For this reason,
/// the SDK has no conversion from `EmailSettings`. Send all style values as strings, for example `"16"`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EmailSettingsUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<BrandUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<LayoutUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typography: Option<TypographyUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<ButtonUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<FooterUpdate>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A change to the brand settings.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BrandUpdate {
    /// One of the `BrandMode` values, for example `Some(BrandMode::Custom.into())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// The `identifier` of a palette from `BrandStyle::get`. The API applies the palette. The API does not
    /// return this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A change to the layout settings.
#[derive(Debug, Clone, Default, Serialize)]
pub struct LayoutUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A change to the typography settings.
#[derive(Debug, Clone, Default, Serialize)]
pub struct TypographyUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<LinkStyleUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<HeadingStyleUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph: Option<ParagraphStyleUpdate>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A change to the style of links.
#[derive(Debug, Clone, Default, Serialize)]
pub struct LinkStyleUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A change to the style of headings.
#[derive(Debug, Clone, Default, Serialize)]
pub struct HeadingStyleUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h1: Option<TextStyleUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2: Option<TextStyleUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h3: Option<TextStyleUpdate>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A change to the style of paragraphs.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ParagraphStyleUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p1: Option<TextStyleUpdate>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A change to the color and the size of a text level.
#[derive(Debug, Clone, Default, Serialize)]
pub struct TextStyleUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A change to the button settings.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ButtonUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<String>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A change to the footer settings.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FooterUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_preferences: Option<bool>,
    /// An empty `editor_content` removes the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<EditorText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<EditorText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_text: Option<EditorText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferences_text: Option<String>,
    /// The new list replaces the whole list. Keep the `link_id` of each link to keep its click history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_links: Option<Vec<SocialLinkUpdate>>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A social link of the footer in a settings change.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SocialLinkUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub link_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The ID of an existing link. Send it to keep the click history of the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<u64>,
    /// Keys that this version of the SDK does not know. The SDK sends them as they are.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// The response of `Email::update_settings`: the settings, the new etag and the removed unsafe values.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct SettingsResult {
    pub data: EmailSettings,
    /// Send this etag with the next write to the email.
    pub etag: String,
    /// The keys of the unsafe values that the API removed or replaced, if there are some.
    #[serde(default)]
    pub rejected: Option<Rejected>,
}

/// A template that the API made from an email.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EmailTemplate {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub template_type: String,
    pub created_at: String,
}

/// The request body of `Email::create_template`.
#[derive(Serialize)]
struct NameBody<'a> {
    name: &'a str,
}

/// A saved copy of the content and the settings of an email.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EmailVersion {
    pub id: String,
    pub created_at: String,
    /// True for the version that the email has now.
    pub current: bool,
}

/// The page options of `Email::versions`. The SDK does not send `None` values.
#[derive(Debug, Clone, Default)]
pub struct ListVersionsOptions {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// The email methods. Get it with `MailerLite::email`.
#[derive(Debug)]
pub struct Email {
    mailerlite: MailerLite,
    idempotency_key: Option<String>,
}

impl Email {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self {
            mailerlite,
            idempotency_key: None,
        }
    }

    /// Sends the `Idempotency-Key` header with the writes of this value. If a write fails with a network
    /// error, send the same write again with the same key. If the first request was a success, the API
    /// returns the stored response, and `ApiResponse::is_idempotent_replay` is `true`.
    ///
    /// - Only these methods send the key: `create_block`, `update_block`, `delete_block`, `move_block`,
    ///   `update_settings`, `create_template`, `create_version` and `restore_version`. The read methods and
    ///   the validate methods ignore it.
    /// - The key must be 1 to 255 printable ASCII characters, without a space at the start or the end.
    ///   The SDK checks the key when a write runs. A bad key gives `Error::InvalidIdempotencyKey`, and the
    ///   SDK sends no request.
    ///
    /// Warning: use a new key for each write. Do not use an `Email` value that has a key for a different
    /// write. The API keeps a key for 24 hours, and it rejects the same key with a different request.
    pub fn with_idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());

        self
    }

    /// Checks the idempotency key and returns it for a write.
    fn write_key(&self) -> Result<Option<&str>, Error> {
        match self.idempotency_key.as_deref() {
            Some(key) => validate::idempotency_key(key).map(|()| Some(key)),
            None => Ok(None),
        }
    }

    /// Gets an email with its settings and blocks, and the etag of the email.
    pub async fn find(
        &self,
        email_id: impl AsRef<str>,
    ) -> Result<ApiResponse<WithEtag<EmailDocument>>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;

        http::send(
            &self.mailerlite,
            Method::GET,
            &format!("{END_POINT}/{email_id}"),
            &[],
            NO_BODY,
            None,
        )
        .await
    }

    /// Gets the HTML of an email, and the etag of the email.
    ///
    /// The API can give a temporary `503` error. Its body can be other than JSON.
    pub async fn html(
        &self,
        email_id: impl AsRef<str>,
    ) -> Result<ApiResponse<WithEtag<EmailHtml>>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;

        http::send(
            &self.mailerlite,
            Method::GET,
            &format!("{END_POINT}/{email_id}/html"),
            &[],
            NO_BODY,
            None,
        )
        .await
    }

    /// Gets a block of an email with its placement, and the etag of the email.
    pub async fn find_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
    ) -> Result<ApiResponse<WithEtag<EmailBlock>>, Error> {
        let (email_id, block_id): (&str, &str) = (email_id.as_ref(), block_id.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::path_id("block_id", block_id)?;

        http::send(
            &self.mailerlite,
            Method::GET,
            &format!("{END_POINT}/{email_id}/blocks/{block_id}"),
            &[],
            NO_BODY,
            None,
        )
        .await
    }

    /// Inserts a block into an email. Returns the new block (status `201`) and the new etag.
    pub async fn create_block(
        &self,
        email_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        block: &NewBlock,
    ) -> Result<ApiResponse<WithEtag<EmailBlock>>, Error> {
        let (email_id, etag): (&str, &str) = (email_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::etag(etag)?;
        if let Some(placement) = &block.placement {
            placement.check()?;
        }

        let idempotency_key: Option<&str> = self.write_key()?;

        http::send(
            &self.mailerlite,
            Method::POST,
            &format!("{END_POINT}/{email_id}/blocks"),
            &[],
            Some(&EtagBody { body: block, etag }),
            idempotency_key,
        )
        .await
    }

    /// Replaces the content of a block. The API removes the keys that you do not send. The type and the
    /// position of the block do not change. Returns the block and the new etag.
    pub async fn update_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        editor_content: &Map<String, Value>,
    ) -> Result<ApiResponse<WithEtag<EmailBlock>>, Error> {
        let (email_id, block_id, etag): (&str, &str, &str) =
            (email_id.as_ref(), block_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::path_id("block_id", block_id)?;
        validate::etag(etag)?;

        let idempotency_key: Option<&str> = self.write_key()?;

        http::send(
            &self.mailerlite,
            Method::PATCH,
            &format!("{END_POINT}/{email_id}/blocks/{block_id}"),
            &[],
            Some(&EditorContentBody {
                editor_content,
                etag,
            }),
            idempotency_key,
        )
        .await
    }

    /// Deletes a block and the blocks in it. The response has only the new etag.
    pub async fn delete_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
        etag: impl AsRef<str>,
    ) -> Result<ApiResponse<EtagOnly>, Error> {
        let (email_id, block_id, etag): (&str, &str, &str) =
            (email_id.as_ref(), block_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::path_id("block_id", block_id)?;
        validate::etag(etag)?;

        let idempotency_key: Option<&str> = self.write_key()?;

        http::send(
            &self.mailerlite,
            Method::DELETE,
            &format!("{END_POINT}/{email_id}/blocks/{block_id}"),
            &[],
            Some(&EtagBody {
                body: &Map::new(),
                etag,
            }),
            idempotency_key,
        )
        .await
    }

    /// Moves a block among the blocks at the same level. The nesting of the block does not change.
    /// Returns the block and the new etag.
    pub async fn move_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        placement: &Placement,
    ) -> Result<ApiResponse<WithEtag<EmailBlock>>, Error> {
        let (email_id, block_id, etag): (&str, &str, &str) =
            (email_id.as_ref(), block_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::path_id("block_id", block_id)?;
        validate::etag(etag)?;
        placement.check()?;

        let idempotency_key: Option<&str> = self.write_key()?;

        http::send(
            &self.mailerlite,
            Method::POST,
            &format!("{END_POINT}/{email_id}/blocks/{block_id}/move"),
            &[],
            Some(&MoveBody { placement, etag }),
            idempotency_key,
        )
        .await
    }

    /// Checks a block with the rules of `create_block`. The API saves nothing. The method needs no email
    /// and no etag.
    pub async fn validate_block(&self, block: &NewBlock) -> Result<ApiResponse<Validation>, Error> {
        if let Some(placement) = &block.placement {
            placement.check()?;
        }

        http::send(
            &self.mailerlite,
            Method::POST,
            &format!("{END_POINT}/blocks/validate"),
            &[],
            Some(block),
            None,
        )
        .await
    }

    /// Changes the design settings of an email. The SDK sends only the values that you set.
    /// Returns the settings, the new etag and the keys of the unsafe values that the API removed.
    pub async fn update_settings(
        &self,
        email_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        settings: &EmailSettingsUpdate,
    ) -> Result<ApiResponse<SettingsResult>, Error> {
        let (email_id, etag): (&str, &str) = (email_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::etag(etag)?;
        validate::no_etag_key(&settings.extra)?;

        let idempotency_key: Option<&str> = self.write_key()?;

        http::send(
            &self.mailerlite,
            Method::PATCH,
            &format!("{END_POINT}/{email_id}/settings"),
            &[],
            Some(&EtagBody {
                body: settings,
                etag,
            }),
            idempotency_key,
        )
        .await
    }

    /// Checks a settings change with the rules of `update_settings`. The API saves nothing. The method
    /// needs no etag.
    pub async fn validate_settings(
        &self,
        email_id: impl AsRef<str>,
        settings: &EmailSettingsUpdate,
    ) -> Result<ApiResponse<Validation>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;
        validate::no_etag_key(&settings.extra)?;

        http::send(
            &self.mailerlite,
            Method::POST,
            &format!("{END_POINT}/{email_id}/settings/validate"),
            &[],
            Some(settings),
            None,
        )
        .await
    }

    /// Saves a copy of an email as a new template (status `201`). The API cannot delete templates.
    pub async fn create_template(
        &self,
        email_id: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Result<ApiResponse<DataOnly<EmailTemplate>>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;

        let idempotency_key: Option<&str> = self.write_key()?;

        http::send(
            &self.mailerlite,
            Method::POST,
            &format!("{END_POINT}/{email_id}/templates"),
            &[],
            Some(&NameBody {
                name: name.as_ref(),
            }),
            idempotency_key,
        )
        .await
    }

    /// Lists the saved versions of an email. The newest version is first.
    pub async fn versions(
        &self,
        email_id: impl AsRef<str>,
        options: ListVersionsOptions,
    ) -> Result<ApiResponse<Paginated<EmailVersion>>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;
        let query: Vec<(&str, String)> = validate::query_pairs([
            ("page", options.page.map(|page| page.to_string())),
            ("limit", options.limit.map(|limit| limit.to_string())),
        ]);

        http::send(
            &self.mailerlite,
            Method::GET,
            &format!("{END_POINT}/{email_id}/versions"),
            &query,
            NO_BODY,
            None,
        )
        .await
    }

    /// Saves the email as a new version. The request has no body. If nothing changed after the last
    /// version, the API saves no new version and returns the last version: then `status_code` is `200`,
    /// not `201`.
    pub async fn create_version(
        &self,
        email_id: impl AsRef<str>,
    ) -> Result<ApiResponse<DataOnly<EmailVersion>>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;

        let idempotency_key: Option<&str> = self.write_key()?;

        http::send(
            &self.mailerlite,
            Method::POST,
            &format!("{END_POINT}/{email_id}/versions"),
            &[],
            NO_BODY,
            idempotency_key,
        )
        .await
    }

    /// Puts an email back to a saved version. Before the restore, the API saves the current state as a
    /// version, so you can undo the restore. Returns the full email and the new etag.
    pub async fn restore_version(
        &self,
        email_id: impl AsRef<str>,
        version_id: impl AsRef<str>,
        etag: impl AsRef<str>,
    ) -> Result<ApiResponse<WithEtag<EmailDocument>>, Error> {
        let (email_id, version_id, etag): (&str, &str, &str) =
            (email_id.as_ref(), version_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::path_id("version_id", version_id)?;
        validate::etag(etag)?;

        let idempotency_key: Option<&str> = self.write_key()?;

        http::send(
            &self.mailerlite,
            Method::POST,
            &format!("{END_POINT}/{email_id}/versions/{version_id}/restore"),
            &[],
            Some(&EtagBody {
                body: &Map::new(),
                etag,
            }),
            idempotency_key,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn open_enums_keep_unknown_values() {
        assert_eq!(
            BlockType::from("paragraph".to_string()),
            BlockType::Paragraph
        );
        assert_eq!(
            BlockType::from("countdown".to_string()),
            BlockType::Other("countdown".to_string())
        );
        assert_eq!(String::from(BlockType::BulletedList), "bulleted_list");
        assert_eq!(String::from(BlockType::Other("x".to_string())), "x");
        assert_eq!(
            serde_json::to_value(Position::After).unwrap(),
            json!("after")
        );
        assert_eq!(
            serde_json::from_value::<BrandMode>(json!("custom")).unwrap(),
            BrandMode::Custom
        );
        assert_eq!(BrandMode::Account.to_string(), "account");
    }
}
