//! Subscriber automation execution events and automation identity.
use super::WebhookSubscriber;
use crate::models::AutomationId;
use serde::Deserialize;
use std::fmt;

/// Subscriber execution event and its automation.
#[derive(Clone, Deserialize)]
pub struct AutomationEvent {
    /// Subscriber that entered or completed the automation.
    pub subscriber: WebhookSubscriber,
    /// Affected automation identity.
    pub automation: WebhookAutomation,
    /// Account identifier, when included.
    pub account_id: Option<u64>,
}
impl fmt::Debug for AutomationEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AutomationEvent(<payload omitted>)")
    }
}

/// Automation identity included in execution events.
#[derive(Clone, Deserialize)]
pub struct WebhookAutomation {
    /// Stable automation identifier.
    pub id: AutomationId,
    /// Automation display name.
    pub name: String,
}
impl fmt::Debug for WebhookAutomation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WebhookAutomation(<payload omitted>)")
    }
}
