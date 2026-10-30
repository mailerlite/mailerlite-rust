//! Subscriber lifecycle events and their subscriber details.
use crate::models::{
    SubscriberId,
    subscribers::{FieldValue, SubscriberStatus},
};
use serde::Deserialize;
use std::{collections::BTreeMap, fmt};

/// Subscriber fields flattened beside the `event` discriminator.
#[derive(Clone, Deserialize)]
pub struct SubscriberEvent {
    /// Subscriber affected by this event.
    #[serde(flatten)]
    pub subscriber: WebhookSubscriber,
    /// Account identifier, absent from some documented batched payloads.
    pub account_id: Option<u64>,
}
impl fmt::Debug for SubscriberEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SubscriberEvent(<payload omitted>)")
    }
}

/// Deletion event with a nested subscriber resource.
#[derive(Clone, Deserialize)]
pub struct DeletedSubscriberEvent {
    /// Subscriber deleted or scheduled for forgetting.
    pub subscriber: WebhookSubscriber,
    /// Account identifier, when included.
    pub account_id: Option<u64>,
}
impl fmt::Debug for DeletedSubscriberEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DeletedSubscriberEvent(<payload omitted>)")
    }
}

/// Subscriber representation delivered in webhook payloads.
#[derive(Clone, Deserialize)]
pub struct WebhookSubscriber {
    /// Stable subscriber identifier, retaining leading zeroes.
    pub id: SubscriberId,
    /// Subscriber email address.
    pub email: String,
    /// Subscription status, including future values.
    pub status: SubscriberStatus,
    /// Acquisition source.
    pub source: Option<String>,
    /// Number of messages sent.
    pub sent: u64,
    /// Total opens.
    pub opens_count: u64,
    /// Total clicks.
    pub clicks_count: u64,
    /// Numeric open percentage.
    pub open_rate: f64,
    /// Numeric click percentage.
    pub click_rate: f64,
    /// Subscription IP address, when available.
    pub ip_address: Option<String>,
    /// Subscription timestamp as supplied by the server.
    pub subscribed_at: Option<String>,
    /// Unsubscription timestamp, when applicable.
    pub unsubscribed_at: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
    /// Deletion timestamp, when applicable.
    pub deleted_at: Option<String>,
    /// Scheduled forgetting timestamp, when applicable.
    pub forget_at: Option<String>,
    /// Named default and custom scalar field values.
    pub fields: BTreeMap<String, FieldValue>,
    /// Confirmation timestamp, when available.
    pub opted_in_at: Option<String>,
    /// Confirmation IP address, when available.
    pub optin_ip: Option<String>,
    /// Resolved location; omitted by older documented payloads.
    pub location: Option<WebhookLocation>,
}
impl fmt::Debug for WebhookSubscriber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WebhookSubscriber(<payload omitted>)")
    }
}

/// Optional subscriber location included by the backend webhook resource.
#[derive(Clone, Deserialize)]
pub struct WebhookLocation {
    /// Location identifier.
    pub id: String,
    /// Display title.
    pub title: Option<String>,
    /// Country name.
    pub country_name: Option<String>,
    /// Continent name.
    pub continent_name: Option<String>,
    /// State or region name.
    pub state_name: Option<String>,
    /// Timezone name.
    pub time_zone: Option<String>,
}
impl fmt::Debug for WebhookLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WebhookLocation(<payload omitted>)")
    }
}
