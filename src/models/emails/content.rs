//! Typed block documents and inline nodes shared by requests and responses.
use super::attributes::{
    BlockquoteAttributes, ButtonAttributes, CodeAttributes, DividerAttributes, HeadingAttributes,
    ImageAttributes, LinkMarkAttributes, ParagraphAttributes, SocialIconAttributes,
    TextStyleMarkAttributes, VariableAttributes, VideoAttributes,
};
use super::choices::BlockVisibility;
use super::models::BlockType;
use super::wire::{self, Tagged, TaggedBlock, optional_value, serialize_extensions};
use crate::models::common::Patch;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

/// An editor document with attributes selected by its block type.
///
/// Missing fields stay omitted; `Some(Vec::new())` sends an explicit empty content
/// array. Block updates replace the whole document, so omission does not retain
/// the previous value. Use `Patch::Null` for nullable attributes when needed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "A: serde::de::DeserializeOwned"))]
pub struct BlockData<A> {
    /// Attributes belonging to this block kind.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub attrs: Option<A>,
    /// Ordered inline content; `Some(vec![])` explicitly clears it.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub content: Option<Vec<InlineNode>>,
    /// Mobile overrides; the API leaves these properties unrestricted for text blocks.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub mobile: Option<Map<String, Value>>,
    /// Devices on which this block is visible.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub visibility: Option<BlockVisibility>,
    /// Future editor fields, preserved independently of the typed fields.
    #[serde(flatten, serialize_with = "BlockData::<A>::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl<A> Default for BlockData<A> {
    fn default() -> Self {
        Self {
            attrs: None,
            content: None,
            mobile: None,
            visibility: None,
            extra: Map::new(),
        }
    }
}
impl<A> BlockData<A> {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(
            extra,
            &["attrs", "content", "mobile", "visibility"],
            serializer,
        )
    }
}
impl<A> BlockData<A> {
    /// Create a document containing one plain text node.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content: Some(vec![InlineNode::text(text)]),
            ..Self::default()
        }
    }
}

/// Document of an image, video, divider, or social-icon block.
/// Unknown properties remain in `extra`; fields without a schema are not guessed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "A: serde::de::DeserializeOwned, M: serde::de::DeserializeOwned"))]
pub struct MediaBlockData<A, M = MobileOverrides> {
    /// Attributes belonging to this block kind.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub attrs: Option<A>,
    /// Mobile overrides. Images and videos use [`MobileOverrides`]; divider and
    /// social-icon documents retain the API’s open JSON object.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub mobile: Option<M>,
    /// Devices on which the block is visible.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub visibility: Option<BlockVisibility>,
    /// Unrecognized document properties, retained without changing their values.
    #[serde(flatten, serialize_with = "MediaBlockData::<A, M>::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl<A, M> Default for MediaBlockData<A, M> {
    fn default() -> Self {
        Self {
            attrs: None,
            mobile: None,
            visibility: None,
            extra: Map::new(),
        }
    }
}
impl<A, M> MediaBlockData<A, M> {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["attrs", "mobile", "visibility"], serializer)
    }
}

/// Document of a bulleted or numbered list.
/// Unknown properties remain in `extra`; fields without a schema are not guessed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListBlockData {
    /// Attributes belonging to this block kind.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub attrs: Option<OpenAttributes>,
    /// Ordered editor nodes; an empty array explicitly clears the content.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub content: Option<Vec<ListItem>>,
    /// Unrecognized document properties, retained without changing their values.
    #[serde(flatten, serialize_with = "ListBlockData::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl ListBlockData {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["attrs", "content"], serializer)
    }
}

/// Document of the email title block.
/// Unknown properties remain in `extra`; fields without a schema are not guessed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TitleBlockData {
    /// Attributes belonging to this block kind.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub attrs: Option<OpenAttributes>,
    /// Ordered editor nodes; an empty array explicitly clears the content.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub content: Option<Vec<InlineNode>>,
    /// Unrecognized document properties, retained without changing their values.
    #[serde(flatten, serialize_with = "TitleBlockData::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl TitleBlockData {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["attrs", "content"], serializer)
    }
}

/// Mobile overrides. The API leaves additional override properties open.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MobileOverrides {
    /// Whether the block is hidden on mobile.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub hidden: Option<bool>,
    /// Other mobile properties permitted by the API.
    #[serde(flatten, serialize_with = "MobileOverrides::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl MobileOverrides {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["hidden"], serializer)
    }
}

