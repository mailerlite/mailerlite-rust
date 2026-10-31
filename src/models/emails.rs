//! Simple-editor email documents, design settings, and versioned response envelopes.
use crate::models::open_enum::open_enum;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
/// The `code` of a `409` error when the email changed after you read the etag.
/// Then `ApiError::etag` has the current etag.
pub const VERSION_CONFLICT: &str = "version_conflict";

open_enum! {
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
    /// Opaque identifier, preserved as returned by the server.
    pub id: String,
    /// The type of the email, for example `"builder_inline_email"`.
    #[serde(rename = "type")]
    pub email_type: String,
    /// Server discriminator for the email owner.
    pub emailable_type: String,
    /// Identifier of the campaign or automation that owns this email.
    pub emailable_id: String,
    /// Design settings currently applied to this email.
    pub settings: EmailSettings,
    #[serde(default)]
    /// Blocks value in the API representation.
    pub blocks: Vec<EmailBlock>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// One editor block with its content and nested children.
pub struct EmailBlock {
    /// Opaque identifier, preserved as returned by the server.
    pub id: String,
    #[serde(rename = "type")]
    /// Editor block discriminator.
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
    /// Identifier of the preceding block, if one exists.
    pub after: Option<String>,
    /// Identifier of the following block, if one exists.
    pub before: Option<String>,
}

/// The design settings of an email, as the API returns them.
///
/// A value is `None` when the API sent `null` or no key. Each `extra` map keeps the keys that this version
/// of the SDK does not know. To change the settings, use `EmailSettingsUpdate`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EmailSettings {
    /// Brand palette and typography overrides.
    pub brand: Option<Brand>,
    /// Email width and alignment settings.
    pub layout: Option<Layout>,
    /// Link, heading, and paragraph styles.
    pub typography: Option<Typography>,
    /// Default button styles for this email.
    pub button: Option<Button>,
    /// Footer content and display settings.
    pub footer: Option<Footer>,
    #[serde(flatten)]
    /// Additional wire properties not represented by the typed fields.
    pub extra: Map<String, Value>,
}

