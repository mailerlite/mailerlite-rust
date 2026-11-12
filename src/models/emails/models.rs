//! Simple-editor email documents, design settings, and versioned response envelopes.
use super::{BlockType, EmailOwnerType, EmailTemplateType, EmailType, SocialLinkType};
use crate::models::open_enum::open_enum;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
/// HTTP metadata for the simple-editor endpoints.
///
/// Access standard headers explicitly through [`Self::http`], for example
/// `response.metadata.http.status`. Replay information belongs to this metadata type.
#[derive(Debug, Clone)]
pub struct EmailResponseMetadata {
    /// Standard status, rate-limit, and correlation metadata.
    pub http: crate::ResponseMetadata,
    /// Whether the server returned a stored response for an idempotency key.
    pub idempotent_replayed: bool,
}

impl EmailResponseMetadata {
    /// Whether this response replayed a previously completed write.
    pub fn is_idempotent_replay(&self) -> bool {
        self.idempotent_replayed
    }
}

/// The `code` of a `409` error when the email changed after you read the etag.
/// Then `ApiError::etag` has the current etag.
pub const VERSION_CONFLICT: &str = "version_conflict";

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

// Settings choices preserve unknown API values in Other.
open_enum! {
    /// Editor width preset.
    pub enum WidthMode {
        Default => "default",
        Full => "full",
    }
}

open_enum! {
    /// Horizontal alignment of the email layout.
    pub enum LayoutAlignment {
        Left => "left",
        Center => "center",
    }
}

open_enum! {
    /// Editor font weight, serialized as a numeric CSS string.
    pub enum FontWeight {
        Light => "300",
        Normal => "400",
        Medium => "500",
        SemiBold => "600",
        Bold => "700",
    }
}

open_enum! {
    /// Editor line height, serialized as a percentage string.
    pub enum LineHeight {
        Percent100 => "100%",
        Percent137 => "137%",
        Percent150 => "150%",
        Percent165 => "165%",
        Percent200 => "200%",
    }
}

