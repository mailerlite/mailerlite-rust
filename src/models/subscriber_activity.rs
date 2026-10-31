//! Typed subscriber activity properties, selected by the server's activity kind.
//!
//! Unknown event kinds retain their original JSON explicitly. Malformed known
//! properties remain decoding errors instead of falling back to unknown data.
use crate::models::subscribers::FieldValue;
use serde::{Deserialize, Deserializer, de::Error as _};
use serde_json::Value;
use std::{collections::BTreeMap, fmt};

/// A subscriber activity entry with kind-specific properties.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SubscriberActivity {
    /// Activity identifier, retained exactly as supplied.
    pub id: String,
    /// Server activity kind; unknown future names are preserved.
    pub log_name: String,
    /// Related entity identifier.
    pub subject_id: Option<String>,
    /// Related entity kind.
    pub subject_type: Option<String>,
    /// Properties decoded according to `log_name` and, for email events, `type`.
    pub properties: ActivityProperties,
    /// Creation timestamp without an inferred timezone.
    pub created_at: Option<String>,
    /// Modification timestamp, retained for API compatibility.
    pub updated_at: Option<String>,
}

impl<'de> Deserialize<'de> for SubscriberActivity {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Wire {
            id: String,
            log_name: String,
            subject_id: Option<String>,
            subject_type: Option<String>,
            properties: Value,
            created_at: Option<String>,
            updated_at: Option<String>,
        }
        let wire = Wire::deserialize(d)?;
        let properties = ActivityProperties::decode(&wire.log_name, wire.properties)
            .map_err(D::Error::custom)?;
        Ok(Self {
            id: wire.id,
            log_name: wire.log_name,
            subject_id: wire.subject_id,
            subject_type: wire.subject_type,
            properties,
            created_at: wire.created_at,
            updated_at: wire.updated_at,
        })
    }
}

/// Concrete properties for the known subscriber activity families.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ActivityProperties {
    /// An event without properties, represented by an empty object or array.
    Empty,
    /// A campaign send, open, or click.
    CampaignEmail(CampaignEmailActivity),
    /// An automation send, open, or click.
    AutomationEmail(AutomationEmailActivity),
    /// Addition to or removal from a group; inspect `log_name` for the operation.
    Group(GroupActivity),
    /// Movement between two groups.
    MovedGroup(MovedGroupActivity),
    /// Addition through a subscriber import.
    Imported(ImportActivity),
    /// Addition through an ecommerce customer import.
    EcommerceImported(EcommerceImportActivity),
    /// Change of email address, by a subscriber or an account user.
    EmailChanged(EmailChangeActivity),
    /// Changes keyed by user-defined field name.
    FieldsChanged(BTreeMap<String, FieldChange>),
    /// A one-to-one email sent to the subscriber.
    OneToOneEmail(OneToOneEmailActivity),
    /// An email forwarded to another recipient.
    ForwardedEmail(ForwardEmailActivity),
    /// Bounce, soft-bounce, or spam-complaint context.
    DeliveryIssue(DeliveryIssueActivity),
    /// Answers to a preference-center survey.
    Survey(SurveyActivity),
    /// A prize won through a form.
    FormPrize(FormPrizeActivity),
    /// Purchase of an internal-shop product.
    ProductPurchase(ProductPurchaseActivity),
    /// Changed marketing preference values.
    MarketingPreferences(MarketingPreferencesActivity),
    /// Activation or confirmation through a form or site.
    WebformActivation(WebformActivity),
    /// An unrecognized event or email/webform subtype, retained explicitly.
    Unknown(UnknownActivityProperties),
}

