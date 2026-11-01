//! Typed attributes for each editor block and inline formatting mark.
//!
//! Optional fields use `Patch` to preserve omission, explicit null, and values.
//! Images may be empty placeholders; media dimensions can be fractional numbers.
use super::choices::{
    BlockAlignment, ButtonSize, ButtonStyle, ButtonWidth, DividerStyle, FooterRole,
    SocialIconColor, SocialIconShape, SocialLinkType, TextAlignment, VideoPreview,
};
use super::wire::serialize_extensions;
use crate::models::common::Patch;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{Map, Value};

/// Attributes of blockquote content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockquoteAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `borderColor` editor attribute; omission and null are preserved separately.
    pub border_color: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Border thickness in the editor’s numeric representation.
    pub border_width: Patch<serde_json::Number>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Alignment of the text inside the block. Missing and null remain distinct.
    pub text_align: Patch<TextAlignment>,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "BlockquoteAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl BlockquoteAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(
            extra,
            &["borderColor", "borderWidth", "textAlign"],
            serializer,
        )
    }
}

/// Attributes of button content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Horizontal placement of the block.
    pub alignment: Patch<BlockAlignment>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `backgroundColor` editor attribute; omission and null are preserved separately.
    pub background_color: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Click-tracking ID, either a server number or a temporary editor string. Missing and null remain distinct.
    pub link_id: Patch<EditorNumberOrString>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Link category supplied by the editor; the API leaves this string open.
    pub link_type: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Whether to open the link in a new tab.
    pub open_new_tab: Patch<bool>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `size` editor attribute.
    pub size: Patch<ButtonSize>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `textColor` editor attribute; omission and null are preserved separately.
    pub text_color: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    #[serde(rename = "type")]
    /// The `type` editor attribute.
    pub kind: Patch<ButtonStyle>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Destination URL of the block or social link.
    pub url: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `width` editor attribute.
    pub width: Patch<ButtonWidth>,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "ButtonAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl ButtonAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(
            extra,
            &[
                "alignment",
                "backgroundColor",
                "linkId",
                "linkType",
                "openNewTab",
                "size",
                "textColor",
                "type",
                "url",
                "width",
            ],
            serializer,
        )
    }
}

/// Attributes of code content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Language label of the code block. Missing and null remain distinct.
    pub language: Patch<String>,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "CodeAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl CodeAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["language"], serializer)
    }
}

/// Attributes of divider content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DividerAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Horizontal placement of the block.
    pub alignment: Patch<BlockAlignment>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `color` editor attribute; omission and null are preserved separately.
    pub color: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `size` editor attribute.
    pub size: Patch<u64>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `style` editor attribute.
    pub style: Patch<DividerStyle>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `width` editor attribute.
    pub width: Patch<String>,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "DividerAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl DividerAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(
            extra,
            &["alignment", "color", "size", "style", "width"],
            serializer,
        )
    }
}

/// Attributes of heading content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadingAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Heading level, retaining its numeric or string representation.
    pub level: Patch<EditorNumberOrString>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Alignment of the text inside the block. Missing and null remain distinct.
    pub text_align: Patch<TextAlignment>,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "HeadingAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl HeadingAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["level", "textAlign"], serializer)
    }
}

/// Attributes of image content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Horizontal placement of the block.
    pub alignment: Patch<BlockAlignment>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Alternative text for the media.
    pub alt: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Caption displayed with the image.
    pub caption: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Role of this block in the footer. Missing and null remain distinct.
    pub footer_role: Patch<FooterRole>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Whether to expand the media to the full available width.
    pub full_width: Patch<bool>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Rendered height, including fractional resizing values; omission and null remain distinct.
    pub height: Patch<serde_json::Number>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Click-tracking ID, either a server number or a temporary editor string. Missing and null remain distinct.
    pub link_id: Patch<EditorNumberOrString>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Link category supplied by the editor; the API leaves this string open.
    pub link_type: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Whether to open the link in a new tab.
    pub open_new_tab: Patch<bool>,
    /// Source URL of the image; absent or null for an empty placeholder.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub src: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Destination URL of the block or social link.
    pub url: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Rendered width, including fractional resizing values; omission and null remain distinct.
    pub width: Patch<serde_json::Number>,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "ImageAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl ImageAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(
            extra,
            &[
                "alignment",
                "alt",
                "caption",
                "footerRole",
                "fullWidth",
                "height",
                "linkId",
                "linkType",
                "openNewTab",
                "src",
                "url",
                "width",
            ],
            serializer,
        )
    }
}
impl ImageAttributes {
    /// Construct an image with a source URL. Use `default()` for a placeholder.
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: Patch::Value(src.into()),
            ..Self::default()
        }
    }
}

/// Attributes of paragraph content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// CSS class supplied by the editor.
    pub class: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Role of this block in the footer. Missing and null remain distinct.
    pub footer_role: Patch<FooterRole>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Alignment of the text inside the block. Missing and null remain distinct.
    pub text_align: Patch<TextAlignment>,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "ParagraphAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl ParagraphAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["class", "footerRole", "textAlign"], serializer)
    }
}

