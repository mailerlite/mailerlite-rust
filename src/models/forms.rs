//! Form metadata.
use crate::models::FormId;
/// Form returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct Form {
    /// Id.
    pub id: FormId,
    /// Kind.
    #[serde(rename = "type")]
    pub kind: String,
    /// Name.
    pub name: String,
    /// Slug.
    pub slug: Option<String>,
    /// Created at.
    pub created_at: Option<String>,
    /// Conversions count.
    pub conversions_count: Option<u64>,
    /// Opens count.
    pub opens_count: Option<u64>,
    /// Conversion rate.
    pub conversion_rate: Option<crate::models::campaigns::Rate>,
    /// Last registration at.
    pub last_registration_at: Option<String>,
    /// Active.
    pub active: Option<bool>,
    /// Is broken.
    pub is_broken: Option<bool>,
    /// Has content.
    pub has_content: Option<bool>,
    /// Used in automations.
    pub used_in_automations: Option<bool>,
    /// Whether double opt-in is enabled; absent or null when not supplied.
    pub double_optin: Option<bool>,
    /// Screenshot url.
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