impl ActivityProperties {
    fn decode(kind: &str, value: Value) -> serde_json::Result<Self> {
        use serde_json::from_value;
        let empty = value.as_array().is_some_and(Vec::is_empty)
            || value.as_object().is_some_and(serde_json::Map::is_empty);
        match kind {
            "campaign_send" | "automation_email_sent" | "email_open" | "link_click" => {
                // The backend explicitly returns empty properties when the
                // related email is no longer a campaign/automation email.
                if empty {
                    return Ok(Self::Empty);
                }
                match value.get("type").and_then(Value::as_str) {
                    Some("campaigns") => from_value(value).map(Self::CampaignEmail),
                    Some("automations") => from_value(value).map(Self::AutomationEmail),
                    Some(_) => Ok(Self::Unknown(UnknownActivityProperties(value))),
                    None => Err(serde::de::Error::custom(
                        "email activity requires a string type",
                    )),
                }
            }
            "added_to_group" | "removed_from_group" => from_value(value).map(Self::Group),
            "moved_to_group" => from_value(value).map(Self::MovedGroup),
            "added_through_import" => from_value(value).map(Self::Imported),
            "added_through_ecommerce_import" => from_value(value).map(Self::EcommerceImported),
            "changed_email" | "changed_email_by_user" => from_value(value).map(Self::EmailChanged),
            "changed_fields" => {
                if empty {
                    Ok(Self::FieldsChanged(BTreeMap::new()))
                } else {
                    from_value(value).map(Self::FieldsChanged)
                }
            }
            "one_to_one_email_sent" => from_value(value).map(Self::OneToOneEmail),
            "email_forward" => from_value(value).map(Self::ForwardedEmail),
            "email_bounce" | "email_soft_bounce" | "spam_complaint" => {
                from_value(value).map(Self::DeliveryIssue)
            }
            "answered_preference_survey" => from_value(value).map(Self::Survey),
            "won_webform_prize" => from_value(value).map(Self::FormPrize),
            "product_purchase" => from_value(value).map(Self::ProductPurchase),
            "marketing_preferences_change" => from_value(value).map(Self::MarketingPreferences),
            "activated_via_webform" | "activated_via_confirmation" => {
                if empty {
                    return Ok(Self::Empty);
                }
                match value.get("webform_type").and_then(Value::as_str) {
                    Some("form" | "site") => from_value(value).map(Self::WebformActivation),
                    Some(_) => Ok(Self::Unknown(UnknownActivityProperties(value))),
                    None => Err(serde::de::Error::custom(
                        "activation properties require a string webform_type",
                    )),
                }
            }
            "deleted"
            | "restored"
            | "unsubscribed"
            | "activated"
            | "resubscribed"
            | "status_changed"
            | "status_change_denied"
            | "forget_at_cleared" => {
                if empty {
                    Ok(Self::Empty)
                } else {
                    Err(serde::de::Error::custom(
                        "status activity properties must be empty",
                    ))
                }
            }
            _ => Ok(Self::Unknown(UnknownActivityProperties(value))),
        }
    }
}

/// Original JSON for an unrecognized event or source subtype.
#[derive(Clone)]
pub struct UnknownActivityProperties(Value);
impl UnknownActivityProperties {
    /// Inspect unknown properties explicitly; they may contain personal data.
    pub fn value(&self) -> &Value {
        &self.0
    }
}
impl fmt::Debug for UnknownActivityProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("UnknownActivityProperties(<omitted>)")
    }
}

/// Context for a campaign send, open, or click.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct CampaignEmailActivity {
    /// Campaign identifier.
    pub campaign_id: String,
    /// Campaign email subject, used by the backend as its display name.
    pub campaign_name: Option<String>,
    /// Preview link, possibly empty when no preview is available.
    pub preview_url: String,
    /// Timezone offset label recorded when sending.
    pub sent_at_timezone_offset: Option<String>,
    /// Clicked link label, supplied only for clicks.
    pub link_label: Option<String>,
    /// Clicked link destination, supplied only for clicks.
    pub link_url: Option<String>,
}

/// Context for an automation email send, open, or click.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct AutomationEmailActivity {
    /// Automation identifier.
    pub automation_id: String,
    /// Automation email identifier.
    pub email_id: String,
    /// Email subject, when present.
    pub email_subject: Option<String>,
    /// Preview link, possibly empty when no preview is available.
    pub preview_url: String,
    /// Timezone offset label recorded when sending.
    pub sent_at_timezone_offset: Option<String>,
    /// Clicked link label.
    pub link_label: Option<String>,
    /// Clicked link destination.
    pub link_url: Option<String>,
}