/// Attributes of social icon content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialIconAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Horizontal placement of the block.
    pub alignment: Patch<BlockAlignment>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `color` editor attribute.
    pub color: Patch<SocialIconColor>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Role of this block in the footer. Missing and null remain distinct.
    pub footer_role: Patch<FooterRole>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Social icon links in display order.
    pub items: Patch<Vec<SocialIconItem>>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Shape of the social icons.
    pub shape: Patch<SocialIconShape>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `size` editor attribute.
    pub size: Patch<u64>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Whether social links open in a new tab.
    pub target_blank: Patch<bool>,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "SocialIconAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl SocialIconAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(
            extra,
            &[
                "alignment",
                "color",
                "footerRole",
                "items",
                "shape",
                "size",
                "targetBlank",
            ],
            serializer,
        )
    }
}

/// Attributes of video content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Horizontal placement of the block.
    pub alignment: Patch<BlockAlignment>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Alternative text for the media.
    pub alt: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// URL of the animated thumbnail.
    pub animated_thumbnail: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Embeddable video URL.
    pub embed_url: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Whether to expand the media to the full available width.
    pub full_width: Patch<bool>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Whether a thumbnail is already available.
    pub has_immediate_thumbnail: Patch<bool>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Rendered height, including fractional resizing values; omission and null remain distinct.
    pub height: Patch<serde_json::Number>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// URL of the static thumbnail.
    pub static_thumbnail: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Video identifier supplied by the provider.
    pub video_code: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Thumbnail rendering mode.
    pub video_preview: Patch<VideoPreview>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Video provider name; the API leaves this string open.
    pub video_source: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Original video URL.
    pub video_url: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Rendered width, including fractional resizing values; omission and null remain distinct.
    pub width: Patch<serde_json::Number>,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "VideoAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl VideoAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(
            extra,
            &[
                "alignment",
                "alt",
                "animatedThumbnail",
                "embedUrl",
                "fullWidth",
                "hasImmediateThumbnail",
                "height",
                "staticThumbnail",
                "videoCode",
                "videoPreview",
                "videoSource",
                "videoUrl",
                "width",
            ],
            serializer,
        )
    }
}

/// Attributes of link mark content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkMarkAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// CSS class supplied by the editor. Missing and null remain distinct.
    pub class: Patch<String>,
    /// Destination URL of the hyperlink.
    pub href: String,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Click-tracking ID; this mark uses the lowercase `linkid` wire spelling. Missing and null remain distinct.
    pub linkid: Patch<EditorNumberOrString>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Relationship tokens for the link. Missing and null remain distinct.
    pub rel: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Link target, such as `_blank`. Missing and null remain distinct.
    pub target: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Optional title of the hyperlink. Missing and null remain distinct.
    pub title: Patch<String>,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "LinkMarkAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl LinkMarkAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(
            extra,
            &["class", "href", "linkid", "rel", "target", "title"],
            serializer,
        )
    }
}
impl LinkMarkAttributes {
    /// Construct attributes with their required values.
    pub fn new(href: impl Into<String>) -> Self {
        Self {
            class: Patch::Unset,
            href: href.into(),
            linkid: Patch::Unset,
            rel: Patch::Unset,
            target: Patch::Unset,
            title: Patch::Unset,
            extra: Map::new(),
        }
    }
}

/// Attributes of text style mark content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStyleMarkAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `backgroundColor` editor attribute; omission and null are preserved separately.
    pub background_color: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `color` editor attribute; omission and null are preserved separately.
    pub color: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Font size in the editor’s string representation. Missing and null remain distinct.
    pub font_size: Patch<String>,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "TextStyleMarkAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl TextStyleMarkAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["backgroundColor", "color", "fontSize"], serializer)
    }
}

/// Attributes of variable content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableAttributes {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Text used when the personalization value is unavailable. Missing and null remain distinct.
    pub fallback: Patch<String>,
    /// Personalization variable name.
    pub name: String,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "VariableAttributes::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl VariableAttributes {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["fallback", "name"], serializer)
    }
}
impl VariableAttributes {
    /// Construct attributes with their required values.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            fallback: Patch::Unset,
            name: name.into(),
            extra: Map::new(),
        }
    }
}

/// Attributes of social icon item content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialIconItem {
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Accessible or displayed label of the social icon.
    pub label: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Click-tracking ID, either a server number or a temporary editor string. Missing and null remain distinct.
    pub link_id: Patch<EditorNumberOrString>,
    #[serde(rename = "type")]
    /// The `type` editor attribute.
    pub kind: SocialLinkType,
    /// Destination URL of the block or social link.
    pub url: String,
    /// Editor identifier of this social icon item.
    pub uuid: String,
    /// Additional attributes not yet modeled by the SDK.
    #[serde(flatten, serialize_with = "SocialIconItem::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl SocialIconItem {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(
            extra,
            &["label", "linkId", "type", "url", "uuid"],
            serializer,
        )
    }
}
impl SocialIconItem {
    /// Construct attributes with their required values.
    pub fn new(uuid: impl Into<String>, url: impl Into<String>, kind: SocialLinkType) -> Self {
        Self {
            label: Patch::Unset,
            link_id: Patch::Unset,
            kind,
            url: url.into(),
            uuid: uuid.into(),
            extra: Map::new(),
        }
    }
}

/// An editor number or string, retained without converting numeric IDs or heading levels.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditorNumberOrString {
    /// A JSON number; integer IDs retain their integer representation.
    Number(serde_json::Number),
    /// A string, including temporary link IDs such as `temp_example`.
    String(String),
}
