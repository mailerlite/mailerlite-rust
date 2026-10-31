//! Typed step configuration and response details.
use super::{ConfigurationId, ConfigurationValue, ExtensionFields, RelatedObject, TimezoneMode};
use crate::models::{Patch, SecretString};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported delay schedules.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DelayUnit {
    /// The `minutes` API value.
    #[serde(rename = "minutes")]
    Minutes,
    /// The `hours` API value.
    #[serde(rename = "hours")]
    Hours,
    /// The `days` API value.
    #[serde(rename = "days")]
    Days,
    /// The `weeks` API value.
    #[serde(rename = "weeks")]
    Weeks,
    /// The `months` API value.
    #[serde(rename = "months")]
    Months,
    /// The `next` API value.
    #[serde(rename = "next")]
    Next,
    /// The `next_weekday` API value.
    #[serde(rename = "next_weekday")]
    NextWeekday,
    /// The `next_monthday` API value.
    #[serde(rename = "next_monthday")]
    NextMonthday,
    /// The `next_date` API value.
    #[serde(rename = "next_date")]
    NextDate,
    /// The `subscriber_field` API value.
    #[serde(rename = "subscriber_field")]
    SubscriberField,
}

/// Behavior when a scheduled delay date has already elapsed.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ElapsedDateAction {
    /// The `stop` API value.
    #[serde(rename = "stop")]
    Stop,
    /// The `move_to_next_step` API value.
    #[serde(rename = "move_to_next_step")]
    NextStep,
    /// The `move_to_step` API value.
    #[serde(rename = "move_to_step")]
    MoveToStep,
}

/// How multiple conditions are combined.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ConditionMatching {
    /// The `any` API value.
    #[serde(rename = "any")]
    Any,
    /// The `all` API value.
    #[serde(rename = "all")]
    All,
}

/// Supported workflow conditions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    /// The `workflow_activity` API value.
    #[serde(rename = "workflow_activity")]
    WorkflowActivity,
    /// The `campaign_activity` API value.
    #[serde(rename = "campaign_activity")]
    CampaignActivity,
    /// The `custom_fields` API value.
    #[serde(rename = "custom_fields")]
    CustomFields,
    /// The `segment_membership` API value.
    #[serde(rename = "segment_membership")]
    SegmentMembership,
    /// The `group_membership` API value.
    #[serde(rename = "group_membership")]
    GroupMembership,
}

/// Supported subscriber actions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    /// The `update_custom_field` API value.
    #[serde(rename = "update_custom_field")]
    UpdateCustomField,
    /// The `mark_as_unsubscribed` API value.
    #[serde(rename = "mark_as_unsubscribed")]
    MarkAsUnsubscribed,
    /// The `remove_from_group` API value.
    #[serde(rename = "remove_from_group")]
    RemoveFromGroup,
    /// The `go_to_step` API value.
    #[serde(rename = "go_to_step")]
    GoToStep,
    /// The `copy_to_group` API value.
    #[serde(rename = "copy_to_group")]
    CopyToGroup,
    /// The `move_to_group` API value.
    #[serde(rename = "move_to_group")]
    MoveToGroup,
    /// The `send_internal_email_notification` API value.
    #[serde(rename = "send_internal_email_notification")]
    SendInternalEmailNotification,
}

/// Known automation step discriminators.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum StepKind {
    /// Send an email.
    Email,
    /// Wait before continuing.
    Delay,
    /// Evaluate conditions.
    Condition,
    /// Split traffic into A/B paths.
    #[serde(rename = "abtest")]
    AbTest,
    /// Deliver a webhook.
    Webhook,
    /// Perform a subscriber action.
    Action,
}
/// A path in an automation A/B step.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AbPath {
    /// Path A.
    APath,
    /// Path B.
    BPath,
    /// Path C.
    CPath,
}

/// ISO weekday used by scheduled delays, from Monday (1) through Sunday (7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weekday {
    /// Monday, ISO day 1.
    Monday = 1,
    /// Tuesday, ISO day 2.
    Tuesday = 2,
    /// Wednesday, ISO day 3.
    Wednesday = 3,
    /// Thursday, ISO day 4.
    Thursday = 4,
    /// Friday, ISO day 5.
    Friday = 5,
    /// Saturday, ISO day 6.
    Saturday = 6,
    /// Sunday, ISO day 7.
    Sunday = 7,
}

