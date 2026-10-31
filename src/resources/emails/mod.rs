//! Simple-editor email blocks, settings, templates, and versions.
mod operations;
mod params;
mod validation;
pub use operations::Emails;
pub use params::{
    BrandUpdate, ButtonUpdate, EmailSettingsUpdate, FooterUpdate, HeadingStyleUpdate, LayoutUpdate,
    LinkStyleUpdate, ListVersionsOptions, NewBlock, ParagraphStyleUpdate, Placement,
    SocialLinkUpdate, TextStyleUpdate, TypographyUpdate,
};