/// Open list/title attributes. The API accepts either an object or an array.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAttributes {
    /// Named attributes; these have no fixed schema.
    Object(Map<String, Value>),
    /// The array representation accepted by the editor.
    Array(Vec<Value>),
}
impl Default for OpenAttributes {
    fn default() -> Self {
        Self::Object(Map::new())
    }
}

/// A text node with typed formatting marks.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InlineText {
    /// Text to render; missing and explicit null are distinct.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub text: Patch<String>,
    /// Formatting marks in editor order.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub marks: Option<Vec<TextMark>>,
    /// Unknown node properties, preserved for future editor versions.
    #[serde(flatten, serialize_with = "InlineText::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl InlineText {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["type", "text", "marks"], serializer)
    }
}

/// A personalization variable and optional fallback.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableNode {
    /// Typed attributes of this node.
    pub attrs: VariableAttributes,
    /// Unknown node properties, preserved for future editor versions.
    #[serde(flatten, serialize_with = "VariableNode::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl VariableNode {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["type", "attrs"], serializer)
    }
}
impl VariableNode {
    /// Construct the node with its required attributes.
    pub fn new(attrs: VariableAttributes) -> Self {
        Self {
            attrs,
            extra: Map::new(),
        }
    }
}

/// A node without known fields, preserving future properties.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmptyNode {
    /// Unknown node properties, preserved for future editor versions.
    #[serde(flatten, serialize_with = "EmptyNode::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl EmptyNode {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["type"], serializer)
    }
}

/// A hyperlink formatting mark.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkMark {
    /// Typed attributes of this node.
    pub attrs: LinkMarkAttributes,
    /// Unknown node properties, preserved for future editor versions.
    #[serde(flatten, serialize_with = "LinkMark::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl LinkMark {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["type", "attrs"], serializer)
    }
}
impl LinkMark {
    /// Construct the node with its required attributes.
    pub fn new(attrs: LinkMarkAttributes) -> Self {
        Self {
            attrs,
            extra: Map::new(),
        }
    }
}

/// Color and font-size formatting for text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextStyleMark {
    /// Typed attributes of this node.
    pub attrs: TextStyleMarkAttributes,
    /// Unknown node properties, preserved for future editor versions.
    #[serde(flatten, serialize_with = "TextStyleMark::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl TextStyleMark {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["type", "attrs"], serializer)
    }
}
impl TextStyleMark {
    /// Construct the node with its required attributes.
    pub fn new(attrs: TextStyleMarkAttributes) -> Self {
        Self {
            attrs,
            extra: Map::new(),
        }
    }
}

/// A list item. Its nested content is explicitly unrestricted by the API schema.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListItem {
    /// The list-item discriminator.
    #[serde(rename = "type")]
    pub node_type: ListItemType,
    /// Typed attributes of this node.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub attrs: Option<Map<String, Value>>,
    /// Nested list content; the API accepts arbitrary JSON values here.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optional_value"
    )]
    pub content: Option<Vec<Value>>,
    /// Unknown node properties, preserved for future editor versions.
    #[serde(flatten, serialize_with = "ListItem::serialize_extra")]
    pub extra: Map<String, Value>,
}
impl ListItem {
    fn serialize_extra<S: Serializer>(
        extra: &Map<String, Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serialize_extensions(extra, &["type", "attrs", "content"], serializer)
    }
}

/// The list-item wire discriminator.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ListItemType {
    /// A `listItem` node.
    #[default]
    #[serde(rename = "listItem")]
    ListItem,
}