impl Serialize for Weekday {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for Weekday {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Wire {
            Number(u8),
            Text(String),
        }
        let value = match Wire::deserialize(deserializer)? {
            Wire::Number(value) => value,
            Wire::Text(value) => value.parse::<u8>().map_err(serde::de::Error::custom)?,
        };
        match value {
            1 => Ok(Self::Monday),
            2 => Ok(Self::Tuesday),
            3 => Ok(Self::Wednesday),
            4 => Ok(Self::Thursday),
            5 => Ok(Self::Friday),
            6 => Ok(Self::Saturday),
            7 => Ok(Self::Sunday),
            _ => Err(serde::de::Error::custom(
                "ISO weekday must be between 1 and 7",
            )),
        }
    }
}

/// A typed condition definition with optional enriched relationship data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// Condition discriminator.
    #[serde(rename = "type")]
    pub kind: ConditionType,
    /// Custom field identifier, including the special email key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<ConfigurationId>,
    /// Group identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<ConfigurationId>,
    /// Segment identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<ConfigurationId>,
    /// Campaign identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<ConfigurationId>,
    /// Automation email identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<ConfigurationId>,
    /// Specific link identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<ConfigurationId>,
    /// Comparison operator or engagement action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Scalar comparison value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_value: Option<ConfigurationValue>,
    /// Resolved group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<RelatedObject>,
    /// Resolved segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<RelatedObject>,
    /// Resolved campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<RelatedObject>,
    /// Resolved email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<RelatedObject>,
    /// Resolved link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<RelatedObject>,
    /// Resolved custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<RelatedObject>,
}
impl Condition {
    /// Construct a group-membership condition.
    pub fn group(id: impl Into<ConfigurationId>) -> Self {
        Self {
            kind: ConditionType::GroupMembership,
            group_id: Some(id.into()),
            field_id: None,
            segment_id: None,
            campaign_id: None,
            email_id: None,
            link_id: None,
            action: None,
            action_value: None,
            group: None,
            segment: None,
            campaign: None,
            email: None,
            link: None,
            field: None,
        }
    }
}

/// Delay configuration; omitted and explicit-null fields remain distinct.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DelayConfiguration {
    /// Delay schedule.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub unit: Patch<DelayUnit>,
    /// Interval value for simple delays.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub value: Patch<ConfigurationValue>,
    /// Weekdays, Monday 1 through Sunday 7.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub week_days: Patch<Vec<Weekday>>,
    /// Day of month or the string last.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub month_day: Patch<ConfigurationValue>,
    /// Calendar date in YYYY-MM-DD format.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub date: Patch<String>,
    /// Subscriber date field identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub field_id: Patch<ConfigurationId>,
    /// Zero-padded local hour.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub hour: Patch<String>,
    /// Zero-padded minute: 00, 15, 30, or 45.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub minute: Patch<String>,
    /// Timezone selection.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub use_timezone: Patch<TimezoneMode>,
    /// Custom timezone identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub timezone_id: Patch<ConfigurationId>,
    /// Action when the target date has passed.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub action_on_date_elapsed: Patch<ElapsedDateAction>,
    /// Destination when moving to another step.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub go_to_step_id: Patch<ConfigurationId>,
    /// Destination step type.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub go_to_step_type: Patch<String>,
}
impl DelayConfiguration {
    /// Configure a simple interval. Backend validation checks unit-specific limits.
    pub fn new(unit: DelayUnit, value: u64) -> Self {
        Self {
            unit: unit.into(),
            value: ConfigurationValue::from(value).into(),
            ..Self::default()
        }
    }
}

