//! Typed response models and shared values used by MailerLite resources.
//!
//! Optional response fields may be absent or null. `None` does not mean zero,
//! false, or an empty collection. Timestamp strings retain the server's format;
//! the SDK does not infer a timezone for timestamps without one. Monetary values
//! use exact decimals, while rate models document their numeric and display units.
//!
//! Server-owned resource, report, activity, and enriched configuration records
//! are non-exhaustive so future SDK versions can expose additional fields.
//! Read fields directly or destructure with `..`; deserialize sanitized JSON
//! to construct response fixtures outside this crate.
//!
//! Request configuration, fixed scalar representations, and small reusable
//! envelopes remain constructible. Examples include [`Patch`],
//! [`subscribers::FieldValue`], [`imports::ImportReceipt`], and batch results.
//! Non-exhaustiveness affects Rust construction and matching, not JSON decoding:
//! unknown server values still follow each model's documented Serde behavior.

pub mod automation_configuration;
pub mod automation_reports;
pub mod automation_steps;
pub mod automation_templates;
pub mod automation_triggers;
pub mod automations;
pub mod batch;
pub mod campaigns;
pub mod common;
pub mod ecommerce;
pub mod fields;
pub mod forms;
pub mod groups;
pub mod ids;
pub mod imports;
pub mod reference;
pub mod segments;
pub mod subscriber_activity;
pub mod subscribers;
pub mod webhooks;

pub use common::{Patch, SecretString};
pub use ids::{
    AutomationId, CampaignId, CartId, CartItemId, CategoryId, CustomerId, FieldId, FormId, GroupId,
    ImportId, OrderId, ProductId, SegmentId, ShopId, SubscriberId, WebhookId,
};

/// Emails API types.
pub mod emails;

/// Files API types.
pub mod files;

/// Brand styles API types.
pub mod brand_styles;

mod open_enum;