/// Typed inline nodes.
/// Known variants reject malformed fields; unknown variants retain their JSON object.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum InlineNode {
    /// The `text` editor variant.
    Text(InlineText),
    /// The `variable` editor variant.
    Variable(VariableNode),
    /// The `hardBreak` editor variant.
    HardBreak(EmptyNode),
    /// A future editor variant. Its type must not name a known variant.
    Other {
        /// Unrecognized wire discriminator.
        kind: String,
        /// Unrecognized node fields, excluding `type`.
        fields: Map<String, Value>,
    },
}
impl<'de> Deserialize<'de> for InlineNode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (kind, fields) = wire::tagged_object(deserializer)?;
        let data = fields;
        match kind.as_str() {
            "text" => serde_json::from_value(Value::Object(data))
                .map(Self::Text)
                .map_err(serde::de::Error::custom),
            "variable" => serde_json::from_value(Value::Object(data))
                .map(Self::Variable)
                .map_err(serde::de::Error::custom),
            "hardBreak" => serde_json::from_value(Value::Object(data))
                .map(Self::HardBreak)
                .map_err(serde::de::Error::custom),
            _ => Ok(Self::Other { kind, fields: data }),
        }
    }
}
impl Serialize for InlineNode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text(data) => Tagged {
                kind: "text",
                fields: data,
            }
            .serialize(serializer),
            Self::Variable(data) => Tagged {
                kind: "variable",
                fields: data,
            }
            .serialize(serializer),
            Self::HardBreak(data) => Tagged {
                kind: "hardBreak",
                fields: data,
            }
            .serialize(serializer),
            Self::Other { kind, fields } => {
                if matches!(kind.as_str(), "text" | "variable" | "hardBreak")
                    || fields.contains_key("type")
                {
                    return Err(serde::ser::Error::custom(
                        "known editor variants must use their typed representation",
                    ));
                }
                Tagged { kind, fields }.serialize(serializer)
            }
        }
    }
}

/// Typed inline formatting marks.
/// Known variants reject malformed fields; unknown variants retain their JSON object.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum TextMark {
    /// The `bold` editor variant.
    Bold(EmptyNode),
    /// The `italic` editor variant.
    Italic(EmptyNode),
    /// The `underline` editor variant.
    Underline(EmptyNode),
    /// The `strike` editor variant.
    Strike(EmptyNode),
    /// The `link` editor variant.
    Link(LinkMark),
    /// The `textStyle` editor variant.
    TextStyle(TextStyleMark),
    /// A future editor variant. Its type must not name a known variant.
    Other {
        /// Unrecognized wire discriminator.
        kind: String,
        /// Unrecognized node fields, excluding `type`.
        fields: Map<String, Value>,
    },
}
impl<'de> Deserialize<'de> for TextMark {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (kind, fields) = wire::tagged_object(deserializer)?;
        let data = fields;
        match kind.as_str() {
            "bold" => serde_json::from_value(Value::Object(data))
                .map(Self::Bold)
                .map_err(serde::de::Error::custom),
            "italic" => serde_json::from_value(Value::Object(data))
                .map(Self::Italic)
                .map_err(serde::de::Error::custom),
            "underline" => serde_json::from_value(Value::Object(data))
                .map(Self::Underline)
                .map_err(serde::de::Error::custom),
            "strike" => serde_json::from_value(Value::Object(data))
                .map(Self::Strike)
                .map_err(serde::de::Error::custom),
            "link" => serde_json::from_value(Value::Object(data))
                .map(Self::Link)
                .map_err(serde::de::Error::custom),
            "textStyle" => serde_json::from_value(Value::Object(data))
                .map(Self::TextStyle)
                .map_err(serde::de::Error::custom),
            _ => Ok(Self::Other { kind, fields: data }),
        }
    }
}
impl Serialize for TextMark {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bold(data) => Tagged {
                kind: "bold",
                fields: data,
            }
            .serialize(serializer),
            Self::Italic(data) => Tagged {
                kind: "italic",
                fields: data,
            }
            .serialize(serializer),
            Self::Underline(data) => Tagged {
                kind: "underline",
                fields: data,
            }
            .serialize(serializer),
            Self::Strike(data) => Tagged {
                kind: "strike",
                fields: data,
            }
            .serialize(serializer),
            Self::Link(data) => Tagged {
                kind: "link",
                fields: data,
            }
            .serialize(serializer),
            Self::TextStyle(data) => Tagged {
                kind: "textStyle",
                fields: data,
            }
            .serialize(serializer),
            Self::Other { kind, fields } => {
                if matches!(
                    kind.as_str(),
                    "bold" | "italic" | "underline" | "strike" | "link" | "textStyle"
                ) || fields.contains_key("type")
                {
                    return Err(serde::ser::Error::custom(
                        "known editor variants must use their typed representation",
                    ));
                }
                Tagged { kind, fields }.serialize(serializer)
            }
        }
    }
}

