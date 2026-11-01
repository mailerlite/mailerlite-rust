//! Simple-editor email documents, typed content, settings, and response envelopes.
mod attributes;
mod choices;
mod content;
mod models;
mod wire;
pub use attributes::{
    BlockquoteAttributes, ButtonAttributes, CodeAttributes, DividerAttributes,
    EditorNumberOrString, HeadingAttributes, ImageAttributes, LinkMarkAttributes,
    ParagraphAttributes, SocialIconAttributes, SocialIconItem, TextStyleMarkAttributes,
    VariableAttributes, VideoAttributes,
};
pub use choices::{
    BlockAlignment, BlockVisibility, ButtonSize, ButtonStyle, ButtonWidth, DividerStyle,
    EmailOwnerType, EmailTemplateType, EmailType, FooterRole, SocialIconColor, SocialIconShape,
    SocialLinkType, TextAlignment, VideoPreview,
};
pub(crate) use content::ContentRef;
pub use content::{
    BlockContent, BlockData, EmptyNode, InlineNode, InlineText, LinkMark, ListBlockData, ListItem,
    ListItemType, MediaBlockData, MobileOverrides, OpenAttributes, TextMark, TextStyleMark,
    TitleBlockData, VariableNode,
};
pub(crate) use models::DataOnly;
pub use models::{
    BlockPlacement, BlockType, Brand, BrandMode, Button, EditorText, EmailBlock, EmailDocument,
    EmailHtml, EmailResponseMetadata, EmailSettings, EmailTemplate, EmailVersion, EtagOnly,
    FontWeight, Footer, HeadingStyle, Layout, LayoutAlignment, LetterSpacing, LineHeight,
    LinkStyle, ParagraphStyle, Position, Rejected, SettingsResult, SocialLink, TextStyle,
    Typography, VERSION_CONFLICT, Validation, ValidationSchema, WidthMode, WithEtag,
};