/// Group involved in an addition/removal activity.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct GroupActivity {
    /// Group identifier.
    pub group_id: String,
    /// Group name at the time of the activity.
    pub group_name: String,
}

/// Origin and destination of a group move.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MovedGroupActivity {
    /// Origin group identifier.
    pub from_group_id: String,
    /// Origin group name.
    pub from_group_name: String,
    /// Destination group identifier.
    pub to_group_id: String,
    /// Destination group name.
    pub to_group_name: String,
}

/// Subscriber import that added the subscriber.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ImportActivity {
    /// Import identifier.
    pub import_id: String,
}

/// Ecommerce import source.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EcommerceImportActivity {
    /// Shop identifier.
    pub ecommerce_shop_id: String,
}

/// Email address before and after a change.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EmailChangeActivity {
    /// Previous email address.
    pub old: String,
    /// Replacement email address.
    pub new: String,
}

/// Old and new scalar values of a subscriber custom field.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct FieldChange {
    /// Previous field value, including explicit null.
    pub old: FieldValue,
    /// Replacement field value, including explicit null.
    pub new: FieldValue,
}

/// Context of a one-to-one email.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct OneToOneEmailActivity {
    /// Email identifier.
    pub email_id: String,
    /// Email subject.
    pub subject: Option<String>,
}

/// Forwarding recipient and original message.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ForwardEmailActivity {
    /// Recipient of the forwarded message.
    pub forward_email: ForwardRecipient,
    /// Original email being forwarded.
    pub email: ForwardedEmail,
}

/// Recipient of a forwarded message.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ForwardRecipient {
    /// Recipient name, when supplied.
    pub to_name: Option<String>,
    /// Recipient email address.
    pub to_email: String,
}

/// Original message referenced by a forwarding activity.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ForwardedEmail {
    /// Email subject.
    pub subject: Option<String>,
    /// Email ID; nested model serialization can return an integer or string.
    #[serde(deserialize_with = "nested_identifier")]
    pub id: String,
}

fn nested_identifier<'de, D: Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Id {
        Text(String),
        Integer(u64),
    }
    Ok(match Id::deserialize(d)? {
        Id::Text(value) => value,
        Id::Integer(value) => value.to_string(),
    })
}

/// Bounce or spam-complaint context.
#[derive(Clone, Deserialize)]
#[non_exhaustive]
pub struct DeliveryIssueActivity {
    /// Email subject.
    pub subject: Option<String>,
    /// Related resource type, such as campaigns or automations.
    pub emailable_type: String,
    /// Related resource identifier.
    pub emailable_id: String,
    /// Additional provider-specific context accepted by the backend logger.
    #[serde(flatten)]
    pub extensions: BTreeMap<String, Value>,
}
impl fmt::Debug for DeliveryIssueActivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeliveryIssueActivity")
            .field("emailable_type", &self.emailable_type)
            .field("emailable_id", &self.emailable_id)
            .finish_non_exhaustive()
    }
}

/// Survey questions and the subscriber's answers.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct SurveyActivity {
    /// Survey identifier.
    pub survey_id: String,
    /// Questions included in the activity snapshot.
    pub questions: Vec<ActivitySurveyQuestion>,
    /// Answers included in the activity snapshot.
    pub answers: Vec<ActivitySurveyAnswer>,
}

/// A question from a survey activity snapshot.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ActivitySurveyQuestion {
    /// Question identifier.
    pub id: String,
    /// Question content.
    pub content: String,
}

/// An answer from a survey activity snapshot.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ActivitySurveyAnswer {
    /// Answer identifier.
    pub id: String,
    /// Related question identifier.
    pub question_id: String,
    /// Answer content.
    pub content: String,
}

