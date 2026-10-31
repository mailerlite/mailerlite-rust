//! Automation templates verified against `AutomationTemplateResource`.
use serde::Deserialize;

/// A reusable automation workflow template.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct AutomationTemplate {
    /// Template identifier.
    pub id: String,
    /// Localized template name.
    pub name: String,
    /// Template category/type, preserving unknown server values.
    #[serde(rename = "type")]
    pub kind: String,
    /// Workflow definition containing variable step and trigger configurations.
    pub workflow: TemplateWorkflow,
    /// Localized detailed description.
    pub description: Option<String>,
    /// Localized short description.
    pub excerpt: Option<String>,
    /// Variable workflow-step descriptions retained as emitted by the server.
    pub workflow_steps_description: Option<Vec<String>>,
    /// Template URL, when present.
    pub url: Option<String>,
}

/// Reusable workflow stored by the template service.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct TemplateWorkflow {
    /// Shared trigger state in the template.
    pub trigger_data: Option<crate::models::automations::TriggerData>,
    /// Template trigger definitions; incomplete settings are allowed.
    #[serde(default)]
    pub triggers: Vec<crate::models::automation_triggers::Trigger>,
    /// Stored steps; template identifiers are placeholders until instantiated.
    #[serde(default)]
    pub steps: Vec<TemplateStep>,
}
/// A stored template step, distinct from a fully enriched API step response.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct TemplateStep {
    /// Template-local identifier.
    pub id: String,
    /// Step type.
    #[serde(rename = "type")]
    pub kind: String,
    /// Template-local parent identifier.
    pub parent_id: Option<String>,
    /// Localized step description.
    pub description: Option<String>,
    /// Referenced template email, when this is an email step.
    pub email: Option<crate::models::automation_configuration::RelatedObject>,
    /// Configuration stored in the template.
    pub data: TemplateStepData,
}
/// Known stored configuration fields plus explicit future extensions.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct TemplateStepData {
    /// Internal step/email name.
    pub name: Option<String>,
    /// Delay unit.
    pub unit: Option<String>,
    /// Stored scalar delay or field value.
    pub value: Option<crate::models::automation_configuration::ConfigurationValue>,
    /// Source newsletter template, where specified by a template email.
    pub newsletter_template_id: Option<crate::models::automation_configuration::ConfigurationId>,
    /// Remaining known type-specific configuration fields.
    #[serde(flatten)]
    pub properties: crate::models::automation_configuration::StepProperties,
}

#[cfg(test)]
mod tests {
    use super::TemplateWorkflow;
    use serde_json::json;

    #[test]
    fn all_backend_seeded_template_workflows_decode_without_unmodeled_step_fields() {
        let workflows: Vec<TemplateWorkflow> = serde_json::from_str(include_str!(
            "../../tests/fixtures/automation/template_workflows.json"
        ))
        .unwrap();
        assert_eq!(workflows.len(), 18);
        for workflow in workflows {
            for step in workflow.steps {
                assert!(
                    step.data.properties.extensions.is_empty(),
                    "unmodeled seeded step {}",
                    step.id
                );
            }
        }
    }

    #[test]
    fn malformed_known_template_data_is_not_accepted_as_generic_json() {
        assert!(
            serde_json::from_value::<TemplateWorkflow>(
                json!({"steps":[{"id":"1","type":"delay","data":{"value":true}}]})
            )
            .is_err()
        );
    }
}
