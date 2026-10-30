//! Opaque identifiers represented exactly as strings.
//!
//! IDs are not numeric quantities. String storage preserves leading zeroes and
//! values larger than machine integers. Numeric JSON is intentionally rejected;
//! endpoints documenting numeric identifiers must use a field-specific decoder.

use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! identifier {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Wraps an opaque identifier without altering its representation.
            #[must_use]
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// Borrows the identifier as a string.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Returns the owned identifier without allocating.
            #[must_use]
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

identifier!(SubscriberId, "An opaque subscriber identifier.");
identifier!(GroupId, "An opaque group identifier.");
identifier!(SegmentId, "An opaque segment identifier.");
identifier!(FieldId, "An opaque custom-field identifier.");
identifier!(CampaignId, "An opaque campaign identifier.");
identifier!(AutomationId, "An opaque automation identifier.");
identifier!(FormId, "An opaque form identifier.");
identifier!(WebhookId, "An opaque webhook identifier.");
identifier!(ImportId, "An opaque subscriber-import identifier.");
identifier!(ShopId, "An opaque e-commerce shop identifier.");
identifier!(ProductId, "An opaque e-commerce product identifier.");
identifier!(CategoryId, "An opaque e-commerce category identifier.");
identifier!(CustomerId, "An opaque e-commerce customer identifier.");
identifier!(OrderId, "An opaque e-commerce order identifier.");
identifier!(CartId, "An opaque e-commerce cart identifier.");
identifier!(CartItemId, "An opaque e-commerce cart-item identifier.");

#[cfg(test)]
mod tests {
    use super::SubscriberId;

    #[test]
    fn preserves_large_and_zero_prefixed_ids() {
        for value in ["000123", "999999999999999999999999999999999"] {
            let json = serde_json::to_string(value).unwrap();
            let id: SubscriberId = serde_json::from_str(&json).unwrap();
            assert_eq!(id.as_str(), value);
            assert_eq!(serde_json::to_string(&id).unwrap(), json);
        }
    }

    #[test]
    fn rejects_numeric_identifiers_without_coercion() {
        for json in ["123", "123.0", "1e20", "null"] {
            assert!(serde_json::from_str::<SubscriberId>(json).is_err());
        }
    }
}
