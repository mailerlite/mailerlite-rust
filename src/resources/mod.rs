//! Resource methods and request parameters.
pub mod automation_actions;
pub mod automation_steps;
pub mod automation_templates;
pub mod automation_triggers;
pub mod automations;
pub mod batch;
pub mod brand_styles;
pub mod campaigns;
pub mod ecommerce;
pub mod emails;
pub mod fields;
pub mod files;
pub mod forms;
pub mod groups;
pub mod imports;
pub mod reference;
pub mod segments;
pub mod subscribers;
pub mod webhooks;

mod accessors;
mod query;
mod sort_direction;

pub use sort_direction::SortDirection;