/// Block content selected by the API block discriminator.
/// Known variants reject malformed fields; unknown variants retain their JSON object.
///
/// # Examples
/// ```
/// use mailerlite_rs::{
///     models::{common::Patch, emails::{BlockContent, BlockData, ParagraphAttributes, TextAlignment}},
///     resources::emails::NewBlock,
/// };
///
/// let content = BlockContent::Paragraph(BlockData {
///     attrs: Some(ParagraphAttributes {
///         text_align: Patch::Value(TextAlignment::Center),
///         ..Default::default()
///     }),
///     ..BlockData::text("Hello")
/// });
/// let block = NewBlock::new(content);
/// assert_eq!(serde_json::to_value(block)?["type"], "paragraph");
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum BlockContent {
    /// The `blockquote` editor variant.
    Blockquote(BlockData<BlockquoteAttributes>),
    /// The `bulleted_list` editor variant.
    BulletedList(ListBlockData),
    /// The `button` editor variant.
    Button(BlockData<ButtonAttributes>),
    /// The `code` editor variant.
    Code(BlockData<CodeAttributes>),
    /// The `divider` editor variant.
    Divider(MediaBlockData<DividerAttributes, Map<String, Value>>),
    /// The `heading` editor variant.
    Heading(BlockData<HeadingAttributes>),
    /// The `image` editor variant.
    Image(MediaBlockData<ImageAttributes>),
    /// The `numbered_list` editor variant.
    NumberedList(ListBlockData),
    /// The `paragraph` editor variant.
    Paragraph(BlockData<ParagraphAttributes>),
    /// The `social_icon` editor variant.
    SocialIcon(MediaBlockData<SocialIconAttributes, Map<String, Value>>),
    /// The `title` editor variant.
    Title(TitleBlockData),
    /// The `video` editor variant.
    Video(MediaBlockData<VideoAttributes>),
    /// A future editor variant. Its type must not name a known variant.
    Other {
        /// Unrecognized wire discriminator.
        kind: String,
        /// Unrecognized editor document.
        fields: Map<String, Value>,
    },
}
impl<'de> Deserialize<'de> for BlockContent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (kind, fields) = wire::tagged_object(deserializer)?;
        let mut fields = fields;
        let data = fields
            .remove("editor_content")
            .unwrap_or_else(|| Value::Object(Map::new()));
        let data = wire::object(data).map_err(serde::de::Error::custom)?;
        match kind.as_str() {
            "blockquote" => serde_json::from_value(Value::Object(data))
                .map(Self::Blockquote)
                .map_err(serde::de::Error::custom),
            "bulleted_list" => serde_json::from_value(Value::Object(data))
                .map(Self::BulletedList)
                .map_err(serde::de::Error::custom),
            "button" => serde_json::from_value(Value::Object(data))
                .map(Self::Button)
                .map_err(serde::de::Error::custom),
            "code" => serde_json::from_value(Value::Object(data))
                .map(Self::Code)
                .map_err(serde::de::Error::custom),
            "divider" => serde_json::from_value(Value::Object(data))
                .map(Self::Divider)
                .map_err(serde::de::Error::custom),
            "heading" => serde_json::from_value(Value::Object(data))
                .map(Self::Heading)
                .map_err(serde::de::Error::custom),
            "image" => serde_json::from_value(Value::Object(data))
                .map(Self::Image)
                .map_err(serde::de::Error::custom),
            "numbered_list" => serde_json::from_value(Value::Object(data))
                .map(Self::NumberedList)
                .map_err(serde::de::Error::custom),
            "paragraph" => serde_json::from_value(Value::Object(data))
                .map(Self::Paragraph)
                .map_err(serde::de::Error::custom),
            "social_icon" => serde_json::from_value(Value::Object(data))
                .map(Self::SocialIcon)
                .map_err(serde::de::Error::custom),
            "title" => serde_json::from_value(Value::Object(data))
                .map(Self::Title)
                .map_err(serde::de::Error::custom),
            "video" => serde_json::from_value(Value::Object(data))
                .map(Self::Video)
                .map_err(serde::de::Error::custom),
            _ => Ok(Self::Other { kind, fields: data }),
        }
    }
}
impl Serialize for BlockContent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Blockquote(data) => TaggedBlock {
                kind: "blockquote",
                editor_content: data,
            }
            .serialize(serializer),
            Self::BulletedList(data) => TaggedBlock {
                kind: "bulleted_list",
                editor_content: data,
            }
            .serialize(serializer),
            Self::Button(data) => TaggedBlock {
                kind: "button",
                editor_content: data,
            }
            .serialize(serializer),
            Self::Code(data) => TaggedBlock {
                kind: "code",
                editor_content: data,
            }
            .serialize(serializer),
            Self::Divider(data) => TaggedBlock {
                kind: "divider",
                editor_content: data,
            }
            .serialize(serializer),
            Self::Heading(data) => TaggedBlock {
                kind: "heading",
                editor_content: data,
            }
            .serialize(serializer),
            Self::Image(data) => TaggedBlock {
                kind: "image",
                editor_content: data,
            }
            .serialize(serializer),
            Self::NumberedList(data) => TaggedBlock {
                kind: "numbered_list",
                editor_content: data,
            }
            .serialize(serializer),
            Self::Paragraph(data) => TaggedBlock {
                kind: "paragraph",
                editor_content: data,
            }
            .serialize(serializer),
            Self::SocialIcon(data) => TaggedBlock {
                kind: "social_icon",
                editor_content: data,
            }
            .serialize(serializer),
            Self::Title(data) => TaggedBlock {
                kind: "title",
                editor_content: data,
            }
            .serialize(serializer),
            Self::Video(data) => TaggedBlock {
                kind: "video",
                editor_content: data,
            }
            .serialize(serializer),
            Self::Other { kind, fields } => {
                if matches!(
                    kind.as_str(),
                    "blockquote"
                        | "bulleted_list"
                        | "button"
                        | "code"
                        | "divider"
                        | "heading"
                        | "image"
                        | "numbered_list"
                        | "paragraph"
                        | "social_icon"
                        | "title"
                        | "video"
                ) {
                    return Err(serde::ser::Error::custom(
                        "known editor variants must use their typed representation",
                    ));
                }
                TaggedBlock {
                    kind,
                    editor_content: fields,
                }
                .serialize(serializer)
            }
        }
    }
}