/// Email-step configuration. Sender ownership and domain rules are validated by the API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailConfiguration {
    /// Internal email name.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub name: Patch<String>,
    /// Email subject.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub subject: Patch<String>,
    /// Verified sender email address.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub from: Patch<String>,
    /// Sender display name.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub from_name: Patch<String>,
    /// Reply-to address.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub reply_to: Patch<String>,
    /// Existing workflow email identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub email_id: Patch<ConfigurationId>,
    /// Email language identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub language_id: Patch<ConfigurationId>,
    /// Whether analytics tracking is enabled.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub google_analytics: Patch<bool>,
    /// Whether open tracking is enabled.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub track_opens: Patch<bool>,
    /// Whether tracking has previously been disabled.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub tracking_was_disabled: Patch<bool>,
    /// Email preview text.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub preheader: Patch<String>,
}
impl EmailConfiguration {
    /// Set the required subject and sender details.
    pub fn new(
        subject: impl Into<String>,
        from: impl Into<String>,
        from_name: impl Into<String>,
    ) -> Self {
        Self {
            subject: subject.into().into(),
            from: from.into().into(),
            from_name: from_name.into().into(),
            ..Self::default()
        }
    }
}

/// Configuration for a condition branch.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConditionConfiguration {
    /// How conditions are combined.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub matching_type: Patch<ConditionMatching>,
    /// Matching branch destination.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub yes_step_id: Patch<ConfigurationId>,
    /// Nonmatching branch destination.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub no_step_id: Patch<ConfigurationId>,
    /// Up to five typed conditions.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub conditions: Patch<Vec<Condition>>,
}

/// A/B path allocation.
#[derive(Debug, Clone, Serialize)]
pub struct PathAllocation {
    /// Display label.
    pub label: String,
    /// A/B path.
    pub path: AbPath,
    /// Traffic percentage; the API requires 1 through 100.
    pub value: u8,
}

/// Configuration for an A/B branch.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AbTestConfiguration {
    /// Step name.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub name: Patch<String>,
    /// Path A destination.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub a_path_step_id: Patch<ConfigurationId>,
    /// Path B destination.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub b_path_step_id: Patch<ConfigurationId>,
    /// Optional path C destination.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub c_path_step_id: Patch<ConfigurationId>,
    /// Selected winning path.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub ab_test_winner: Patch<AbPath>,
    /// Two or three allocations totaling 100.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub split_percentages: Patch<Vec<PathAllocation>>,
}

/// Configuration for a subscriber action; fields depend on the selected action.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActionConfiguration {
    /// Subscriber action discriminator.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub action_type: Patch<ActionType>,
    /// Included groups for copy/remove actions.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub group_ids: Patch<Vec<String>>,
    /// Source groups for a move action.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub from_group_ids: Patch<Vec<String>>,
    /// Destination groups for a move action.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub to_group_ids: Patch<Vec<String>>,
    /// Field to update.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub field_id: Patch<ConfigurationId>,
    /// Date update operation.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub date_operation: Patch<String>,
    /// Number update operation.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub number_operation: Patch<String>,
    /// New field value or adjustment.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub value: Patch<ConfigurationValue>,
    /// Destination step type.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub go_to_step_type: Patch<String>,
    /// Destination step identifier.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub go_to_step_id: Patch<ConfigurationId>,
    /// Internal notification subject.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub subject: Patch<String>,
    /// Verified sender address.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub from: Patch<String>,
    /// Sender display name.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub from_name: Patch<String>,
    /// Verified reply-to address.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub reply_to: Patch<String>,
    /// One to five notification recipients.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub recipients: Patch<Vec<String>>,
    /// Internal notification text.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub message: Patch<String>,
    /// Up to twenty included subscriber fields.
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub fields: Patch<Vec<ConfigurationId>>,
}
impl ActionConfiguration {
    /// Select the action to configure.
    pub fn new(action: ActionType) -> Self {
        Self {
            action_type: action.into(),
            ..Self::default()
        }
    }
}