/// The brand settings. `mode` is one of the `BrandMode` values.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Brand {
    /// Source of the brand settings: account, template, or custom.
    pub mode: Option<String>,
    /// CSS font-family value used for this style.
    pub font_family: Option<String>,
    /// Background color in the API color format.
    pub background_color: Option<String>,
    /// Foreground text color in the API color format.
    pub text_color: Option<String>,
    /// Border color in the API color format.
    pub border_color: Option<String>,
    #[serde(flatten)]
    /// Additional wire properties not represented by the typed fields.
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Layout settings returned by the editor.
pub struct Layout {
    /// Width preset understood by the editor.
    pub width_mode: Option<String>,
    /// Horizontal alignment understood by the editor.
    pub alignment: Option<String>,
    #[serde(flatten)]
    /// Additional wire properties not represented by the typed fields.
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Typography settings returned by the editor.
pub struct Typography {
    /// Styles for hyperlinks.
    pub link: Option<LinkStyle>,
    /// Styles shared by heading levels.
    pub heading: Option<HeadingStyle>,
    /// Styles shared by paragraphs.
    pub paragraph: Option<ParagraphStyle>,
    #[serde(flatten)]
    /// Additional wire properties not represented by the typed fields.
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Link style settings returned by the editor.
pub struct LinkStyle {
    /// Text color in the API color format.
    pub color: Option<String>,
    #[serde(flatten)]
    /// Additional wire properties not represented by the typed fields.
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Heading style settings returned by the editor.
pub struct HeadingStyle {
    /// CSS font-family value used for this style.
    pub font_family: Option<String>,
    /// Font weight as a CSS string.
    pub font_weight: Option<String>,
    /// Line height as a CSS string.
    pub line_height: Option<String>,
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<String>,
    /// Styles for first-level headings.
    pub h1: Option<TextStyle>,
    /// Styles for second-level headings.
    pub h2: Option<TextStyle>,
    /// Styles for third-level headings.
    pub h3: Option<TextStyle>,
    #[serde(flatten)]
    /// Additional wire properties not represented by the typed fields.
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Paragraph style settings returned by the editor.
pub struct ParagraphStyle {
    /// CSS font-family value used for this style.
    pub font_family: Option<String>,
    /// Font weight as a CSS string.
    pub font_weight: Option<String>,
    /// Line height as a CSS string.
    pub line_height: Option<String>,
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<String>,
    /// Styles for the default paragraph level.
    pub p1: Option<TextStyle>,
    #[serde(flatten)]
    /// Additional wire properties not represented by the typed fields.
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Text style settings returned by the editor.
pub struct TextStyle {
    /// Text color in the API color format.
    pub color: Option<String>,
    /// Font size as a CSS string.
    pub size: Option<String>,
    #[serde(flatten)]
    /// Additional wire properties not represented by the typed fields.
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Button settings returned by the editor.
pub struct Button {
    /// CSS font-family value used for this style.
    pub font_family: Option<String>,
    /// Font weight as a CSS string.
    pub font_weight: Option<String>,
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<String>,
    /// Background color in the API color format.
    pub background_color: Option<String>,
    /// Foreground text color in the API color format.
    pub text_color: Option<String>,
    /// Corner radius as a CSS string.
    pub border_radius: Option<String>,
    #[serde(flatten)]
    /// Additional wire properties not represented by the typed fields.
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Footer settings returned by the editor.
pub struct Footer {
    /// Whether the footer displays preference-management links.
    pub show_preferences: Option<bool>,
    /// Company name represented as editor content.
    pub company_name: Option<EditorText>,
    /// Postal address represented as editor content.
    pub address: Option<EditorText>,
    /// Additional footer information represented as editor content.
    pub info_text: Option<EditorText>,
    /// Label for the unsubscribe link.
    pub unsubscribe_text: Option<String>,
    /// Label for the preferences link.
    pub preferences_text: Option<String>,
    /// Social-profile links displayed in the footer.
    pub social_links: Option<Vec<SocialLink>>,
    #[serde(flatten)]
    /// Additional wire properties not represented by the typed fields.
    pub extra: Map<String, Value>,
}

/// A footer text as an editor document. In an update, an empty document (`{}` or `{"content": []}`) removes
/// the text, and the email uses the text of the account again.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EditorText {
    #[serde(default, deserialize_with = "object_or_empty_array")]
    /// Editor document with the API camelCase property names.
    pub editor_content: Map<String, Value>,
}

/// A social link of the footer. `link_id` keeps the click history of the link.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct SocialLink {
    /// URL returned by or submitted to the API.
    pub url: Option<String>,
    #[serde(rename = "type")]
    /// Social-link type understood by the editor.
    pub link_type: Option<String>,
    /// Display label for the link.
    pub label: Option<String>,
    /// Link id value in the API representation.
    pub link_id: Option<u64>,
    #[serde(flatten)]
    /// Additional wire properties not represented by the typed fields.
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Email html settings returned by the editor.
pub struct EmailHtml {
    /// Rendered email HTML returned by the API.
    pub html: String,
}

/// The result of a validation. An invalid input is not an error: `valid` is `false`, and `errors` has
/// the messages for each field.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Validation {
    /// Whether the submitted content satisfies the server validation rules.
    pub valid: bool,
    /// The messages for each field path. Empty when the input is valid.
    #[serde(default, deserialize_with = "object_or_empty_array")]
    pub errors: BTreeMap<String, Vec<String>>,
    /// Version and hash of the validation rules used by the server.
    pub schema: ValidationSchema,
    /// The keys of the unsafe values that the API removed. Only the settings validation has it.
    #[serde(default)]
    pub rejected: Option<Rejected>,
}

/// The version of the rules that the validation used. The API can send `null` for both values.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ValidationSchema {
    /// Server validation schema version, when available.
    pub version: Option<String>,
    /// Server validation schema hash, when available.
    pub hash: Option<String>,
}

/// The unsafe values that the API removed or replaced.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Rejected {
    /// Server explanation of rejected settings.
    pub message: String,
    /// The paths of the keys, for example `footer.social_links[0].url`.
    #[serde(default)]
    pub keys: Vec<String>,
}

/// The response of `Emails::update_settings`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct SettingsResult {
    /// Typed payload returned inside the API envelope.
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
    /// Opaque identifier, preserved as returned by the server.
    pub id: String,
    /// Display name assigned by the API.
    pub name: String,
    #[serde(rename = "type")]
    /// Template type value in the API representation.
    pub template_type: String,
    /// Server timestamp retained without assuming a timezone.
    pub created_at: String,
}

/// A saved copy of the content and the settings of an email.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EmailVersion {
    /// Opaque identifier, preserved as returned by the server.
    pub id: String,
    /// Server timestamp retained without assuming a timezone.
    pub created_at: String,
    /// True for the version that the email has now.
    pub current: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Typed response payload paired with an optimistic-concurrency version.
pub struct WithEtag<T> {
    /// Typed payload returned inside the API envelope.
    pub data: T,
    /// Send this etag with the next write to the email.
    pub etag: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Typed response payload without version metadata.
pub(crate) struct DataOnly<T> {
    /// Typed payload returned inside the API envelope.
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// New optimistic-concurrency version returned after a write.
pub struct EtagOnly {
    /// Send this etag with the next write to the email.
    pub etag: String,
}

/// Decodes a JSON object, and also accepts `[]` as an empty object. PHP encodes an empty array as `[]`.
pub(crate) fn object_or_empty_array<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + serde::de::DeserializeOwned,
{
    match Value::deserialize(deserializer)? {
        Value::Array(items) if items.is_empty() => Ok(T::default()),
        Value::Null => Ok(T::default()),
        value => serde_json::from_value(value).map_err(serde::de::Error::custom),
    }
}
