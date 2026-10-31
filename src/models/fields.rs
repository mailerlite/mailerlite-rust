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
///
/// This server-owned record may gain fields. Deserialize a response fixture and
/// use `..` when destructuring it:
///
/// # Examples
///
/// ```
/// use mailerlite_rs::models::fields::Field;
///
/// # fn main() -> serde_json::Result<()> {
/// let field: Field = serde_json::from_str(
///     r#"{"id":"123","name":"Company","key":"company","type":"text"}"#,
/// )?;
/// let Field { name, .. } = field;
///
/// assert_eq!(name, "Company");
/// # Ok(())
/// # }
/// ```
///
/// External struct literals are intentionally unsupported:
///
/// ```compile_fail,E0639
/// use mailerlite_rs::models::fields::Field;
///
/// let field = Field {
///     id: "123".into(),
///     name: "Company".into(),
///     key: "company".into(),
///     field_type: "text".into(),
/// };
/// ```
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
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
