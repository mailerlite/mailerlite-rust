//! Automation step reports and subscriber executions verified against backend resources.
use crate::models::groups::Rate;
use serde::Deserialize;
use std::collections::BTreeMap;

/// A step value stored as either JSON text or a JSON number by the backend.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum StepValue {
    /// Text retained without numeric conversion.
    String(String),
    /// A JSON number retained without conversion to floating point.
    Number(serde_json::Number),
}

/// Subscriber counts for an automation step.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct StepRunCounts {
    /// Step identifier.
    pub step_id: String,
    /// Number of completed step executions.
    pub completed_step_runs: u64,
    /// Number of queued step executions.
    pub in_queue_step_runs: u64,
}

/// Traffic allocation to one A/B path.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct SplitPercentage {
    /// Human-readable label, such as `A`.
    pub label: String,
    /// Path name, such as `a_path`.
    pub path: String,
    /// Allocated percentage, retaining the backend's stored number or numeric text.
    pub value: StepValue,
}

/// Engagement statistics for a single A/B path.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct AbPathStats {
    /// Subscribers allocated to this path.
    pub subscribers: u64,
    /// Messages sent.
    pub sent: u64,
    /// Opens recorded.
    pub opens: u64,
    /// Clicks recorded.
    pub clicks: u64,
    /// Open rate, including the server's formatted value.
    pub open_rate: Rate,
    /// Click rate, including the server's formatted value.
    pub click_rate: Rate,
    /// Click-to-open rate, including the server's formatted value.
    pub click_to_open_rate: Rate,
}

/// A/B step results returned by `AbTestStatsResource`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct AbTestStats {
    /// Step identifier.
    pub step_id: String,
    /// Step name.
    pub name: String,
    /// Selected winning path, if one has been selected.
    pub ab_test_winner: Option<String>,
    /// Parent step identifier.
    pub parent_id: Option<String>,
    /// Whether a winner can currently be selected.
    pub can_select_winner: bool,
    /// First step in path A.
    pub a_path_step_id: Option<String>,
    /// First step in path B.
    pub b_path_step_id: Option<String>,
    /// First step in path C, if present.
    pub c_path_step_id: Option<String>,
    /// Configured traffic allocation.
    pub split_percentages: Vec<SplitPercentage>,
    /// Results keyed by active path name.
    pub stats: BTreeMap<String, AbPathStats>,
}

/// Subscriber identity associated with a step or trigger execution row.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct StepSubscriberIdentity {
    /// Subscriber identifier.
    pub id: String,
    /// Subscriber email address.
    pub email: String,
}

/// A subscriber execution returned by `AutomationStepSubscribersResource`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct StepSubscriber {
    /// Execution row identifier, distinct from the subscriber identifier.
    pub id: String,
    /// Subscriber associated with the execution.
    pub subscriber: StepSubscriberIdentity,
    /// Backend timestamp without an implied timezone.
    pub date: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn split_percentage_preserves_numeric_strings() {
        let allocation: crate::models::automation_steps::SplitPercentage =
            serde_json::from_value(json!({"label":"A","path":"a_path","value":"50"})).unwrap();
        assert!(matches!(allocation.value, StepValue::String(value) if value == "50"));
    }
}
