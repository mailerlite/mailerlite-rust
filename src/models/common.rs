//! Shared scalar values for requests and responses.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// A fractional rate paired with the server's formatted percentage.
///
/// `float` is suitable for calculations; `string` is the display representation.
/// The SDK preserves both values without rescaling or recomputing them. Formatting
/// and rounding come from the server, so the string need not preserve every digit
/// of the numeric value.
#[derive(Debug, Clone, Deserialize)]
pub struct Rate {
    /// Fractional rate before percentage conversion; `0.25` represents 25%.
    pub float: f64,
    /// Server-formatted percentage including `%`, such as `"25%"`.
    pub string: String,
}

/// An optional update that distinguishes omission from an explicit JSON null.
///
/// Only use this type for fields whose endpoint documents null semantics. Fields
/// in a containing request must use `#[serde(default, skip_serializing_if =
/// "Patch::is_unset")]`. Serializing `Unset` directly returns an error because
/// omission is only meaningful for a containing field.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Patch<T> {
    /// Omit the field, leaving the existing value unchanged.
    #[default]
    Unset,
    /// Send an explicit JSON null.
    Null,
    /// Send a new value.
    Value(T),
}

impl<T> Patch<T> {
    /// Returns whether a containing request should omit this field.
    #[must_use]
    pub fn is_unset(&self) -> bool {
        matches!(self, Self::Unset)
    }
}

impl<T> From<T> for Patch<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl<T: Serialize> Serialize for Patch<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unset => Err(serde::ser::Error::custom(
                "Patch::Unset must be omitted using skip_serializing_if",
            )),
            Self::Null => serializer.serialize_none(),
            Self::Value(value) => value.serialize(serializer),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Patch<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<T>::deserialize(deserializer).map(|value| value.map_or(Self::Null, Self::Value))
    }
}

/// A secret string excluded from debug output and automatic serialization.
///
/// Use [`Self::expose_secret`] only when constructing authenticated requests or
/// verifying webhook signatures. This wrapper provides redaction, not memory
/// zeroization. It deliberately does not implement `Display` or `Serialize`.
#[derive(Clone, Deserialize)]
#[serde(transparent)]
pub struct SecretString(String);

impl SecretString {
    /// Stores a secret without exposing it through formatting.
    #[must_use]
    pub fn new(secret: impl Into<String>) -> Self {
        Self(secret.into())
    }

    /// Explicitly exposes the secret for authentication or signature checking.
    #[must_use]
    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretString([REDACTED])")
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use super::{Patch, Rate, SecretString};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct Update {
        #[serde(default, skip_serializing_if = "Patch::is_unset")]
        name: Patch<String>,
    }

    #[test]
    fn rate_preserves_fraction_and_server_rounded_display() {
        let rate: Rate = serde_json::from_str(r#"{"float":0.123456,"string":"12.35%"}"#).unwrap();

        assert_eq!(rate.float, 0.123456);
        assert_eq!(rate.string, "12.35%");
    }

    #[test]
    fn rate_public_paths_share_the_same_type() {
        let rate: crate::models::campaigns::Rate =
            serde_json::from_str(r#"{"float":0.25,"string":"25%"}"#).unwrap();
        let rate: crate::models::groups::Rate = rate;
        let rate: Rate = rate;

        assert_eq!(rate.float, 0.25);
        assert_eq!(rate.string, "25%");
    }

    #[test]
    fn omission_null_and_value_are_distinct() {
        for (value, expected) in [
            (Patch::Unset, json!({})),
            (Patch::Null, json!({"name": null})),
            (Patch::Value("Jane".into()), json!({"name": "Jane"})),
        ] {
            let update = Update { name: value };
            assert_eq!(serde_json::to_value(&update).unwrap(), expected);
            assert_eq!(serde_json::from_value::<Update>(expected).unwrap(), update);
        }
    }

    #[test]
    fn unset_cannot_silently_become_null() {
        assert!(serde_json::to_string(&Patch::<String>::Unset).is_err());
    }

    #[test]
    fn secrets_are_redacted_in_nested_debug_output() {
        let secret: SecretString = serde_json::from_str("\"private-token\"").unwrap();
        assert_eq!(secret.expose_secret(), "private-token");
        assert!(!format!("{:?}", Some(secret.clone())).contains("private-token"));
        assert!(!format!("{secret:#?}").contains("private-token"));
    }
}
