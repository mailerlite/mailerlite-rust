//! Form metadata.
use crate::models::FormId;
/// Form returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct Form {
    /// MailerLite identifier, preserved as text.
    pub id: FormId,
    /// Server type discriminator, preserving unrecognized values as text.
    #[serde(rename = "type")]
    pub kind: String,
    /// Display name assigned to this resource.
    pub name: String,
    /// Server-assigned slug used to identify the form in URLs.
    pub slug: Option<String>,
    /// Creation timestamp as returned by the server; no timezone is inferred.
    pub created_at: Option<String>,
    /// Number of conversions recorded for this form.
    pub conversions_count: Option<u64>,
    /// Number of times this form was opened.
    pub opens_count: Option<u64>,
    /// Conversion rate in the server-provided representation.
    pub conversion_rate: Option<crate::models::common::Rate>,
    /// Most recent registration timestamp, when one is reported.
    pub last_registration_at: Option<String>,
    /// Whether the form is currently active.
    pub active: Option<bool>,
    /// Whether the server reports a broken form configuration.
    pub is_broken: Option<bool>,
    /// Whether the form has configured content.
    pub has_content: Option<bool>,
    /// Whether an automation uses this form, when the relationship is loaded.
    pub used_in_automations: Option<bool>,
    /// Whether double opt-in is enabled; absent or null when not supplied.
    pub double_optin: Option<bool>,
    /// URL of the generated visual preview, when available.
    pub screenshot_url: Option<String>,
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use serde_json::{Value, json};

    fn form() -> Value {
        json!({"id":"003","name":"Form","type":"embedded"})
    }

    #[test]
    fn double_optin_decodes_false_without_treating_it_as_absent() {
        let mut value = form();
        value["double_optin"] = json!(false);
        let form: crate::models::forms::Form = serde_json::from_value(value).unwrap();
        assert_eq!(form.double_optin, Some(false));
    }

    #[test]
    fn double_optin_accepts_documented_null() {
        let mut value = form();
        value["double_optin"] = Value::Null;
        let form: crate::models::forms::Form = serde_json::from_value(value).unwrap();
        assert_eq!(form.double_optin, None);
    }
}
