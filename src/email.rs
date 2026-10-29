//! Emails built with the simple editor: blocks, design settings, templates and versions.
//!
//! To make such an email, create a campaign with `editor` set to `campaign::EDITOR_SIMPLE`.
//! Each write needs the etag of the last response for the email, and returns a new etag.

use crate::{
    api_response::{object_or_empty_array, ApiResponse, EtagOnly, WithEtag},
    error::Error,
    http::{self, Method, NO_BODY},
    validate, MailerLite,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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
#[derive(Debug, Clone, Default, Serialize)]
pub struct Placement {
    pub position: Position,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
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

    fn check(&self) -> Result<(), Error> {
        match &self.block_id {
            Some(block_id) => validate::path_id("placement.block_id", block_id),
            None => Ok(()),
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

/// The email methods. Get it with `MailerLite::email`.
#[derive(Debug)]
pub struct Email {
    mailerlite: MailerLite,
}

impl Email {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
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

        http::send(
            &self.mailerlite,
            Method::POST,
            &format!("{END_POINT}/{email_id}/blocks"),
            &[],
            Some(&EtagBody { body: block, etag }),
            None,
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

        http::send(
            &self.mailerlite,
            Method::PATCH,
            &format!("{END_POINT}/{email_id}/blocks/{block_id}"),
            &[],
            Some(&EditorContentBody {
                editor_content,
                etag,
            }),
            None,
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

        http::send(
            &self.mailerlite,
            Method::DELETE,
            &format!("{END_POINT}/{email_id}/blocks/{block_id}"),
            &[],
            Some(&EtagBody {
                body: &Map::new(),
                etag,
            }),
            None,
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

        http::send(
            &self.mailerlite,
            Method::POST,
            &format!("{END_POINT}/{email_id}/blocks/{block_id}/move"),
            &[],
            Some(&MoveBody { placement, etag }),
            None,
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
