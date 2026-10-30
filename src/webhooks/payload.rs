//! Single-event and batch delivery envelopes.
use super::WebhookEvent;
use serde::{Deserialize, Deserializer, de::Error as _};
use serde_json::Value;

/// A single delivery or a batch of independently typed webhook events.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum WebhookPayload {
    /// One event delivered directly in the request body.
    Single(WebhookEvent),
    /// Several events delivered together under an `events` key.
    Batch(WebhookBatch),
}
impl<'de> Deserialize<'de> for WebhookPayload {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let object = value
            .as_object()
            .ok_or_else(|| D::Error::custom("webhook payload must be an object"))?;
        if object.contains_key("events") {
            if object.contains_key("event") || object.contains_key("type") {
                return Err(D::Error::custom(
                    "webhook payload cannot combine batch and single-event envelopes",
                ));
            }
            serde_json::from_value(value)
                .map(Self::Batch)
                .map_err(D::Error::custom)
        } else {
            serde_json::from_value(value)
                .map(Self::Single)
                .map_err(D::Error::custom)
        }
    }
}

/// A webhook batch, preserving the server's order and reported total.
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookBatch {
    /// Events in the order delivered by MailerLite.
    pub events: Vec<WebhookEvent>,
    /// Number of events reported by the sender.
    pub total: u64,
}
