//! Automation metadata and execution activity.
use crate::models::AutomationId;
/// Automation returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Automation {
    /// Whether a template-based automation is still being built.
    pub building_in_progress: Option<bool>,
    /// Id.
    pub id: AutomationId,
    /// Name.
    pub name: String,
    /// Enabled.
    pub enabled: bool,
    /// Created at.
    pub created_at: Option<String>,
    /// Complete.
    pub complete: Option<bool>,
    /// Broken.
    pub broken: Option<bool>,
    /// Emails count.
    pub emails_count: Option<u64>,
    /// Qualified subscribers count.
    pub qualified_subscribers_count: Option<u64>,
    /// Trigger data.
    pub trigger_data: Option<TriggerData>,
    /// Steps.
    pub steps: Option<Vec<AutomationStep>>,
    /// Triggers.
    pub triggers: Option<Vec<AutomationTrigger>>,
    /// Stats.
    pub stats: Option<AutomationStats>,
}
/// TriggerData returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct TriggerData {
    /// Whether a test workflow is still being processed.
    pub test_workflow_in_progress: Option<bool>,
    /// Whether a subscriber-buffer limit notification has been emitted.
    pub buffer_limit_event_sent: Option<bool>,
    /// Whether this workflow originated in Classic.
    pub migrated_from_classic: Option<bool>,
    /// Original Classic workflow identifier, when present in current API metadata.
    pub classic_automation_id: Option<crate::models::automation_configuration::ConfigurationId>,
    /// First activation timestamp after migration.
    pub classic_first_enabled_at: Option<String>,
    /// First completed migration synchronization timestamp.
    pub classic_first_synced_at: Option<String>,
    /// Persisted trigger interval unit on older workflow records.
    pub unit: Option<String>,
    /// Persisted trigger interval or comparison value on older records.
    pub value: Option<crate::models::automation_configuration::ConfigurationValue>,
    /// Persisted custom-field identifier on older workflow records.
    pub field_id: Option<crate::models::automation_configuration::ConfigurationId>,
    /// Unrecognized future metadata, omitted from debug formatting.
    #[serde(flatten)]
    pub extensions: crate::models::automation_configuration::ExtensionFields,
    /// Whether a template-based automation is still being built.
    pub building_in_progress: Option<bool>,
    /// Re-entry waiting behavior.
    pub repeatable_settings: Option<RepeatableSettings>,
    /// Whether analytics are displayed.
    pub show_analytics: Option<bool>,
    /// Whether the complete workflow is configured.
    pub complete_workflow: Option<bool>,
    /// Whether a workflow relationship is broken.
    pub broken_workflow: Option<bool>,
    /// Track ecommerce.
    pub track_ecommerce: Option<bool>,
    /// Repeatable.
    pub repeatable: Option<bool>,
    /// Valid.
    pub valid: Option<bool>,
}
/// AutomationStep returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AutomationStep {
    /// Id.
    pub id: String,
    /// Kind.
    #[serde(rename = "type")]
    pub kind: String,
    /// Parent id.
    pub parent_id: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Complete.
    pub complete: Option<bool>,
    /// Broken.
    pub broken: Option<bool>,
    /// Unit.
    pub unit: Option<String>,
    /// Value.
    pub value: Option<crate::models::automation_steps::StepValue>,
    /// Email.
    pub email: Option<crate::models::campaigns::CampaignEmail>,
    /// Typed type-specific configuration from the backend step resource.
    #[serde(flatten)]
    pub properties: crate::models::automation_configuration::StepProperties,
}
/// A trigger embedded in an automation, with the same typed settings as trigger endpoints.
pub type AutomationTrigger = crate::models::automation_triggers::Trigger;
/// AutomationStats returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AutomationStats {
    /// Completed subscribers count.
    pub completed_subscribers_count: Option<u64>,
    /// Subscribers in queue count.
    pub subscribers_in_queue_count: Option<u64>,
    /// Sent.
    pub sent: Option<u64>,
    /// Opens count.
    pub opens_count: Option<u64>,
    /// Clicks count.
    pub clicks_count: Option<u64>,
}
/// AutomationActivity returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AutomationActivity {
    /// Id.
    pub id: String,
    /// Status.
    pub status: String,
    /// Date.
    pub date: Option<String>,
    /// Reason.
    pub reason: Option<String>,
    /// Reason description.
    pub reason_description: Option<String>,
    /// Step execution history.
    #[serde(rename = "stepRuns")]
    pub step_runs: Option<Vec<StepRun>>,
    /// Next step.
    #[serde(rename = "nextStep")]
    pub next_step: Option<AutomationStep>,
    /// Current step.
    #[serde(rename = "currentStep")]
    pub current_step: Option<AutomationStep>,
}
/// StepRun returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct StepRun {
    /// Id.
    pub id: String,
    /// Step id.
    pub step_id: String,
    /// Description.
    pub description: Option<String>,
    /// Scheduled for.
    pub scheduled_for: Option<String>,
}

/// Shared re-entry settings returned with automation trigger data.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct RepeatableSettings {
    /// Re-entry mode, retaining future server values.
    pub mode: String,
    /// Waiting interval for wait mode.
    pub value: Option<crate::models::automation_configuration::ConfigurationValue>,
    /// Waiting unit for wait mode.
    pub unit: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::automation_steps::StepValue;
    use serde_json::json;

    #[test]
    fn step_preserves_leading_zero_string_values() {
        let text_step: AutomationStep =
            serde_json::from_value(json!({"id":"1","type":"delay","value":"001"})).unwrap();
        assert!(matches!(text_step.value, Some(StepValue::String(value)) if value == "001"));
    }

    #[test]
    fn automation_decodes_background_build_flags() {
        let pending: Automation = serde_json::from_value(json!({"id":"1","name":"Template","enabled":false,"building_in_progress":true,"trigger_data":{"building_in_progress":true}})).unwrap();
        assert_eq!(pending.building_in_progress, Some(true));
        assert_eq!(
            pending.trigger_data.unwrap().building_in_progress,
            Some(true)
        );
    }
}