/// Webhook-step configuration. Debug output omits the signing secret.
#[derive(Clone, Serialize, Deserialize)]
pub struct WebhookConfiguration {
    /// Delivery URL.
    pub url: String,
    /// Signing secret.
    pub secret: String,
    /// Existing webhook identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<ConfigurationId>,
    /// Whether this webhook belongs to the step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_webhook: Option<bool>,
}
impl WebhookConfiguration {
    /// Set the required delivery URL and signing secret.
    pub fn new(url: impl Into<String>, secret: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            secret: secret.into(),
            webhook_id: None,
            step_webhook: None,
        }
    }
}
impl fmt::Debug for WebhookConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebhookConfiguration")
            .finish_non_exhaustive()
    }
}
/// Typed request data for the six supported step types.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StepConfiguration {
    /// Email settings.
    Email(EmailConfiguration),
    /// Delay settings.
    Delay(DelayConfiguration),
    /// Condition settings.
    Condition(ConditionConfiguration),
    /// A/B settings.
    AbTest(AbTestConfiguration),
    /// Webhook settings.
    Webhook(WebhookConfiguration),
    /// Subscriber action settings.
    Action(ActionConfiguration),
}
impl StepConfiguration {
    pub(crate) fn kind(&self) -> StepKind {
        match self {
            Self::Email(_) => StepKind::Email,
            Self::Delay(_) => StepKind::Delay,
            Self::Condition(_) => StepKind::Condition,
            Self::AbTest(_) => StepKind::AbTest,
            Self::Webhook(_) => StepKind::Webhook,
            Self::Action(_) => StepKind::Action,
        }
    }
}

/// Field display value in an internal notification action.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct NotificationField {
    /// Field key.
    pub id: String,
    /// Field display name.
    pub text: String,
}

/// Known type-specific fields emitted by step resources.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct StepProperties {
    /// Backend `type_count` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_count: Option<u64>,
    /// Backend `created_at` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Backend `updated_at` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Backend `subject` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Backend `from` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Backend `from_name` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Backend `reply_to` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Backend `email_id` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<ConfigurationId>,
    /// Backend `language_id` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<ConfigurationId>,
    /// Backend `track_opens` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_opens: Option<bool>,
    /// Backend `google_analytics` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_analytics: Option<bool>,
    /// Backend `tracking_was_disabled` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_was_disabled: Option<bool>,
    /// Backend `has_unsubscribe_url` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_unsubscribe_url: Option<bool>,
    /// Backend `has_good_unsubscribe_contrast` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_good_unsubscribe_contrast: Option<bool>,
    /// Backend `has_unlinked_buttons` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_unlinked_buttons: Option<bool>,
    /// Backend `eligible_for_sending` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligible_for_sending: Option<bool>,
    /// Backend `preheader` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preheader: Option<String>,
    /// Backend `builder_type` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_type: Option<String>,
    /// Backend `a_path_step_id` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_path_step_id: Option<ConfigurationId>,
    /// Backend `b_path_step_id` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_path_step_id: Option<ConfigurationId>,
    /// Backend `c_path_step_id` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_path_step_id: Option<ConfigurationId>,
    /// Backend `ab_test_winner` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ab_test_winner: Option<String>,
    /// Backend `can_select_winner` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_select_winner: Option<bool>,
    /// Backend `split_percentages` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_percentages: Option<Vec<crate::models::automation_steps::SplitPercentage>>,
    /// Backend `yes_step_id` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yes_step_id: Option<ConfigurationId>,
    /// Backend `no_step_id` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_step_id: Option<ConfigurationId>,
    /// Backend `matching_type` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_type: Option<String>,
    /// Backend `conditions` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Backend `week_days` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week_days: Option<Vec<Weekday>>,
    /// Backend `month_day` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_day: Option<ConfigurationValue>,
    /// Backend `date` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Backend `field_id` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<ConfigurationId>,
    /// Backend `field` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<RelatedObject>,
    /// Backend `hour` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<String>,
    /// Backend `minute` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<String>,
    /// Backend `use_timezone` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_timezone: Option<String>,
    /// Backend `timezone_id` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_id: Option<ConfigurationId>,
    /// Backend `timezone` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<crate::models::reference::Timezone>,
    /// Backend `delay_is_in_the_past` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_is_in_the_past: Option<bool>,
    /// Backend `action_on_date_elapsed` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_on_date_elapsed: Option<String>,
    /// Backend `go_to_step_id` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_to_step_id: Option<ConfigurationId>,
    /// Backend `go_to_step_type` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_to_step_type: Option<String>,
    /// Backend `action_type` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// Backend `group_ids` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Backend `groups` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::groups::Group>>,
    /// Backend `from_group_ids` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_group_ids: Option<Vec<String>>,
    /// Backend `to_group_ids` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_group_ids: Option<Vec<String>>,
    /// Backend `from_groups` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_groups: Option<Vec<crate::models::groups::Group>>,
    /// Backend `to_groups` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_groups: Option<Vec<crate::models::groups::Group>>,
    /// Backend `date_operation` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_operation: Option<String>,
    /// Backend `number_operation` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_operation: Option<String>,
    /// Backend `recipients` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    /// Backend `message` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Backend `fields` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ConfigurationId>>,
    /// Backend `field_values` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_values: Option<NotificationFields>,
    /// Backend `unverified_recipients` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unverified_recipients: Option<Vec<String>>,
    /// Backend `url` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Backend `secret` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretString>,
    /// Backend `webhook_id` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<ConfigurationId>,
    /// Backend `warning` value, when applicable to this step type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    /// Unrecognized future fields; known fields never fall back to this map.
    #[serde(flatten)]
    pub extensions: ExtensionFields,
}

