//! Subscriber response models.
use crate::models::groups::Group;
pub use crate::models::subscriber_activity::*;
use crate::models::{GroupId, SubscriberId};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;

/// A subscriber status, including future values returned by the server.
///
/// Include a wildcard when matching: new SDK variants can be added independently
/// of [`Self::Unknown`], which retains unrecognized values received from the API.
/// Known variants can still be constructed and submitted in requests.
///
/// # Examples
///
/// Exhaustively listing today's variants is rejected outside this crate:
///
/// ```compile_fail,E0004
/// use mailerlite_rs::models::subscribers::SubscriberStatus;
///
/// fn label(status: &SubscriberStatus) -> &str {
///     match status {
///         SubscriberStatus::Active => "active",
///         SubscriberStatus::Unsubscribed => "unsubscribed",
///         SubscriberStatus::Unconfirmed => "unconfirmed",
///         SubscriberStatus::Bounced => "bounced",
///         SubscriberStatus::Junk => "junk",
///         SubscriberStatus::Unknown(value) => value,
///         // Add a `_` arm to handle future SDK variants.
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SubscriberStatus {
    /// Receives campaigns.
    Active,
    /// Has unsubscribed.
    Unsubscribed,
    /// Has not confirmed subscription.
    Unconfirmed,
    /// Email has bounced.
    Bounced,
    /// Classified as junk.
    Junk,
    /// An unrecognized response status; cannot be submitted in requests.
    Unknown(String),
}
impl SubscriberStatus {
    /// Wire representation of this status.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Unsubscribed => "unsubscribed",
            Self::Unconfirmed => "unconfirmed",
            Self::Bounced => "bounced",
            Self::Junk => "junk",
            Self::Unknown(value) => value,
        }
    }
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if matches!(self, Self::Unknown(_)) {
            Err(crate::Error::Config(
                "unknown subscriber status cannot be submitted".into(),
            ))
        } else {
            Ok(())
        }
    }
}
impl Serialize for SubscriberStatus {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if matches!(self, Self::Unknown(_)) {
            return Err(serde::ser::Error::custom(
                "unknown subscriber status cannot be submitted",
            ));
        }
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for SubscriberStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Ok(match s.as_str() {
            "active" => Self::Active,
            "unsubscribed" => Self::Unsubscribed,
            "unconfirmed" => Self::Unconfirmed,
            "bounced" => Self::Bounced,
            "junk" => Self::Junk,
            _ => Self::Unknown(s),
        })
    }
}
/// A custom field value. Null explicitly clears a documented nullable field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    /// Text or a date in the endpoint's documented format.
    Text(String),
    /// A JSON number without conversion through floating point.
    Number(serde_json::Number),
    /// Explicit JSON null.
    Null,
}
impl From<&str> for FieldValue {
    fn from(v: &str) -> Self {
        Self::Text(v.to_owned())
    }
}
impl From<String> for FieldValue {
    fn from(v: String) -> Self {
        Self::Text(v)
    }
}
impl From<i64> for FieldValue {
    fn from(v: i64) -> Self {
        Self::Number(v.into())
    }
}
impl From<u64> for FieldValue {
    fn from(v: u64) -> Self {
        Self::Number(v.into())
    }
}
/// Subscriber returned by MailerLite.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Subscriber {
    /// Stable subscriber identifier.
    pub id: SubscriberId,
    /// Email address.
    pub email: String,
    /// Subscription status.
    pub status: SubscriberStatus,
    /// Acquisition source.
    pub source: Option<String>,
    /// Number of emails sent.
    pub sent: Option<u64>,
    /// Total opens.
    pub opens_count: Option<u64>,
    /// Total clicks.
    pub clicks_count: Option<u64>,
    /// Numeric open percentage.
    pub open_rate: Option<f64>,
    /// Numeric click percentage.
    pub click_rate: Option<f64>,
    /// Subscription IP address.
    pub ip_address: Option<String>,
    /// Subscription timestamp without an assumed timezone.
    pub subscribed_at: Option<String>,
    /// Unsubscription timestamp.
    pub unsubscribed_at: Option<String>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Last modification timestamp.
    pub updated_at: Option<String>,
    /// User-defined and default fields.
    #[serde(default)]
    pub fields: BTreeMap<String, FieldValue>,
    /// Included groups; absent if not requested or not returned.
    pub groups: Option<Vec<Group>>,
    /// Opt-in timestamp.
    pub opted_in_at: Option<String>,
    /// Opt-in IP address.
    pub optin_ip: Option<String>,
    /// Scheduled forgetting timestamp, if present.
    pub forget_at: Option<String>,
    /// Deletion timestamp, if present.
    pub deleted_at: Option<String>,
}
/// Response confirming a forgetting request, not immediate deletion.
#[derive(Debug, Clone, Deserialize)]
pub struct ForgetSubscriber {
    /// Server explanation of the forgetting schedule.
    pub message: String,
    /// Subscriber scheduled to be forgotten.
    pub data: Subscriber,
}
/// Group identifiers supplied when updating membership.
pub type SubscriberGroups = Vec<GroupId>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    fn subscriber() -> Value {
        serde_json::from_str(include_str!(
            "../../tests/fixtures/audience/subscriber.json"
        ))
        .unwrap()
    }

    #[test]
    fn unknown_response_status_is_preserved() {
        let mut value = subscriber();
        value["status"] = json!("future-status");
        let decoded: Subscriber = serde_json::from_value(value).unwrap();
        assert_eq!(
            decoded.status,
            SubscriberStatus::Unknown("future-status".into())
        );
    }
}
