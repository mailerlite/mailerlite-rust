//! Typed attributes for each editor block and inline formatting mark.
//!
//! Optional fields decode missing values and JSON null as `None`. Fields using
//! `Patch` retain the distinction between omission and an explicit null.
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Border thickness in the editor’s numeric representation.
    pub border_width: Option<serde_json::Number>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Horizontal placement of the block.
    pub alignment: Option<BlockAlignment>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `backgroundColor` editor attribute; omission and null are preserved separately.
    pub background_color: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Click-tracking ID, either a server number or a temporary editor string. Missing and null remain distinct.
    pub link_id: Patch<EditorNumberOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Link category supplied by the editor; the API leaves this string open.
    pub link_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Whether to open the link in a new tab.
    pub open_new_tab: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The `size` editor attribute.
    pub size: Option<ButtonSize>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `textColor` editor attribute; omission and null are preserved separately.
    pub text_color: Patch<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    /// The `type` editor attribute.
    pub kind: Option<ButtonStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Destination URL of the block or social link.
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The `width` editor attribute.
    pub width: Option<ButtonWidth>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Horizontal placement of the block.
    pub alignment: Option<BlockAlignment>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// The `color` editor attribute; omission and null are preserved separately.
    pub color: Patch<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The `size` editor attribute.
    pub size: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The `style` editor attribute.
    pub style: Option<DividerStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The `width` editor attribute.
    pub width: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Heading level, retaining its numeric or string representation.
    pub level: Option<EditorNumberOrString>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Horizontal placement of the block.
    pub alignment: Option<BlockAlignment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Alternative text for the media.
    pub alt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Caption displayed with the image.
    pub caption: Option<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Role of this block in the footer. Missing and null remain distinct.
    pub footer_role: Patch<FooterRole>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Whether to expand the media to the full available width.
    pub full_width: Option<bool>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Rendered height, including fractional resizing values; omission and null remain distinct.
    pub height: Patch<serde_json::Number>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Click-tracking ID, either a server number or a temporary editor string. Missing and null remain distinct.
    pub link_id: Patch<EditorNumberOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Link category supplied by the editor; the API leaves this string open.
    pub link_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Whether to open the link in a new tab.
    pub open_new_tab: Option<bool>,
    /// Source URL of the image; absent or null for an empty placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Destination URL of the block or social link.
    pub url: Option<String>,
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
            src: Some(src.into()),
            ..Self::default()
        }
    }
}

/// Attributes of paragraph content.
/// Unknown fields are preserved in `extra`; known field names cannot be inserted there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// CSS class supplied by the editor.
    pub class: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Horizontal placement of the block.
    pub alignment: Option<BlockAlignment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The `color` editor attribute.
    pub color: Option<SocialIconColor>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Role of this block in the footer. Missing and null remain distinct.
    pub footer_role: Patch<FooterRole>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Social icon links in display order.
    pub items: Option<Vec<SocialIconItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Shape of the social icons.
    pub shape: Option<SocialIconShape>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The `size` editor attribute.
    pub size: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Whether social links open in a new tab.
    pub target_blank: Option<bool>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Horizontal placement of the block.
    pub alignment: Option<BlockAlignment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Alternative text for the media.
    pub alt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// URL of the animated thumbnail.
    pub animated_thumbnail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Embeddable video URL.
    pub embed_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Whether to expand the media to the full available width.
    pub full_width: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Whether a thumbnail is already available.
    pub has_immediate_thumbnail: Option<bool>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    /// Rendered height, including fractional resizing values; omission and null remain distinct.
    pub height: Patch<serde_json::Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// URL of the static thumbnail.
    pub static_thumbnail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Video identifier supplied by the provider.
    pub video_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Thumbnail rendering mode.
    pub video_preview: Option<VideoPreview>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Video provider name; the API leaves this string open.
    pub video_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Original video URL.
    pub video_url: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Accessible or displayed label of the social icon.
    pub label: Option<String>,
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
            label: None,
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
