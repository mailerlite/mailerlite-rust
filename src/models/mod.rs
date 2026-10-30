//! Typed response models and shared values used by MailerLite resources.

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
pub use ids::*;
