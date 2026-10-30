//! Custom field response models.
use crate::models::FieldId;
use serde::{Deserialize, Serialize};
/// Documented custom-field types for requests.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    /// Text field.
    Text,
    /// Numeric field.
    Number,
    /// Date field.
    Date,
}
impl FieldType {
    /// Wire name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Date => "date",
        }
    }
}
/// A custom field definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Field {
    /// Field identifier.
    pub id: FieldId,
    /// Display name.
    pub name: String,
    /// Field key used on subscribers.
    pub key: String,
    /// Wire type name, preserving future server values.
    #[serde(rename = "type")]
    pub field_type: String,
}