/// A prize issued by a form.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct FormPrizeActivity {
    /// Form identifier.
    pub form_id: String,
    /// Form name.
    pub form_name: String,
    /// Form type.
    pub form_type: String,
    /// Prize code issued to the subscriber.
    pub code: String,
}

/// Internal-shop product associated with a purchase.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ProductPurchaseActivity {
    /// Product identifier; the resource casts missing product IDs to empty text.
    pub product_id: String,
    /// Product name, or empty text when the product is no longer available.
    pub product_name: String,
    /// Product type, or null when no product was available.
    pub product_type: Option<String>,
}

/// Marketing preference values before and after an update.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MarketingPreferencesActivity {
    /// Previous preference value.
    pub from: FieldValue,
    /// Replacement preference value.
    pub to: FieldValue,
}

/// Form or site through which confirmation or activation occurred.
///
/// Match with a wildcard for future SDK variants and `..` inside known variants
/// for future fields. Unknown `webform_type` values still fail decoding; this
/// attribute does not add an unknown-value fallback to the wire schema.
///
/// # Examples
///
/// Known variant patterns must allow additional fields:
///
/// ```compile_fail,E0638
/// use mailerlite_rs::models::subscribers::WebformActivity;
///
/// fn form_id(activity: &WebformActivity) -> Option<&str> {
///     match activity {
///         WebformActivity::Form { form_id, form_name: _ } => Some(form_id),
///         // Use `WebformActivity::Form { form_id, .. }` instead.
///         _ => None,
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "webform_type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum WebformActivity {
    /// A form confirmed the subscription.
    #[non_exhaustive]
    Form {
        /// Form identifier.
        form_id: String,
        /// Form name.
        form_name: String,
    },
    /// A site confirmed the subscription.
    #[non_exhaustive]
    Site {
        /// Site identifier.
        site_id: String,
        /// Site name.
        site_name: String,
    },
}

#[cfg(test)]
mod tests {
    use crate::models::subscribers::FieldValue;
    use serde_json::{Value, json};

    fn activity_schema(
        kind: &str,
        properties: Value,
    ) -> serde_json::Result<crate::models::subscribers::SubscriberActivity> {
        serde_json::from_value(json!({"id":"00001","log_name":kind,"properties":properties}))
    }

