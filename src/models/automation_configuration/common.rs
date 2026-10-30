//! Values and related objects shared by step and trigger configurations.
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt};

/// An identifier stored as either text or a nonnegative integer in workflow JSON.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ConfigurationId {
    /// Text, retaining leading zeroes.
    Text(String),
    /// Exact nonnegative integer.
    Number(u64),
}
impl From<&str> for ConfigurationId {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}
impl From<String> for ConfigurationId {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl From<u64> for ConfigurationId {
    fn from(value: u64) -> Self {
        Self::Number(value)
    }
}

/// A scalar custom-field comparison or stored delay value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConfigurationValue {
    /// Text, including date strings.
    Text(String),
    /// Exact JSON number, without a floating-point conversion.
    Number(serde_json::Number),
}
impl From<&str> for ConfigurationValue {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}
impl From<String> for ConfigurationValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl From<u64> for ConfigurationValue {
    fn from(value: u64) -> Self {
        Self::Number(value.into())
    }
}
impl From<i64> for ConfigurationValue {
    fn from(value: i64) -> Self {
        Self::Number(value.into())
    }
}

/// Future response fields, exposed explicitly and omitted from debug formatting.
#[derive(Clone, Default, Deserialize)]
#[serde(transparent)]
pub struct ExtensionFields(BTreeMap<String, serde_json::Value>);
impl ExtensionFields {
    /// Read a future field without changing its wire value.
    pub fn get(&self, name: &str) -> Option<&serde_json::Value> {
        self.0.get(name)
    }
    /// Whether the server returned any unmodeled fields.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl fmt::Debug for ExtensionFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ExtensionFields(<values omitted>)")
    }
}

/// Timezone used for a scheduled operation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TimezoneMode {
    /// The `account` API value.
    #[serde(rename = "account")]
    Account,
    /// The `subscriber` API value.
    #[serde(rename = "subscriber")]
    Subscriber,
    /// The `custom` API value.
    #[serde(rename = "custom")]
    Custom,
}

/// Compact related object included in automation responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedObject {
    /// Identifier, retaining the stored wire representation.
    pub id: ConfigurationId,
    /// Display name when included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Object URL when included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether an ecommerce shop is internal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    /// Whether the referenced email or campaign tracks opens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_opens: Option<bool>,
    /// Object type when included.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Custom-field key when included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