impl InlineNode {
    /// Construct a plain text node without formatting marks.
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(InlineText {
            text: Patch::Value(text.into()),
            ..InlineText::default()
        })
    }
}
impl BlockContent {
    /// Construct a paragraph with one plain text node.
    pub fn paragraph(text: impl Into<String>) -> Self {
        Self::Paragraph(BlockData::text(text))
    }
    /// Return the API discriminator associated with this typed document.
    pub fn block_type(&self) -> BlockType {
        match self {
            Self::Blockquote(_) => BlockType::Blockquote,
            Self::BulletedList(_) => BlockType::BulletedList,
            Self::Button(_) => BlockType::Button,
            Self::Code(_) => BlockType::Code,
            Self::Divider(_) => BlockType::Divider,
            Self::Heading(_) => BlockType::Heading,
            Self::Image(_) => BlockType::Image,
            Self::NumberedList(_) => BlockType::NumberedList,
            Self::Paragraph(_) => BlockType::Paragraph,
            Self::SocialIcon(_) => BlockType::SocialIcon,
            Self::Title(_) => BlockType::Title,
            Self::Video(_) => BlockType::Video,
            Self::Other { kind, .. } => BlockType::from(kind.clone()),
        }
    }
}

/// Serialize only the document when updating an existing block.
pub(crate) struct ContentRef<'a>(pub(crate) &'a BlockContent);
impl Serialize for ContentRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            BlockContent::Blockquote(data) => data.serialize(serializer),
            BlockContent::BulletedList(data) => data.serialize(serializer),
            BlockContent::Button(data) => data.serialize(serializer),
            BlockContent::Code(data) => data.serialize(serializer),
            BlockContent::Divider(data) => data.serialize(serializer),
            BlockContent::Heading(data) => data.serialize(serializer),
            BlockContent::Image(data) => data.serialize(serializer),
            BlockContent::NumberedList(data) => data.serialize(serializer),
            BlockContent::Paragraph(data) => data.serialize(serializer),
            BlockContent::SocialIcon(data) => data.serialize(serializer),
            BlockContent::Title(data) => data.serialize(serializer),
            BlockContent::Video(data) => data.serialize(serializer),
            BlockContent::Other { kind, fields } => {
                if !matches!(BlockType::from(kind.clone()), BlockType::Other(_)) {
                    return Err(serde::ser::Error::custom(
                        "known blocks must use typed content",
                    ));
                }
                fields.serialize(serializer)
            }
        }
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert typed editor wire contracts"
)]
mod tests {
    use super::*;
    use crate::models::emails::{EmailBlock, EmailDocument, TextAlignment, WithEtag};
    use serde_json::json;

