//! Automation metadata and execution activity.
use crate::models::AutomationId;
/// Automation returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct Automation {
    /// Whether a template-based automation is still being built.
    pub building_in_progress: Option<bool>,
    /// MailerLite identifier, preserved as text.
    pub id: AutomationId,
    /// Display name assigned to this resource.
    pub name: String,
    /// Whether this resource is enabled on the server.
    pub enabled: bool,
    /// Creation timestamp as returned by the server; no timezone is inferred.
    pub created_at: Option<String>,
    /// Whether the server considers the workflow fully configured.
    pub complete: Option<bool>,
    /// Whether the server reports broken workflow relationships.
    pub broken: Option<bool>,
    /// Number of email steps reported for the workflow.
    pub emails_count: Option<u64>,
    /// Number of subscribers reported as qualified for the workflow.
    pub qualified_subscribers_count: Option<u64>,
    /// Shared trigger configuration and workflow state, when included.
    pub trigger_data: Option<TriggerData>,
    /// Workflow steps, when included in the response.
    pub steps: Option<Vec<AutomationStep>>,
    /// Entry triggers, when included in the response.
    pub triggers: Option<Vec<AutomationTrigger>>,
    /// Aggregate workflow execution and email statistics, when included.
    pub stats: Option<AutomationStats>,
}
/// TriggerData returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
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
    /// Whether e-commerce tracking is enabled for the workflow.
    pub track_ecommerce: Option<bool>,
    /// Whether subscribers may enter the workflow again.
    pub repeatable: Option<bool>,
    /// Server-reported validity of the trigger configuration.
    pub valid: Option<bool>,
}
/// AutomationStep returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct AutomationStep {
    /// MailerLite identifier, preserved as text.
    pub id: String,
    /// Server type discriminator, preserving unrecognized values as text.
    #[serde(rename = "type")]
    pub kind: String,
    /// Preceding step identifier; absent for a step without a parent.
    pub parent_id: Option<String>,
    /// Display name assigned to this resource.
    pub name: Option<String>,
    /// Server-provided description of this workflow step.
    pub description: Option<String>,
    /// Whether the step configuration is complete.
    pub complete: Option<bool>,
    /// Whether the server reports broken references for the step.
    pub broken: Option<bool>,
    /// Unit used by the step configuration, when applicable.
    pub unit: Option<String>,
    /// Type-dependent step value, retaining its supported wire representation.
    pub value: Option<crate::models::automation_steps::StepValue>,
    /// Email associated with an email step, when included.
    pub email: Option<crate::models::campaigns::CampaignEmail>,
    /// Typed type-specific configuration from the backend step resource.
    #[serde(flatten)]
    pub properties: crate::models::automation_configuration::StepProperties,
}
/// A trigger embedded in an automation, with the same typed settings as trigger endpoints.
pub type AutomationTrigger = crate::models::automation_triggers::Trigger;
/// AutomationStats returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct AutomationStats {
    /// Subscribers that have completed this workflow.
    pub completed_subscribers_count: Option<u64>,
    /// Subscribers currently queued in this workflow.
    pub subscribers_in_queue_count: Option<u64>,
    /// Number of emails sent, rather than the number successfully delivered.
    pub sent: Option<u64>,
    /// Server-reported email open count.
    pub opens_count: Option<u64>,
    /// Server-reported email click count.
    pub clicks_count: Option<u64>,
}
/// AutomationActivity returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct AutomationActivity {
    /// MailerLite identifier, preserved as text.
    pub id: String,
    /// Server-reported state, preserving unrecognized values as text.
    pub status: String,
    /// Timestamp associated with the reported execution state.
    pub date: Option<String>,
    /// Server reason code explaining the execution state, when supplied.
    pub reason: Option<String>,
    /// Human-readable explanation of the execution state, when supplied.
    pub reason_description: Option<String>,
    /// Step execution history.
    #[serde(rename = "stepRuns")]
    pub step_runs: Option<Vec<StepRun>>,
    /// Next workflow step, when one is reported.
    #[serde(rename = "nextStep")]
    pub next_step: Option<AutomationStep>,
    /// Current workflow step, when one is reported.
    #[serde(rename = "currentStep")]
    pub current_step: Option<AutomationStep>,
}
/// StepRun returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct StepRun {
    /// MailerLite identifier, preserved as text.
    pub id: String,
    /// Identifier of the workflow step that was executed.
    pub step_id: String,
    /// Server-provided description of this step execution.
    pub description: Option<String>,
    /// Scheduled execution timestamp, when an execution has been scheduled.
    pub scheduled_for: Option<String>,
}

/// Shared re-entry settings returned with automation trigger data.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct RepeatableSettings {
    /// Re-entry mode, retaining future server values.
    pub mode: String,
    /// Waiting interval for wait mode.
    pub value: Option<crate::models::automation_configuration::ConfigurationValue>,
    /// Waiting unit for wait mode.
    pub unit: Option<String>,
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
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