impl ConditionConfiguration {
    /// Set the matching rule and typed conditions.
    pub fn new(matching: ConditionMatching, conditions: Vec<Condition>) -> Self {
        Self {
            matching_type: matching.into(),
            conditions: conditions.into(),
            ..Self::default()
        }
    }
}
impl AbTestConfiguration {
    /// Set the traffic allocations; the API validates totals and active branches.
    pub fn new(allocations: Vec<PathAllocation>) -> Self {
        Self {
            split_percentages: allocations.into(),
            ..Self::default()
        }
    }
}

/// Field display rows retain the backend collection's array or keyed-object shape.
/// Filtering selected fields in the backend preserves indices, so sparse lists
/// are encoded as objects rather than JSON arrays.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum NotificationFields {
    /// A contiguous list of selected field display rows.
    List(Vec<NotificationField>),
    /// Selected rows keyed by their retained collection indices.
    Indexed(std::collections::BTreeMap<String, NotificationField>),
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use super::*;
    use crate::{
        models::automations::AutomationStep,
        resources::automation_steps::{CreateStep, StepType, UpdateStep},
    };
    use serde_json::json;

    #[test]
    fn typed_delay_infers_step_type_and_preserves_integer_interval() {
        let request = CreateStep::configured(StepConfiguration::Delay(DelayConfiguration::new(
            DelayUnit::Days,
            2,
        )))
        .unwrap();
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({"type":"delay","data":{"unit":"days","value":2}})
        );
    }

    #[test]
    fn typed_email_update_keeps_omitted_false_and_null_distinct() {
        let config = EmailConfiguration {
            track_opens: false.into(),
            preheader: Patch::Null,
            ..Default::default()
        };
        let request = UpdateStep::new()
            .configuration(StepConfiguration::Email(config))
            .unwrap();
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({"data":{"track_opens":false,"preheader":null}})
        );
    }

    #[test]
    fn typed_condition_retains_relationship_identifiers() {
        let config =
            ConditionConfiguration::new(ConditionMatching::All, vec![Condition::group("0009")]);
        let request = CreateStep::configured(StepConfiguration::Condition(config)).unwrap();
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({"type":"condition","data":{"matching_type":"all","conditions":[{"type":"group_membership","group_id":"0009"}]}})
        );
    }

    #[test]
    fn typed_ab_split_uses_backend_abtest_spelling() {
        let config = AbTestConfiguration::new(vec![
            PathAllocation {
                label: "A".into(),
                path: AbPath::APath,
                value: 50,
            },
            PathAllocation {
                label: "B".into(),
                path: AbPath::BPath,
                value: 50,
            },
        ]);
        let request = CreateStep::configured(StepConfiguration::AbTest(config)).unwrap();
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({"type":"abtest","data":{"split_percentages":[{"label":"A","path":"a_path","value":50},{"label":"B","path":"b_path","value":50}]}})
        );
    }

    #[test]
    fn typed_action_preserves_empty_group_list() {
        let mut config = ActionConfiguration::new(ActionType::CopyToGroup);
        config.group_ids = Vec::new().into();
        let request = CreateStep::configured(StepConfiguration::Action(config)).unwrap();
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({"type":"action","data":{"action_type":"copy_to_group","group_ids":[]}})
        );
    }

    #[test]
    fn webhook_secret_is_serialized_but_redacted_from_all_debug_containers() {
        let config =
            WebhookConfiguration::new("https://example.test/hooks", "private-signing-secret");
        let data = StepConfiguration::Webhook(config.clone());
        let create = CreateStep::configured(data.clone()).unwrap();
        let update = UpdateStep::new().configuration(data.clone()).unwrap();
        assert!(
            !format!("{config:?} {data:?} {create:?} {update:?}")
                .contains("private-signing-secret")
        );
        assert_eq!(
            serde_json::to_value(create).unwrap()["data"]["secret"],
            "private-signing-secret"
        );
        let response: AutomationStep = serde_json::from_value(
            json!({"id":"1","type":"webhook","secret":"private-signing-secret"}),
        )
        .unwrap();
        assert_eq!(
            response.properties.secret.as_ref().unwrap().expose_secret(),
            "private-signing-secret"
        );
        assert!(!format!("{response:?}").contains("private-signing-secret"));
    }

    #[test]
    fn known_step_fields_cannot_be_inserted_through_extensions() {
        assert!(
            CreateStep::new(StepType::Delay)
                .extension_data("value", "wrong")
                .is_err()
        );
        assert!(UpdateStep::new().extension_data("secret", true).is_err());
    }

    #[test]
    fn malformed_known_step_field_does_not_fall_back_to_extensions() {
        assert!(
            serde_json::from_value::<AutomationStep>(
                json!({"id":"1","type":"delay","week_days":{"wrong":true}})
            )
            .is_err()
        );
    }

    #[test]
    fn internal_notification_fields_preserve_sparse_collection_indices() {
        let step: AutomationStep = serde_json::from_value(json!({"id":"1","type":"action","action_type":"send_internal_email_notification","field_values":{"2":{"id":"name","text":"Name"},"5":{"id":"company","text":"Company"}}})).unwrap();
        let NotificationFields::Indexed(fields) = step.properties.field_values.unwrap() else {
            panic!("expected keyed fields")
        };
        assert_eq!(fields["2"].id, "name");
        assert_eq!(fields["5"].text, "Company");
    }

    #[test]
    fn step_request_type_mismatch_is_a_local_error() {
        assert!(
            CreateStep::new(StepType::Email)
                .configuration(StepConfiguration::Delay(DelayConfiguration::new(
                    DelayUnit::Days,
                    1
                )))
                .is_err()
        );
    }

    #[test]
    #[expect(
        clippy::expect_used,
        reason = "The fixture must decode both weekday representations"
    )]
    fn delay_response_accepts_stored_integer_and_text_weekdays() {
        // DelayStep::fill stores week_days unchanged and ::toArray returns them as
        // stored. AutomationDelayStepCalculationTest uses integer [1, 7], while
        // UpdateDelayAutomationStepTest uses text ["1", "2"].
        let step: AutomationStep = serde_json::from_value(json!({
            "id":"001", "type":"delay", "unit":"next_weekday",
            "week_days":[1,"7"], "hour":"14", "minute":"15"
        }))
        .expect("both backend weekday representations must decode");
        assert_eq!(step.properties.week_days.as_ref().map(Vec::len), Some(2));
    }

    #[test]
    fn delay_response_rejects_weekdays_outside_the_backend_range() {
        assert!(
            serde_json::from_value::<AutomationStep>(json!({
                "id":"001", "type":"delay", "unit":"next_weekday", "week_days":[8]
            }))
            .is_err()
        );
    }

    #[test]
    fn delay_response_rejects_boolean_weekdays_instead_of_using_extensions() {
        assert!(
            serde_json::from_value::<AutomationStep>(json!({
                "id":"001", "type":"delay", "unit":"next_weekday", "week_days":[true]
            }))
            .is_err()
        );
    }
}