open_enum! {
    /// Editor letter-spacing preset, preserving its exact wire string.
    pub enum LetterSpacing {
        Tightest => "-0.05",
        Tight => "-0.025",
        Normal => "0",
        Wide => "0.025",
        Widest => "0.05",
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
    pub email_type: EmailType,
    /// Server discriminator for the email owner.
    pub emailable_type: EmailOwnerType,
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
    /// Editor block discriminator, preserving future block types.
    #[serde(rename = "type")]
    pub block_type: BlockType,
    /// Stored editor document, preserved without schema validation or coercion.
    #[serde(default = "super::wire::empty_object")]
    pub editor_content: Value,
    /// The blocks in this block, for example the items of a list.
    #[serde(default, deserialize_with = "super::wire::null_default")]
    pub children: Vec<EmailBlock>,
    /// The neighbors of the block. Only the responses for one block have it.
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

/// Brand settings with a typed source and preserved unknown modes.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Brand {
    /// Source of the brand settings: account, template, or custom.
    pub mode: Option<BrandMode>,
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
    pub width_mode: Option<WidthMode>,
    /// Horizontal alignment understood by the editor.
    pub alignment: Option<LayoutAlignment>,
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
    pub font_weight: Option<FontWeight>,
    /// Line height as a CSS string.
    pub line_height: Option<LineHeight>,
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<LetterSpacing>,
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
    pub font_weight: Option<FontWeight>,
    /// Line height as a CSS string.
    pub line_height: Option<LineHeight>,
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<LetterSpacing>,
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
    pub font_weight: Option<FontWeight>,
    /// Letter spacing as a CSS string.
    pub letter_spacing: Option<LetterSpacing>,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditorText {
    #[serde(default = "super::wire::empty_object")]
    /// Editor document with the API camelCase property names.
    pub editor_content: Value,
}

impl Default for EditorText {
    fn default() -> Self {
        Self {
            editor_content: super::wire::empty_object(),
        }
    }
}

/// A social link of the footer. `link_id` keeps the click history of the link.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct SocialLink {
    /// URL returned by or submitted to the API.
    pub url: Option<String>,
    #[serde(rename = "type")]
    /// Social-link type understood by the editor.
    pub link_type: Option<SocialLinkType>,
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
    /// The keys that the same write would name in [`Rejected`], if there are some. The API omits it
    /// when `valid` is `false`.
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

/// The keys of the values that the API removes, or replaces with a safe value, before it stores a
/// write. The write still succeeds.
///
/// - [`Emails::create_block`] and [`Emails::update_block`]: the keys that the block type does not
///   declare, at the top level of `editor_content`, in `attrs` or in `mobile`. The API removes them
///   and stores the rest of the block. A value that is not allowed in email content is not
///   rejected this way: it fails the whole write with HTTP 422.
/// - [`Emails::update_settings`]: the keys of unsafe values. The API removes the value, or replaces
///   it with a safe one and keeps the key, for example a URL becomes `#`. The API checks the whole
///   footer after it merges your changes, so a key can name a value that was already stored.
/// - [`Emails::validate_block`] and [`Emails::validate_settings`]: the keys that the same write
///   would name. Nothing is stored. The API omits `rejected` when `valid` is `false`, because a
///   write that fails stores nothing.
///
/// [`Emails::create_block`]: crate::resources::emails::Emails::create_block
/// [`Emails::update_block`]: crate::resources::emails::Emails::update_block
/// [`Emails::update_settings`]: crate::resources::emails::Emails::update_settings
/// [`Emails::validate_block`]: crate::resources::emails::Emails::validate_block
/// [`Emails::validate_settings`]: crate::resources::emails::Emails::validate_settings
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Rejected {
    /// Server explanation of the rejected values.
    pub message: String,
    /// The paths of the keys, for example `editor_content.attrs.onclick` or
    /// `footer.social_links[0].url`.
    #[serde(default)]
    pub keys: Vec<String>,
}

/// The response of a versioned write that can remove or replace values: [`BlockWriteResult`] and
/// [`SettingsResult`].
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct WriteResult<T> {
    /// Typed payload returned inside the API envelope.
    pub data: T,
    /// Send this etag with the next write to the email.
    pub etag: String,
    /// The keys that the API removed or replaced, if there are some. See [`Rejected`] for what they
    /// mean for each method.
    pub rejected: Option<Rejected>,
}

/// The response of [`Emails::create_block`](crate::resources::emails::Emails::create_block) and
/// [`Emails::update_block`](crate::resources::emails::Emails::update_block).
pub type BlockWriteResult = WriteResult<EmailBlock>;

/// The response of [`Emails::update_settings`](crate::resources::emails::Emails::update_settings).
pub type SettingsResult = WriteResult<EmailSettings>;

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
    pub template_type: EmailTemplateType,
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
/// Typed response payload paired with an optimistic-concurrency version.
///
/// This reusable envelope remains constructible by callers.
/// ```
/// use mailerlite_rs::models::emails::{WithEtag, EtagOnly};
///
/// let envelope = WithEtag { data: "payload", etag: "42".to_owned() };
/// let version = EtagOnly { etag: envelope.etag };
/// assert_eq!(version.etag, "42");
/// ```
pub struct WithEtag<T> {
    /// Typed payload returned inside the API envelope.
    pub data: T,
    /// Send this etag with the next write to the email.
    pub etag: String,
}

#[derive(Debug, Clone, Deserialize)]
/// New optimistic-concurrency version returned after a write.
pub struct EtagOnly {
    /// Send this etag with the next write to the email.
    pub etag: String,
}

/// Decodes a JSON object, accepting the API's `[]` representation for an empty object.
fn object_or_empty_array<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + serde::de::DeserializeOwned,
{
    match Value::deserialize(deserializer)? {
        Value::Array(items) if items.is_empty() => Ok(T::default()),
        value => serde_json::from_value(value).map_err(serde::de::Error::custom),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, reason = "Tests assert synthetic model decoding")]
mod tests {
    use super::*;
    use crate::models::emails::BlockType;
    use serde_json::json;

    #[test]
    fn brand_modes_decode_known_unknown_and_missing_values() {
        for (wire, expected) in [
            (json!("account"), Some(BrandMode::Account)),
            (json!("custom"), Some(BrandMode::Custom)),
            (json!("future"), Some(BrandMode::Other("future".into()))),
            (Value::Null, None),
        ] {
            let brand: Brand = serde_json::from_value(json!({"mode": wire})).unwrap();
            assert_eq!(brand.mode, expected);
        }
        let brand: Brand = serde_json::from_value(json!({})).unwrap();
        assert_eq!(brand.mode, None);
        assert!(serde_json::from_value::<Brand>(json!({"mode": 42})).is_err());
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
    #[test]
    fn style_choices_preserve_known_and_unknown_wire_values() {
        // Verify the API wire strings for each supported settings choice.
        assert_eq!(
            serde_json::to_value(WidthMode::Default).unwrap(),
            json!("default")
        );
        assert_eq!(
            serde_json::from_value::<WidthMode>(json!("default")).unwrap(),
            WidthMode::Default
        );
        assert_eq!(
            serde_json::to_value(WidthMode::Full).unwrap(),
            json!("full")
        );
        assert_eq!(
            serde_json::from_value::<WidthMode>(json!("full")).unwrap(),
            WidthMode::Full
        );
        assert_eq!(
            serde_json::from_value::<WidthMode>(json!("future")).unwrap(),
            WidthMode::Other("future".into())
        );
        assert_eq!(
            serde_json::to_value(LayoutAlignment::Left).unwrap(),
            json!("left")
        );
        assert_eq!(
            serde_json::from_value::<LayoutAlignment>(json!("left")).unwrap(),
            LayoutAlignment::Left
        );
        assert_eq!(
            serde_json::to_value(LayoutAlignment::Center).unwrap(),
            json!("center")
        );
        assert_eq!(
            serde_json::from_value::<LayoutAlignment>(json!("center")).unwrap(),
            LayoutAlignment::Center
        );
        assert_eq!(
            serde_json::from_value::<LayoutAlignment>(json!("future")).unwrap(),
            LayoutAlignment::Other("future".into())
        );
        assert_eq!(
            serde_json::to_value(FontWeight::Light).unwrap(),
            json!("300")
        );
        assert_eq!(
            serde_json::from_value::<FontWeight>(json!("300")).unwrap(),
            FontWeight::Light
        );
        assert_eq!(
            serde_json::to_value(FontWeight::Normal).unwrap(),
            json!("400")
        );
        assert_eq!(
            serde_json::from_value::<FontWeight>(json!("400")).unwrap(),
            FontWeight::Normal
        );
        assert_eq!(
            serde_json::to_value(FontWeight::Medium).unwrap(),
            json!("500")
        );
        assert_eq!(
            serde_json::from_value::<FontWeight>(json!("500")).unwrap(),
            FontWeight::Medium
        );
        assert_eq!(
            serde_json::to_value(FontWeight::SemiBold).unwrap(),
            json!("600")
        );
        assert_eq!(
            serde_json::from_value::<FontWeight>(json!("600")).unwrap(),
            FontWeight::SemiBold
        );
        assert_eq!(
            serde_json::to_value(FontWeight::Bold).unwrap(),
            json!("700")
        );
        assert_eq!(
            serde_json::from_value::<FontWeight>(json!("700")).unwrap(),
            FontWeight::Bold
        );
        assert_eq!(
            serde_json::from_value::<FontWeight>(json!("future")).unwrap(),
            FontWeight::Other("future".into())
        );
        assert_eq!(
            serde_json::to_value(LineHeight::Percent100).unwrap(),
            json!("100%")
        );
        assert_eq!(
            serde_json::from_value::<LineHeight>(json!("100%")).unwrap(),
            LineHeight::Percent100
        );
        assert_eq!(
            serde_json::to_value(LineHeight::Percent137).unwrap(),
            json!("137%")
        );
        assert_eq!(
            serde_json::from_value::<LineHeight>(json!("137%")).unwrap(),
            LineHeight::Percent137
        );
        assert_eq!(
            serde_json::to_value(LineHeight::Percent150).unwrap(),
            json!("150%")
        );
        assert_eq!(
            serde_json::from_value::<LineHeight>(json!("150%")).unwrap(),
            LineHeight::Percent150
        );
        assert_eq!(
            serde_json::to_value(LineHeight::Percent165).unwrap(),
            json!("165%")
        );
        assert_eq!(
            serde_json::from_value::<LineHeight>(json!("165%")).unwrap(),
            LineHeight::Percent165
        );
        assert_eq!(
            serde_json::to_value(LineHeight::Percent200).unwrap(),
            json!("200%")
        );
        assert_eq!(
            serde_json::from_value::<LineHeight>(json!("200%")).unwrap(),
            LineHeight::Percent200
        );
        assert_eq!(
            serde_json::from_value::<LineHeight>(json!("future")).unwrap(),
            LineHeight::Other("future".into())
        );
        assert_eq!(
            serde_json::to_value(LetterSpacing::Tightest).unwrap(),
            json!("-0.05")
        );
        assert_eq!(
            serde_json::from_value::<LetterSpacing>(json!("-0.05")).unwrap(),
            LetterSpacing::Tightest
        );
        assert_eq!(
            serde_json::to_value(LetterSpacing::Tight).unwrap(),
            json!("-0.025")
        );
        assert_eq!(
            serde_json::from_value::<LetterSpacing>(json!("-0.025")).unwrap(),
            LetterSpacing::Tight
        );
        assert_eq!(
            serde_json::to_value(LetterSpacing::Normal).unwrap(),
            json!("0")
        );
        assert_eq!(
            serde_json::from_value::<LetterSpacing>(json!("0")).unwrap(),
            LetterSpacing::Normal
        );
        assert_eq!(
            serde_json::to_value(LetterSpacing::Wide).unwrap(),
            json!("0.025")
        );
        assert_eq!(
            serde_json::from_value::<LetterSpacing>(json!("0.025")).unwrap(),
            LetterSpacing::Wide
        );
        assert_eq!(
            serde_json::to_value(LetterSpacing::Widest).unwrap(),
            json!("0.05")
        );
        assert_eq!(
            serde_json::from_value::<LetterSpacing>(json!("0.05")).unwrap(),
            LetterSpacing::Widest
        );
        assert_eq!(
            serde_json::from_value::<LetterSpacing>(json!("future")).unwrap(),
            LetterSpacing::Other("future".into())
        );
    }
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
    #[test]
    fn object_fields_accept_empty_arrays_but_reject_malformed_values() {
        // Object fields also accept the API's alternate empty-array representation.
        // Validation-error maps still require objects; editor documents remain opaque.
        for wire in [json!({}), json!([])] {
            let text: EditorText = serde_json::from_value(json!({"editor_content": wire})).unwrap();
            assert_eq!(text.editor_content, wire);
            let validation: Validation = serde_json::from_value(json!({
                "valid": true, "errors": wire, "schema": {"version": null, "hash": null}
            }))
            .unwrap();
            assert!(validation.errors.is_empty());
        }
        for wire in [
            Value::Null,
            json!(true),
            json!(7),
            json!("bad"),
            json!([{}]),
        ] {
            assert_eq!(
                serde_json::from_value::<EditorText>(json!({"editor_content": wire}))
                    .unwrap()
                    .editor_content,
                wire
            );
            assert_eq!(
                serde_json::from_value::<EmailBlock>(
                    json!({"id":"1", "type":"paragraph", "editor_content":wire})
                )
                .unwrap()
                .editor_content,
                wire
            );
            assert!(
                serde_json::from_value::<Validation>(json!({
                    "valid": true, "errors": wire, "schema": {"version": null, "hash": null}
                }))
                .is_err()
            );
        }
    }
    #[test]
    fn nullable_attributes_decode() {
        for (kind, attrs) in [
            (
                "image",
                json!({"src":"https://example.test/image.png","url":null,"alt":null,"caption":null,"linkType":null}),
            ),
            ("button", json!({"url":null,"linkType":null})),
            ("paragraph", json!({"class":null})),
            ("video", json!({"alt":null,"videoCode":null})),
            (
                "social_icon",
                json!({"items":[{"uuid":"one","type":"website","url":"https://example.test","label":null}]}),
            ),
        ] {
            let result = serde_json::from_value::<EmailBlock>(
                json!({"id":"57","type":kind,"editor_content":{"attrs":attrs}}),
            );
            assert!(result.is_ok(), "{kind}: {result:?}");
        }
    }

    #[test]
    fn placeholder_and_fractional_images_decode() {
        for attrs in [
            json!({}),
            json!({"src":null}),
            json!({"src":"https://example.test/image.png","width":300.5,"height":150.25}),
        ] {
            let result = serde_json::from_value::<EmailBlock>(
                json!({"id":"57","type":"image","editor_content":{"attrs":attrs}}),
            );
            assert!(result.is_ok(), "{result:?}");
            if attrs.get("width").is_some() {
                assert_eq!(
                    result.unwrap().editor_content["attrs"]["width"],
                    json!(300.5)
                );
            }
        }
    }

    #[test]
    fn nullable_document_fields_decode_consistently() {
        for kind in [
            "paragraph",
            "heading",
            "button",
            "image",
            "bulleted_list",
            "title",
        ] {
            let result = serde_json::from_value::<EmailBlock>(
                json!({"id":"57","type":kind,"editor_content":{"attrs":null,"content":null,"mobile":null,"visibility":null}}),
            );
            assert!(result.is_ok(), "{kind}: {result:?}");
        }
    }

    #[test]
    fn null_block_documents_preserve_kind_and_default_children() {
        for kind in ["paragraph", "image", "future_block"] {
            let block: EmailBlock = serde_json::from_value(
                json!({"id":"57","type":kind,"editor_content":null,"children":null}),
            )
            .unwrap();
            assert!(block.children.is_empty());
            assert_eq!(block.editor_content, Value::Null);
            assert_eq!(block.block_type.as_str(), kind);
        }
    }

    #[test]
    fn empty_documents_and_unknown_block_variants_remain_usable() {
        for kind in ["paragraph", "future_block"] {
            for content in [json!({}), json!([]), json!({"opaque":[null,{"x":42}]})] {
                let block: EmailBlock = serde_json::from_value(
                    json!({"id":"001","type":kind,"editor_content":content}),
                )
                .unwrap();
                assert_eq!(block.editor_content, content);
                assert_eq!(block.block_type.as_str(), kind);
            }
        }
    }
}
