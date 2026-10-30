//! Event-name dispatch and explicit unknown-event payloads.
use super::{
    AutomationEvent, CampaignClickEvent, CampaignOpenEvent, CampaignSentEvent,
    DeletedSubscriberEvent, GroupEvent, SubscriberEvent,
};
use serde::{Deserialize, Deserializer, de::Error as _};
use serde_json::Value;
use std::fmt;

/// An incoming event. This differs from the subscription enum in `models::webhooks`.
///
/// Payloads are boxed to keep the enum compact. Known names always decode their
/// concrete schema; a malformed known event never becomes [`Self::Unknown`].
#[derive(Clone)]
#[non_exhaustive]
pub enum WebhookEvent {
    /// A subscriber was created.
    SubscriberCreated(Box<SubscriberEvent>),
    /// Subscriber fields or confirmation changed.
    SubscriberUpdated(Box<SubscriberEvent>),
    /// A subscriber unsubscribed.
    SubscriberUnsubscribed(Box<SubscriberEvent>),
    /// A subscriber became active.
    SubscriberActive(Box<SubscriberEvent>),
    /// A subscriber bounced.
    SubscriberBounced(Box<SubscriberEvent>),
    /// A subscriber reported spam.
    SubscriberSpamReported(Box<SubscriberEvent>),
    /// A subscriber was deleted or forgotten.
    SubscriberDeleted(Box<DeletedSubscriberEvent>),
    /// A subscriber joined a group.
    SubscriberAddedToGroup(Box<GroupEvent>),
    /// A subscriber left a group.
    SubscriberRemovedFromGroup(Box<GroupEvent>),
    /// A subscriber started an automation.
    SubscriberAutomationTriggered(Box<AutomationEvent>),
    /// A subscriber completed an automation.
    SubscriberAutomationCompleted(Box<AutomationEvent>),
    /// A campaign was sent.
    CampaignSent(Box<CampaignSentEvent>),
    /// A subscriber opened a campaign.
    CampaignOpen(Box<CampaignOpenEvent>),
    /// A subscriber clicked a campaign link.
    CampaignClick(Box<CampaignClickEvent>),
    /// A future or deployment-specific event with no documented public schema.
    Unknown(UnknownEvent),
}
impl WebhookEvent {
    /// Return the event name as supplied by MailerLite.
    pub fn name(&self) -> &str {
        match self {
            Self::SubscriberCreated(_) => "subscriber.created",
            Self::SubscriberUpdated(_) => "subscriber.updated",
            Self::SubscriberUnsubscribed(_) => "subscriber.unsubscribed",
            Self::SubscriberActive(_) => "subscriber.active",
            Self::SubscriberBounced(_) => "subscriber.bounced",
            Self::SubscriberSpamReported(_) => "subscriber.spam_reported",
            Self::SubscriberDeleted(_) => "subscriber.deleted",
            Self::SubscriberAddedToGroup(_) => "subscriber.added_to_group",
            Self::SubscriberRemovedFromGroup(_) => "subscriber.removed_from_group",
            Self::SubscriberAutomationTriggered(_) => "subscriber.automation_triggered",
            Self::SubscriberAutomationCompleted(_) => "subscriber.automation_completed",
            Self::CampaignSent(_) => "campaign.sent",
            Self::CampaignOpen(_) => "campaign.open",
            Self::CampaignClick(_) => "campaign.click",
            Self::Unknown(event) => event.name(),
        }
    }
}
impl fmt::Debug for WebhookEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(_) => f.write_str("Unknown(<payload omitted>)"),
            _ => f
                .debug_tuple(self.name())
                .field(&"<payload omitted>")
                .finish(),
        }
    }
}
impl<'de> Deserialize<'de> for WebhookEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let object = value
            .as_object()
            .ok_or_else(|| D::Error::custom("webhook event must be an object"))?;
        let (key, name) = match (object.get("event"), object.get("type")) {
            (Some(_), Some(_)) => {
                return Err(D::Error::custom(
                    "webhook event has ambiguous discriminators",
                ));
            }
            (Some(name), None) => ("event", name),
            (None, Some(name)) => ("type", name),
            (None, None) => {
                return Err(D::Error::custom(
                    "webhook event is missing its discriminator",
                ));
            }
        };
        let name = name
            .as_str()
            .ok_or_else(|| D::Error::custom("webhook event discriminator must be text"))?
            .to_owned();
        let expected_key = match name.as_str() {
            "subscriber.created"
            | "subscriber.updated"
            | "subscriber.unsubscribed"
            | "subscriber.active"
            | "subscriber.bounced"
            | "subscriber.spam_reported"
            | "campaign.sent" => Some("event"),
            "subscriber.deleted"
            | "subscriber.added_to_group"
            | "subscriber.removed_from_group"
            | "subscriber.automation_triggered"
            | "subscriber.automation_completed"
            | "campaign.open"
            | "campaign.click" => Some("type"),
            _ => None,
        };
        if expected_key.is_some_and(|expected| expected != key) {
            return Err(D::Error::custom(
                "webhook event uses the wrong discriminator for its schema",
            ));
        }
        let result = match name.as_str() {
            "subscriber.created" => serde_json::from_value(value).map(Self::SubscriberCreated),
            "subscriber.updated" => serde_json::from_value(value).map(Self::SubscriberUpdated),
            "subscriber.unsubscribed" => {
                serde_json::from_value(value).map(Self::SubscriberUnsubscribed)
            }
            "subscriber.active" => serde_json::from_value(value).map(Self::SubscriberActive),
            "subscriber.bounced" => serde_json::from_value(value).map(Self::SubscriberBounced),
            "subscriber.spam_reported" => {
                serde_json::from_value(value).map(Self::SubscriberSpamReported)
            }
            "subscriber.deleted" => serde_json::from_value(value).map(Self::SubscriberDeleted),
            "subscriber.added_to_group" => {
                serde_json::from_value(value).map(Self::SubscriberAddedToGroup)
            }
            "subscriber.removed_from_group" => {
                serde_json::from_value(value).map(Self::SubscriberRemovedFromGroup)
            }
            "subscriber.automation_triggered" => {
                serde_json::from_value(value).map(Self::SubscriberAutomationTriggered)
            }
            "subscriber.automation_completed" => {
                serde_json::from_value(value).map(Self::SubscriberAutomationCompleted)
            }
            "campaign.sent" => serde_json::from_value(value).map(Self::CampaignSent),
            "campaign.open" => serde_json::from_value(value).map(Self::CampaignOpen),
            "campaign.click" => serde_json::from_value(value).map(Self::CampaignClick),
            _ => Ok(Self::Unknown(UnknownEvent {
                name,
                payload: value,
            })),
        };
        result.map_err(D::Error::custom)
    }
}

/// Explicitly retained payload for an unrecognized event name.
#[derive(Clone)]
pub struct UnknownEvent {
    name: String,
    payload: Value,
}
impl UnknownEvent {
    /// Unrecognized event name as received.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Original JSON value. It may contain personal information or secrets.
    pub fn payload(&self) -> &Value {
        &self.payload
    }
}
impl fmt::Debug for UnknownEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("UnknownEvent(<payload omitted>)")
    }
}
