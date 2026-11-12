//! Simple-editor email documents, settings, and response envelopes.
mod choices;
mod models;
mod wire;
pub use choices::{BlockType, EmailOwnerType, EmailTemplateType, EmailType, SocialLinkType};
pub use models::{
    BlockPlacement, BlockWriteResult, Brand, BrandMode, Button, EditorText, EmailBlock,
    EmailDocument, EmailHtml, EmailResponseMetadata, EmailSettings, EmailTemplate, EmailVersion,
    EtagOnly, FontWeight, Footer, HeadingStyle, Layout, LayoutAlignment, LetterSpacing, LineHeight,
    LinkStyle, ParagraphStyle, Position, Rejected, SettingsResult, SocialLink, TextStyle,
    Typography, VERSION_CONFLICT, Validation, ValidationSchema, WidthMode, WithEtag, WriteResult,
};
