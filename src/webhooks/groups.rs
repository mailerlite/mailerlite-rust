//! Subscriber group membership events and group identity.
use super::WebhookSubscriber;
use crate::models::GroupId;
use serde::Deserialize;
use std::fmt;

/// Subscriber membership change and the affected group.
#[derive(Clone, Deserialize)]
pub struct GroupEvent {
    /// Subscriber whose membership changed.
    pub subscriber: WebhookSubscriber,
    /// Affected group identity.
    pub group: WebhookGroup,
    /// Account identifier, when included.
    pub account_id: Option<u64>,
}
impl fmt::Debug for GroupEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GroupEvent(<payload omitted>)")
    }
}

/// Group identity included in membership events.
#[derive(Clone, Deserialize)]
pub struct WebhookGroup {
    /// Stable group identifier.
    pub id: GroupId,
    /// Group display name.
    pub name: String,
}
impl fmt::Debug for WebhookGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WebhookGroup(<payload omitted>)")
    }
}
