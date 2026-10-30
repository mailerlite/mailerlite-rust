//! Webhook subscription models.
use crate::models::{SecretString, WebhookId};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A webhook event; unknown response values are retained.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WebhookEvent {
    /// A subscriber was created.
    SubscriberCreated,
    /// Subscriber custom fields or confirmation changed.
    SubscriberUpdated,
    /// A subscriber unsubscribed.
    SubscriberUnsubscribed,
    /// A subscriber joined a group.
    SubscriberAddedToGroup,
    /// A subscriber left a group.
    SubscriberRemovedFromGroup,
    /// A subscriber bounced.
    SubscriberBounced,
    /// An automation started for a subscriber.
    SubscriberAutomationTriggered,
    /// An automation completed for a subscriber.
    SubscriberAutomationCompleted,
    /// A subscriber reported spam.
    SubscriberSpamReported,
    /// A subscriber was deleted or forgotten.
    SubscriberDeleted,
    /// A subscriber became active.
    SubscriberActive,
    /// A campaign was sent.
    CampaignSent,
    /// A campaign link was clicked.
    CampaignClick,
    /// A campaign was opened.
    CampaignOpen,
    /// A future event returned by the service.
    Unknown(String),
}
impl WebhookEvent {
    /// Return the API event spelling.
    pub fn as_str(&self) -> &str {
        match self {
            Self::SubscriberCreated => "subscriber.created",
            Self::SubscriberUpdated => "subscriber.updated",
            Self::SubscriberUnsubscribed => "subscriber.unsubscribed",
            Self::SubscriberAddedToGroup => "subscriber.added_to_group",
            Self::SubscriberRemovedFromGroup => "subscriber.removed_from_group",
            Self::SubscriberBounced => "subscriber.bounced",
            Self::SubscriberAutomationTriggered => "subscriber.automation_triggered",
            Self::SubscriberAutomationCompleted => "subscriber.automation_completed",
            Self::SubscriberSpamReported => "subscriber.spam_reported",
            Self::SubscriberDeleted => "subscriber.deleted",
            Self::SubscriberActive => "subscriber.active",
            Self::CampaignSent => "campaign.sent",
            Self::CampaignClick => "campaign.click",
            Self::CampaignOpen => "campaign.open",
            Self::Unknown(value) => value,
        }
    }
    pub(crate) fn requires_batching(&self) -> bool {
        matches!(
            self,
            Self::SubscriberDeleted | Self::CampaignClick | Self::CampaignOpen
        )
    }
}
impl Serialize for WebhookEvent {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for WebhookEvent {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "subscriber.created" => Self::SubscriberCreated,
            "subscriber.updated" => Self::SubscriberUpdated,
            "subscriber.unsubscribed" => Self::SubscriberUnsubscribed,
            "subscriber.added_to_group" => Self::SubscriberAddedToGroup,
            "subscriber.removed_from_group" => Self::SubscriberRemovedFromGroup,
            "subscriber.bounced" => Self::SubscriberBounced,
            "subscriber.automation_triggered" => Self::SubscriberAutomationTriggered,
            "subscriber.automation_completed" => Self::SubscriberAutomationCompleted,
            "subscriber.spam_reported" => Self::SubscriberSpamReported,
            "subscriber.deleted" => Self::SubscriberDeleted,
            "subscriber.active" => Self::SubscriberActive,
            "campaign.sent" => Self::CampaignSent,
            "campaign.click" => Self::CampaignClick,
            "campaign.open" => Self::CampaignOpen,
            _ => Self::Unknown(value),
        })
    }
}
/// A registered webhook. Debug output redacts its signing secret.
#[derive(Debug, Clone, Deserialize)]
pub struct Webhook {
    /// Webhook identifier.
    pub id: WebhookId,
    /// Optional display name.
    pub name: Option<String>,
    /// Callback URL.
    pub url: String,
    /// Subscribed events.
    pub events: Vec<WebhookEvent>,
    /// Whether deliveries are enabled.
    pub enabled: bool,
    /// Secret used to authenticate incoming callbacks.
    pub secret: SecretString,
    /// Whether deliveries can be grouped.
    pub batchable: bool,
    /// Creation timestamp, as supplied by MailerLite.
    pub created_at: Option<String>,
    /// Update timestamp, as supplied by MailerLite.
    pub updated_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn unknown_response_event_is_preserved() {
        let event: WebhookEvent = serde_json::from_value(json!("future.event")).unwrap();
        assert_eq!(event.as_str(), "future.event");
    }
}