    #[test]
    fn every_block_kind_decodes_and_round_trips_its_attributes() {
        let cases = [
            (
                "blockquote",
                json!({"borderColor":null,"borderWidth":2,"textAlign":"justify"}),
            ),
            ("bulleted_list", json!([])),
            (
                "button",
                json!({"alignment":"center","backgroundColor":null,"linkId":"temp_button","linkType":"url","openNewTab":true,"size":"medium","textColor":"#ffffff","type":"primary","url":"https://example.test","width":"auto"}),
            ),
            ("code", json!({"language":null})),
            (
                "divider",
                json!({"alignment":"left","color":null,"size":1,"style":"dashed","width":"50%"}),
            ),
            ("heading", json!({"level":2,"textAlign":null})),
            (
                "image",
                json!({"alignment":"right","alt":"Photo","caption":"Caption","footerRole":null,"fullWidth":false,"height":200,"width":300,"linkId":9007199254740991_u64,"linkType":"url","openNewTab":false,"src":"https://example.test/a.png","url":"https://example.test"}),
            ),
            ("numbered_list", json!({"future":true})),
            (
                "paragraph",
                json!({"class":"intro","footerRole":"companyInfo","textAlign":"left"}),
            ),
            (
                "social_icon",
                json!({"alignment":"center","color":"color","footerRole":"social","items":[{"uuid":"item-1","url":"https://example.test","type":"website","linkId":null,"label":"Website"}],"shape":"round_stroke","size":24,"targetBlank":true}),
            ),
            ("title", json!([])),
            (
                "video",
                json!({"alignment":"center","alt":"Video","animatedThumbnail":"https://example.test/a.gif","embedUrl":"https://example.test/embed","fullWidth":true,"hasImmediateThumbnail":false,"height":null,"staticThumbnail":"https://example.test/a.png","videoCode":"abc","videoPreview":"animated","videoSource":"youtube","videoUrl":"https://example.test/video","width":640}),
            ),
        ];
        for (kind, attrs) in cases {
            let wire = json!({"type":kind,"editor_content":{"attrs":attrs}});
            let parsed: BlockContent = serde_json::from_value(wire.clone()).unwrap();
            assert_eq!(parsed.block_type().as_str(), kind);
            assert_eq!(serde_json::to_value(parsed).unwrap(), wire, "{kind}");
        }
    }

    #[test]
    fn real_document_shape_exposes_typed_fields() {
        let email: WithEtag<EmailDocument> =
            serde_json::from_str(include_str!("../../../tests/fixtures/email.json")).unwrap();
        let BlockContent::Paragraph(data) = &email.data.blocks[0].content else {
            panic!("expected paragraph")
        };
        assert_eq!(
            data.attrs.as_ref().unwrap().text_align,
            Patch::Value(TextAlignment::Left)
        );
        let InlineNode::Text(text) = &data.content.as_ref().unwrap()[0] else {
            panic!("expected text")
        };
        assert_eq!(text.text, Patch::Value("Hello".into()));
    }

    #[test]
    fn inline_nodes_and_marks_preserve_known_and_unknown_properties() {
        let wire = json!({"type":"paragraph","editor_content":{
            "attrs":{"textAlign":"future-alignment","futureAttribute":7},
            "content":[
                {"type":"text","text":null,"marks":[
                    {"type":"bold","futureFlag":true}, {"type":"italic"},
                    {"type":"underline"}, {"type":"strike"},
                    {"type":"link","attrs":{"href":"https://example.test","linkid":"temp_link","target":null}},
                    {"type":"textStyle","attrs":{"fontSize":"18px","backgroundColor":null}},
                    {"type":"futureMark","attrs":{"opaque":[1,2]}}
                ]},
                {"type":"variable","attrs":{"name":"name","fallback":"friend"}},
                {"type":"hardBreak"},
                {"type":"futureNode","data":{"nested":true}}
            ],
            "visibility":"mobile", "mobile":{"hidden":false,"future":null}, "futureDocument":[1]
        }});
        let parsed: BlockContent = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
    }