    #[test]
    fn activity_schema_campaign_click_retains_precise_ids_and_link_context() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema("link_click", json!({"type":"campaigns","campaign_id":"900719925474099312345","campaign_name":"Campaign","preview_url":"","link_label":"View","link_url":"https://example.test","sent_at_timezone_offset":"+06:00"})).unwrap();
        let ActivityProperties::CampaignEmail(properties) = result.properties else {
            panic!("expected campaign email")
        };
        assert_eq!(properties.campaign_id, "900719925474099312345");
        assert_eq!(properties.link_url.as_deref(), Some("https://example.test"));
        assert_eq!(
            properties.sent_at_timezone_offset.as_deref(),
            Some("+06:00")
        );
    }

    #[test]
    fn activity_schema_automation_open_uses_automation_email_properties() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema("email_open", json!({"type":"automations","automation_id":"001","email_id":"002","email_subject":"Welcome","preview_url":""})).unwrap();
        let ActivityProperties::AutomationEmail(properties) = result.properties else {
            panic!("expected automation email")
        };
        assert_eq!(properties.automation_id, "001");
        assert_eq!(properties.email_id, "002");
    }

    #[test]
    fn activity_schema_group_membership_uses_typed_context() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema(
            "added_to_group",
            json!({"group_id":"0002","group_name":"Readers"}),
        )
        .unwrap();
        let ActivityProperties::Group(properties) = result.properties else {
            panic!("expected group")
        };
        assert_eq!(properties.group_id, "0002");
        assert_eq!(properties.group_name, "Readers");
    }

    #[test]
    fn activity_schema_rejects_empty_group_properties() {
        assert!(activity_schema("added_to_group", json!({})).is_err());
    }

    #[test]
    fn activity_schema_rejects_nonempty_arrays_for_known_groups() {
        assert!(activity_schema("removed_from_group", json!([{"group_id":"1"}])).is_err());
    }

    #[test]
    fn activity_schema_changed_fields_retains_scalar_null_and_number_values() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema(
            "changed_fields",
            json!({"name":{"old":null,"new":"Jane"},"score":{"old":1,"new":9007199254740993u64}}),
        )
        .unwrap();
        let ActivityProperties::FieldsChanged(properties) = result.properties else {
            panic!("expected field changes")
        };
        assert_eq!(properties["name"].old, FieldValue::Null);
        assert_eq!(properties["name"].new, FieldValue::Text("Jane".into()));
        assert_eq!(
            properties["score"].new,
            FieldValue::Number(9007199254740993u64.into())
        );
    }

    #[test]
    fn activity_schema_rejects_object_values_for_scalar_field_changes() {
        assert!(
            activity_schema(
                "changed_fields",
                json!({"name":{"old":"Jane","new":{"unexpected":true}}})
            )
            .is_err()
        );
    }

    #[test]
    fn activity_schema_unknown_events_are_explicit_and_debug_redacted() {
        use crate::models::subscribers::ActivityProperties;
        let result =
            activity_schema("future_event", json!({"new_property":"private-marker"})).unwrap();
        assert!(!format!("{result:?}").contains("private-marker"));
        let ActivityProperties::Unknown(properties) = result.properties else {
            panic!("expected unknown event")
        };
        assert_eq!(properties.value()["new_property"], "private-marker");
    }

    #[test]
    fn activity_schema_unknown_email_subtype_retains_its_original_data() {
        use crate::models::subscribers::ActivityProperties;
        let result =
            activity_schema("email_open", json!({"type":"future_email","id":"001"})).unwrap();
        let ActivityProperties::Unknown(properties) = result.properties else {
            panic!("expected unknown subtype")
        };
        assert_eq!(properties.value()["type"], "future_email");
    }

    #[test]
    fn activity_schema_accepts_backend_empty_email_properties() {
        use crate::models::subscribers::ActivityProperties;
        assert!(matches!(
            activity_schema("email_open", json!([])).unwrap().properties,
            ActivityProperties::Empty
        ));
    }

    #[test]
    fn activity_schema_accepts_empty_status_object() {
        use crate::models::subscribers::ActivityProperties;
        assert!(matches!(
            activity_schema("deleted", json!({})).unwrap().properties,
            ActivityProperties::Empty
        ));
    }

    #[test]
    fn activity_schema_survey_preserves_question_answer_relationships() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema("answered_preference_survey", json!({"survey_id":"001","questions":[{"id":"002","content":"Which topic?"}],"answers":[{"id":"003","question_id":"002","content":"Rust"}]})).unwrap();
        let ActivityProperties::Survey(properties) = result.properties else {
            panic!("expected survey")
        };
        assert_eq!(
            properties.answers[0].question_id,
            properties.questions[0].id
        );
        assert_eq!(properties.answers[0].content, "Rust");
    }

    #[test]
    fn activity_schema_forwarded_message_preserves_numeric_nested_id() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema("email_forward", json!({"forward_email":{"to_name":null,"to_email":"reader@example.test"},"email":{"id":9007199254740993u64,"subject":"News"}})).unwrap();
        let ActivityProperties::ForwardedEmail(properties) = result.properties else {
            panic!("expected forwarded email")
        };
        assert_eq!(properties.email.id, "9007199254740993");
        assert_eq!(properties.forward_email.to_name, None);
    }

    #[test]
    fn activity_schema_webform_confirmation_uses_typed_source() {
        use crate::models::subscribers::{ActivityProperties, WebformActivity};
        let result = activity_schema(
            "activated_via_confirmation",
            json!({"webform_type":"form","form_id":"002","form_name":"Newsletter"}),
        )
        .unwrap();
        assert!(
            matches!(result.properties, ActivityProperties::WebformActivation(WebformActivity::Form { form_id, .. }) if form_id == "002")
        );
    }

    #[test]
    fn activity_schema_delivery_context_preserves_explicit_provider_extensions_without_debug_leaks()
    {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema("email_bounce", json!({"subject":"News","emailable_type":"campaigns","emailable_id":"001","provider_detail":"private-marker"})).unwrap();
        assert!(!format!("{result:?}").contains("private-marker"));
        let ActivityProperties::DeliveryIssue(properties) = result.properties else {
            panic!("expected delivery issue")
        };
        assert_eq!(properties.extensions["provider_detail"], "private-marker");
    }

    #[test]
    fn activity_schema_group_move_retains_both_group_identifiers() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema("moved_to_group", json!({"from_group_id":"001","from_group_name":"Old","to_group_id":"002","to_group_name":"New"})).unwrap();
        let ActivityProperties::MovedGroup(properties) = result.properties else {
            panic!("expected group move")
        };
        assert_eq!(
            (
                properties.from_group_id.as_str(),
                properties.to_group_id.as_str()
            ),
            ("001", "002")
        );
    }

    #[test]
    fn activity_schema_import_preserves_import_identifier() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema("added_through_import", json!({"import_id":"001"})).unwrap();
        assert!(
            matches!(result.properties, ActivityProperties::Imported(properties) if properties.import_id == "001")
        );
    }

    #[test]
    fn activity_schema_ecommerce_import_preserves_shop_identifier() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema(
            "added_through_ecommerce_import",
            json!({"ecommerce_shop_id":"001"}),
        )
        .unwrap();
        assert!(
            matches!(result.properties, ActivityProperties::EcommerceImported(properties) if properties.ecommerce_shop_id == "001")
        );
    }

    #[test]
    fn activity_schema_email_change_preserves_old_and_new_addresses() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema(
            "changed_email_by_user",
            json!({"old":"old@example.test","new":"new@example.test"}),
        )
        .unwrap();
        let ActivityProperties::EmailChanged(properties) = result.properties else {
            panic!("expected email change")
        };
        assert_eq!(properties.old, "old@example.test");
        assert_eq!(properties.new, "new@example.test");
    }

    #[test]
    fn activity_schema_one_to_one_email_preserves_email_identity() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema(
            "one_to_one_email_sent",
            json!({"email_id":"001","subject":"Hello"}),
        )
        .unwrap();
        assert!(
            matches!(result.properties, ActivityProperties::OneToOneEmail(properties) if properties.email_id == "001")
        );
    }

    #[test]
    fn activity_schema_form_prize_preserves_code_and_form_kind() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema(
            "won_webform_prize",
            json!({"form_id":"001","form_name":"Prize","form_type":"wheel","code":"example-code"}),
        )
        .unwrap();
        let ActivityProperties::FormPrize(properties) = result.properties else {
            panic!("expected prize")
        };
        assert_eq!(properties.form_type, "wheel");
        assert_eq!(properties.code, "example-code");
    }

    #[test]
    fn activity_schema_missing_purchased_product_preserves_empty_id_and_null_type() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema(
            "product_purchase",
            json!({"product_id":"","product_name":"","product_type":null}),
        )
        .unwrap();
        let ActivityProperties::ProductPurchase(properties) = result.properties else {
            panic!("expected purchase")
        };
        assert!(properties.product_id.is_empty());
        assert!(properties.product_type.is_none());
    }

    #[test]
    fn activity_schema_marketing_preference_change_uses_scalar_values() {
        use crate::models::subscribers::ActivityProperties;
        let result = activity_schema(
            "marketing_preferences_change",
            json!({"from":"","to":"email"}),
        )
        .unwrap();
        let ActivityProperties::MarketingPreferences(properties) = result.properties else {
            panic!("expected marketing preferences")
        };
        assert_eq!(properties.from, FieldValue::Text(String::new()));
        assert_eq!(properties.to, FieldValue::Text("email".into()));
    }
}
