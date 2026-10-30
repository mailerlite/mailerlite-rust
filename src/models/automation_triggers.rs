//! Automation trigger responses verified against the application resource classes.
use serde::Deserialize;

/// A trigger with its type-specific configuration preserved.
#[derive(Debug, Clone, Deserialize)]
pub struct Trigger {
    /// String-preserved trigger identifier.
    pub id: String,
    /// Trigger type, preserving future server values.
    #[serde(rename = "type")]
    pub kind: String,
    /// Included group identifiers, when this is a group trigger.
    pub group_ids: Option<Vec<String>>,
    /// Excluded group identifiers, when this is a group trigger.
    pub exclude_group_ids: Option<Vec<String>>,
    /// Whether the trigger configuration is broken.
    pub broken: Option<bool>,
    /// Typed settings and related objects emitted by the concrete trigger resource.
    #[serde(flatten)]
    pub settings: crate::models::automation_configuration::TriggerProperties,
}

/// Result of replacing a trigger's configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatedTrigger {
    /// Refreshed trigger configuration.
    pub trigger: Trigger,
    /// Automation-level trigger settings, varying by automation configuration.
    pub trigger_data: Option<crate::models::automations::TriggerData>,
    /// Included for automations with group or segment triggers.
    pub qualified_subscribers_count: Option<u64>,
}

/// Execution counts for one trigger.
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerRunCounts {
    /// Finished automation executions.
    pub completed_runs: u64,
    /// Active automation executions.
    pub queued_runs: u64,
}

/// A trigger identifier together with its execution counts.
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerCounts {
    /// Trigger identifier.
    pub id: String,
    /// Execution counts.
    #[serde(flatten)]
    pub counts: TriggerRunCounts,
}

/// Execution counts for an automation's hidden manual trigger.
#[derive(Debug, Clone, Deserialize)]
pub struct ManualRunCounts {
    /// Active manual executions.
    pub in_progress_runs: u64,
    /// Finished manual executions.
    pub completed_runs: u64,
}

/// Counts for an automation's triggers and optional manual trigger.
#[derive(Debug, Clone, Deserialize)]
pub struct AllTriggerRunCounts {
    /// Visible triggers and their counts.
    pub triggers: Vec<TriggerCounts>,
    /// Absent when the automation has no manual trigger.
    pub manual_runs: Option<ManualRunCounts>,
}
