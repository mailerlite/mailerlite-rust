//! Independent HTTP contract and boundary scenarios, organized by API resource.
#![allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]

mod support;

#[path = "resources/automation_actions.rs"]
mod automation_actions;
#[path = "resources/automation_reports.rs"]
mod automation_reports;
#[path = "resources/automation_steps.rs"]
mod automation_steps;
#[path = "resources/automation_templates.rs"]
mod automation_templates;
#[path = "resources/automation_triggers.rs"]
mod automation_triggers;
#[path = "resources/automations.rs"]
mod automations;
#[path = "resources/batch.rs"]
mod batch;
#[path = "resources/campaigns.rs"]
mod campaigns;
#[path = "resources/ecommerce/mod.rs"]
mod ecommerce;
#[path = "resources/fields.rs"]
mod fields;
#[path = "resources/forms.rs"]
mod forms;
#[path = "resources/groups.rs"]
mod groups;
#[path = "resources/imports.rs"]
mod imports;
#[path = "resources/reference.rs"]
mod reference;
#[path = "resources/segments.rs"]
mod segments;
#[path = "resources/subscribers.rs"]
mod subscribers;
#[path = "resources/webhooks.rs"]
mod webhooks;

#[path = "support/editor.rs"]
mod editor_support;

#[path = "resources/emails.rs"]
mod emails;

#[path = "resources/files.rs"]
mod files;

#[path = "resources/brand_styles.rs"]
mod brand_styles;

#[path = "resources/campaign_editor.rs"]
mod campaign_editor;