    #[test]
    fn malformed_known_variants_do_not_fall_back_to_raw_json() {
        for content in [
            json!({"attrs":{"textAlign":3}}),
            json!({"attrs":[]}),
            json!({"attrs":null}),
            json!({"content":null}),
            json!({"content":[{"type":"text","text":42}]}),
            json!({"content":[{"type":"variable","attrs":{}}]}),
            json!({"content":[{"type":"text","marks":[{"type":"link","attrs":{}}]}]}),
            json!({"content":[{"type":"text","marks":[{"type":"textStyle","attrs":{"color":false}}]}]}),
            json!({"content":[{"type":42}]}),
        ] {
            assert!(
                serde_json::from_value::<BlockContent>(
                    json!({"type":"paragraph","editor_content":content})
                )
                .is_err()
            );
        }
        assert!(
            serde_json::from_value::<BlockContent>(
                json!({"type":"image","editor_content":{"attrs":{"src":"https://example.test/a.png","width":"200"}}})
            )
            .is_err()
        );
        assert!(serde_json::from_value::<BlockContent>(json!({"type":"social_icon","editor_content":{"attrs":{"items":[{"type":"website"}]}}})).is_err());
    }

    #[test]
    fn empty_documents_and_unknown_block_variants_remain_usable() {
        for kind in ["paragraph", "future_block"] {
            for empty in [json!({}), json!([])] {
                let parsed: EmailBlock =
                    serde_json::from_value(json!({"id":"001","type":kind,"editor_content":empty}))
                        .unwrap();
                assert_eq!(
                    serde_json::to_value(parsed.content).unwrap(),
                    json!({"type":kind,"editor_content":{}})
                );
            }
        }
        let wire = json!({"type":"future_block","editor_content":{"opaque":[null,{"x":42}]}});
        let parsed: BlockContent = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
    }

    #[test]
    fn omitted_null_and_empty_values_keep_their_wire_meanings() {
        for data in [
            json!({}),
            json!({"content":[]}),
            json!({"attrs":{}}),
            json!({"attrs":{"textAlign":null}}),
        ] {
            let wire = json!({"type":"paragraph","editor_content":data});
            let parsed: BlockContent = serde_json::from_value(wire.clone()).unwrap();
            assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
        }
        for level in [json!("2"), json!(2), json!(2.5)] {
            let wire = json!({"type":"heading","editor_content":{"attrs":{"level":level}}});
            let parsed: BlockContent = serde_json::from_value(wire.clone()).unwrap();
            assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
        }
    }

    #[test]
    fn extension_fields_cannot_override_typed_fields_or_discriminators() {
        let mut data = BlockData::<ParagraphAttributes>::default();
        data.extra.insert("content".into(), json!([]));
        assert!(serde_json::to_value(BlockContent::Paragraph(data)).is_err());
        let mut attrs = ParagraphAttributes::default();
        attrs.extra.insert("textAlign".into(), json!("left"));
        assert!(serde_json::to_value(attrs).is_err());
        assert!(
            serde_json::to_value(InlineNode::Other {
                kind: "text".into(),
                fields: Map::new()
            })
            .is_err()
        );
        let spoof = BlockContent::Other {
            kind: "image".into(),
            fields: Map::new(),
        };
        assert!(serde_json::to_value(&spoof).is_err());
        assert!(serde_json::to_value(ContentRef(&spoof)).is_err());
    }

    #[test]
    fn properties_outside_each_block_schema_stay_open() {
        for wire in [
            json!({"type":"image","editor_content":{"content":{"future":"shape"}}}),
            json!({"type":"paragraph","editor_content":{"mobile":{"hidden":{"future":true}}}}),
            json!({"type":"divider","editor_content":{"mobile":{"hidden":"future"}}}),
            json!({"type":"bulleted_list","editor_content":{"mobile":[1,2],"visibility":null}}),
            json!({"type":"title","editor_content":{"visibility":{"future":true}}}),
        ] {
            let parsed: BlockContent = serde_json::from_value(wire.clone()).unwrap();
            assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
        }
    }

    #[test]
    fn list_items_preserve_the_schema_open_content() {
        let wire = json!({"type":"bulleted_list","editor_content":{"attrs":[],"content":[{"type":"listItem","attrs":{"open":true},"content":[null,{"type":"paragraph","content":[]}]}]}});
        let parsed: BlockContent = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
        assert!(
            serde_json::from_value::<BlockContent>(
                json!({"type":"numbered_list","editor_content":{"content":[{"type":"paragraph"}]}})
            )
            .is_err()
        );
    }
}
