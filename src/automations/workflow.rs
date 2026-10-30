//! Workflow endpoints ported from SDK v1. These additions are not yet covered by the public
//! automation documentation; kind-specific configuration and report bodies remain JSON.

use super::{
    Automation, AutomationActivityStatus, Automations, Step, StepType, Trigger, TriggerType,
};
use crate::types::string_enum;
use crate::{Id, MailerLite, Page, Result, Sort, Timestamp};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

string_enum! {
 /// Wire values for `WorkflowBranch`.
 pub enum WorkflowBranch {
 /// `all`.
 All => "all",
 /// `yes`.
 Yes => "yes",
 /// `no`.
 No => "no",
 /// `a_path`.
 APath => "a_path",
 /// `b_path`.
 BPath => "b_path",
 /// `c_path`.
 CPath => "c_path",
}
}
string_enum! {
 /// Wire values for `ReaddTo`.
 pub enum ReaddTo {
 /// `last_completed_step`.
 LastCompletedStep => "last_completed_step",
 /// `last_failed_step`.
 LastFailedStep => "last_failed_step",
 /// `selected_step`.
 SelectedStep => "selected_step",
}
}
string_enum! {
 /// Wire values for `WorkflowSubscriberStatus`.
 pub enum WorkflowSubscriberStatus {
 /// `queued`.
 Queued => "queued",
 /// `completed`.
 Completed => "completed",
}
}
string_enum! {
 /// Wire values for `WorkflowSubscriberSort`.
 pub enum WorkflowSubscriberSort {
 /// When the subscriber reached the step or trigger.
 Date => "date",
 /// `email`.
 Email => "email",
}
}
/// Name and optional trigger settings for [`Automations::update`].
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UpdateAutomation {
    /// Automation or template name.
    pub name: String,
    /// Trigger settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_data: Option<Map<String, Value>>,
}
impl UpdateAutomation {
    /// Creates a payload.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            trigger_data: None,
        }
    }
    /// Sets `trigger_data`.
    #[must_use]
    pub fn trigger_data(mut self, trigger_data: Map<String, Value>) -> Self {
        self.trigger_data = Some(trigger_data);
        self
    }
}
/// Subscribers to enqueue in an enabled automation. Supply emails, groups, or both.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AddAutomationSubscribers {
    /// Email addresses to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// Group ids whose active subscribers should enter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Id>>,
    /// Step where subscribers enter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<Id>,
}
impl AddAutomationSubscribers {
    /// Creates a payload.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets `emails`.
    #[must_use]
    pub fn emails(mut self, emails: Vec<String>) -> Self {
        self.emails = Some(emails);
        self
    }
    /// Sets `groups`.
    #[must_use]
    pub fn groups(mut self, groups: Vec<Id>) -> Self {
        self.groups = Some(groups);
        self
    }
    /// Sets `step_id`.
    #[must_use]
    pub fn step_id(mut self, step_id: impl Into<Id>) -> Self {
        self.step_id = Some(step_id.into());
        self
    }
}
/// Runs selected by state, dates, step, email search or run ids.
/// Cancel and skip-delay actions require [`AutomationActivityStatus::Active`].
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AutomationRunFilter {
    /// `status`.
    pub status: AutomationActivityStatus,
    /// `id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<Id>>,
    /// `step`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<Id>,
    /// `search`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// `date_from`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// `date_to`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// `scheduled_from`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_from: Option<String>,
    /// `scheduled_to`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_to: Option<String>,
}
impl AutomationRunFilter {
    /// Creates a payload.
    #[must_use]
    pub fn new(status: AutomationActivityStatus) -> Self {
        Self {
            status,
            id: None,
            step: None,
            search: None,
            date_from: None,
            date_to: None,
            scheduled_from: None,
            scheduled_to: None,
        }
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, id: Vec<Id>) -> Self {
        self.id = Some(id);
        self
    }
    /// Sets `step`.
    #[must_use]
    pub fn step(mut self, step: impl Into<Id>) -> Self {
        self.step = Some(step.into());
        self
    }
    /// Sets `search`.
    #[must_use]
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }
    /// Sets `date_from`.
    #[must_use]
    pub fn date_from(mut self, date_from: impl Into<String>) -> Self {
        self.date_from = Some(date_from.into());
        self
    }
    /// Sets `date_to`.
    #[must_use]
    pub fn date_to(mut self, date_to: impl Into<String>) -> Self {
        self.date_to = Some(date_to.into());
        self
    }
    /// Sets `scheduled_from`.
    #[must_use]
    pub fn scheduled_from(mut self, scheduled_from: impl Into<String>) -> Self {
        self.scheduled_from = Some(scheduled_from.into());
        self
    }
    /// Sets `scheduled_to`.
    #[must_use]
    pub fn scheduled_to(mut self, scheduled_to: impl Into<String>) -> Self {
        self.scheduled_to = Some(scheduled_to.into());
        self
    }
}
/// `AutomationRunAction` payload.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AutomationRunAction {
    /// Run selection, serialized as a nested JSON object.
    pub filter: AutomationRunFilter,
}
impl AutomationRunAction {
    /// Creates a payload.
    #[must_use]
    pub fn new(filter: AutomationRunFilter) -> Self {
        Self { filter }
    }
}
/// `CopyAutomationSubscribers` payload.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CopyAutomationSubscribers {
    /// Run selection, serialized as a nested JSON object.
    pub filter: AutomationRunFilter,
    /// Groups to copy matching subscribers into.
    pub destination_groups: Vec<Id>,
}
impl CopyAutomationSubscribers {
    /// Creates a payload.
    #[must_use]
    pub fn new(filter: AutomationRunFilter, destination_groups: Vec<Id>) -> Self {
        Self {
            filter,
            destination_groups,
        }
    }
}
/// Past runs to re-add and where they re-enter.
/// `SelectedStep` requires `selected_step_id`; only completed, canceled or failed runs qualify.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ReaddAutomationSubscribers {
    /// Run selection, serialized as a nested JSON object.
    pub filter: AutomationRunFilter,
    /// Where the matching subscribers re-enter.
    pub readd_subscribers_to: ReaddTo,
    /// Step to re-enter when selecting a specific step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_step_id: Option<Id>,
}
impl ReaddAutomationSubscribers {
    /// Creates a payload.
    #[must_use]
    pub fn new(filter: AutomationRunFilter, readd_subscribers_to: ReaddTo) -> Self {
        Self {
            filter,
            readd_subscribers_to,
            selected_step_id: None,
        }
    }
    /// Sets `selected_step_id`.
    #[must_use]
    pub fn selected_step_id(mut self, selected_step_id: impl Into<Id>) -> Self {
        self.selected_step_id = Some(selected_step_id.into());
        self
    }
}
/// Optional inclusive reporting period. The API requires both dates or neither.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AutomationEmailStatsQuery {
    /// First day, `YYYY-MM-DD`; set together with `end_date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Last day inclusive, `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}
impl AutomationEmailStatsQuery {
    /// Creates a payload.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets `start_date`.
    #[must_use]
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// Sets `end_date`.
    #[must_use]
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
}
impl AutomationEmailStatsQuery {
    /// Reports on an inclusive date range (`YYYY-MM-DD`).
    #[must_use]
    pub fn period(mut self, start: impl Into<String>, end: impl Into<String>) -> Self {
        self.start_date = Some(start.into());
        self.end_date = Some(end.into());
        self
    }
}

/// `AutomationEmailContent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct AutomationEmailContent {
    /// Full HTML email content, including merge tags.
    #[serde(default)]
    pub html: Option<String>,
}
/// Kind and configuration of a new workflow step.
/// Omit `data` to configure later. Send the complete kind-specific payload when provided.
///
/// ```no_run
/// # async fn run(client: mailerlite_rs::MailerLite) -> mailerlite_rs::Result<()> {
/// use mailerlite_rs::automations::{CreateAutomationStep, StepType};
/// let step = client.automation_steps()
///     .create("123", &CreateAutomationStep::new(StepType::Delay))
///     .await?;
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CreateAutomationStep {
    /// `type`.
    #[serde(rename = "type")]
    pub step_type: StepType,
    /// Preceding step; omit to place first in the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id>,
    /// Branch of a condition or A/B parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<WorkflowBranch>,
    /// Where an existing condition branch is moved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_branch: Option<WorkflowBranch>,
    /// Complete kind-specific configuration as a JSON object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Map<String, Value>>,
}
impl CreateAutomationStep {
    /// Creates a payload.
    #[must_use]
    pub fn new(step_type: StepType) -> Self {
        Self {
            step_type,
            parent_id: None,
            branch: None,
            move_to_branch: None,
            data: None,
        }
    }
    /// Sets `parent_id`.
    #[must_use]
    pub fn parent_id(mut self, parent_id: impl Into<Id>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }
    /// Sets `branch`.
    #[must_use]
    pub fn branch(mut self, branch: WorkflowBranch) -> Self {
        self.branch = Some(branch);
        self
    }
    /// Sets `move_to_branch`.
    #[must_use]
    pub fn move_to_branch(mut self, move_to_branch: WorkflowBranch) -> Self {
        self.move_to_branch = Some(move_to_branch);
        self
    }
    /// Sets `data`.
    #[must_use]
    pub fn data(mut self, data: Map<String, Value>) -> Self {
        self.data = Some(data);
        self
    }
}
/// Complete kind-specific configuration, replacing the existing step settings.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UpdateAutomationStep {
    /// Complete kind-specific configuration as a JSON object.
    pub data: Map<String, Value>,
}
impl UpdateAutomationStep {
    /// Creates a payload.
    #[must_use]
    pub fn new(data: Map<String, Value>) -> Self {
        Self { data }
    }
}
/// `CopyAutomationStep` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct CopyAutomationStep {
    /// Preceding step; omit to place first in the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id>,
    /// Branch of a condition or A/B parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<WorkflowBranch>,
    /// Where an existing condition branch is moved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_branch: Option<WorkflowBranch>,
}
impl CopyAutomationStep {
    /// Creates a payload.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets `parent_id`.
    #[must_use]
    pub fn parent_id(mut self, parent_id: impl Into<Id>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }
    /// Sets `branch`.
    #[must_use]
    pub fn branch(mut self, branch: WorkflowBranch) -> Self {
        self.branch = Some(branch);
        self
    }
    /// Sets `move_to_branch`.
    #[must_use]
    pub fn move_to_branch(mut self, move_to_branch: WorkflowBranch) -> Self {
        self.move_to_branch = Some(move_to_branch);
        self
    }
}
/// `DeleteAutomationStep` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct DeleteAutomationStep {
    /// Branch to remove; `All` also removes following steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_to_delete: Option<WorkflowBranch>,
}
impl DeleteAutomationStep {
    /// Creates a payload.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets `branch_to_delete`.
    #[must_use]
    pub fn branch_to_delete(mut self, branch_to_delete: WorkflowBranch) -> Self {
        self.branch_to_delete = Some(branch_to_delete);
        self
    }
}
/// `ReorderAutomationSteps` payload.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ReorderAutomationSteps {
    /// Step to move.
    pub dragged_step_id: Id,
    /// New preceding step; omit to move to the beginning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_parent_step_id: Option<Id>,
    /// Branch of a condition or A/B parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<WorkflowBranch>,
    /// Condition branch to receive existing child steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_for_child: Option<WorkflowBranch>,
}
impl ReorderAutomationSteps {
    /// Creates a payload.
    #[must_use]
    pub fn new(dragged_step_id: Id) -> Self {
        Self {
            dragged_step_id,
            new_parent_step_id: None,
            branch: None,
            branch_for_child: None,
        }
    }
    /// Sets `new_parent_step_id`.
    #[must_use]
    pub fn new_parent_step_id(mut self, new_parent_step_id: impl Into<Id>) -> Self {
        self.new_parent_step_id = Some(new_parent_step_id.into());
        self
    }
    /// Sets `branch`.
    #[must_use]
    pub fn branch(mut self, branch: WorkflowBranch) -> Self {
        self.branch = Some(branch);
        self
    }
    /// Sets `branch_for_child`.
    #[must_use]
    pub fn branch_for_child(mut self, branch_for_child: WorkflowBranch) -> Self {
        self.branch_for_child = Some(branch_for_child);
        self
    }
}
/// `SelectAutomationWinner` payload.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SelectAutomationWinner {
    /// Winning A/B path.
    pub ab_test_winner: WorkflowBranch,
}
impl SelectAutomationWinner {
    /// Creates a payload.
    #[must_use]
    pub fn new(ab_test_winner: WorkflowBranch) -> Self {
        Self { ab_test_winner }
    }
}
/// `CreateAutomationTrigger` payload.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CreateAutomationTrigger {
    /// `type`.
    #[serde(rename = "type")]
    pub trigger_type: TriggerType,
    /// Complete kind-specific configuration as a JSON object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Map<String, Value>>,
}
impl CreateAutomationTrigger {
    /// Creates a payload.
    #[must_use]
    pub fn new(trigger_type: TriggerType) -> Self {
        Self {
            trigger_type,
            data: None,
        }
    }
    /// Sets `data`.
    #[must_use]
    pub fn data(mut self, data: Map<String, Value>) -> Self {
        self.data = Some(data);
        self
    }
}
/// Complete kind-specific configuration, replacing the existing trigger settings.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UpdateAutomationTrigger {
    /// Complete kind-specific configuration as a JSON object.
    pub data: Map<String, Value>,
}
impl UpdateAutomationTrigger {
    /// Creates a payload.
    #[must_use]
    pub fn new(data: Map<String, Value>) -> Self {
        Self { data }
    }
}
/// Updated trigger and automation settings.
/// Fields outside the public documentation are optional.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct AutomationTriggerUpdate {
    /// Updated trigger.
    #[serde(default)]
    pub trigger: Option<Trigger>,
    /// Trigger settings.
    #[serde(default)]
    pub trigger_data: Option<Map<String, Value>>,
    /// Subscribers that currently qualify.
    #[serde(default)]
    pub qualified_subscribers_count: Option<u64>,
}
/// Page of workflow subscribers in the required queued or completed state.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ListWorkflowSubscribers {
    /// `status`.
    pub status: WorkflowSubscriberStatus,
    /// Date or email sort, with ascending or descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort<WorkflowSubscriberSort>>,
    /// `filter[search]`.
    #[serde(rename = "filter[search]", skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// First day, `YYYY-MM-DD`.
    #[serde(rename = "filter[date_from]", skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// Last day, `YYYY-MM-DD`.
    #[serde(rename = "filter[date_to]", skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Comma-separated row ids.
    #[serde(rename = "filter[id]", skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
impl ListWorkflowSubscribers {
    /// Creates a payload.
    #[must_use]
    pub fn new(status: WorkflowSubscriberStatus) -> Self {
        Self {
            status,
            sort: None,
            search: None,
            date_from: None,
            date_to: None,
            ids: None,
            page: None,
            limit: None,
        }
    }
    /// Sets `sort`.
    #[must_use]
    pub fn sort(mut self, sort: Sort<WorkflowSubscriberSort>) -> Self {
        self.sort = Some(sort);
        self
    }
    /// Sets `filter[search]`.
    #[must_use]
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }
    /// Sets `page`.
    #[must_use]
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
    /// Sets `limit`.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}
impl ListWorkflowSubscribers {
    /// Sets `date_from`.
    #[must_use]
    pub fn date_from(mut self, value: impl Into<String>) -> Self {
        self.date_from = Some(value.into());
        self
    }
}
impl ListWorkflowSubscribers {
    /// Sets `date_to`.
    #[must_use]
    pub fn date_to(mut self, value: impl Into<String>) -> Self {
        self.date_to = Some(value.into());
        self
    }
}
impl ListWorkflowSubscribers {
    /// Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: impl Into<String>) -> Self {
        self.ids = Some(value.into());
        self
    }
}
/// A subscriber run at a workflow step or trigger.
/// Fields outside the public documentation are optional.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct WorkflowSubscriber {
    /// `id`.
    #[serde(default)]
    pub id: Option<Id>,
    /// Subscriber identity.
    #[serde(default)]
    pub subscriber: Option<super::ActivitySubscriber>,
    /// When the subscriber reached the step or trigger.
    #[serde(default)]
    pub date: Option<Timestamp>,
}
/// Template filters and page controls.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ListAutomationTemplates {
    /// `filter[type]`.
    #[serde(rename = "filter[type]", skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    /// `filter[category]`.
    #[serde(rename = "filter[category]", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
impl ListAutomationTemplates {
    /// Creates a payload.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets `filter[type]`.
    #[must_use]
    pub fn template_type(mut self, template_type: impl Into<String>) -> Self {
        self.template_type = Some(template_type.into());
        self
    }
    /// Sets `filter[category]`.
    #[must_use]
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
    /// Sets `page`.
    #[must_use]
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
    /// Sets `limit`.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}
/// Ready-made workflow returned by [`AutomationTemplates::list`].
/// Fields outside the public documentation are optional.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct AutomationTemplate {
    /// `id`.
    #[serde(default)]
    pub id: Option<Id>,
    /// Automation or template name.
    #[serde(default)]
    pub name: Option<String>,
    /// `type`.
    #[serde(rename = "type", default)]
    pub template_type: Option<String>,
    /// Template workflow; shape is not in the public documentation.
    #[serde(default)]
    pub workflow: Option<Value>,
    /// Template description.
    #[serde(default)]
    pub description: Option<String>,
    /// Short template description.
    #[serde(default)]
    pub excerpt: Option<String>,
    /// Template step descriptions; shape is not in the public documentation.
    #[serde(default)]
    pub workflow_steps_description: Option<Value>,
    /// Template preview URL.
    #[serde(default)]
    pub url: Option<String>,
}
/// Workflow endpoints obtained with [`MailerLite::automation_steps`].
#[derive(Clone, Copy, Debug)]
pub struct AutomationSteps<'a> {
    client: &'a MailerLite,
}
impl MailerLite {
    /// Workflow endpoints.
    #[must_use]
    pub fn automation_steps(&self) -> AutomationSteps<'_> {
        AutomationSteps { client: self }
    }
}
/// Workflow endpoints obtained with [`MailerLite::automation_triggers`].
#[derive(Clone, Copy, Debug)]
pub struct AutomationTriggers<'a> {
    client: &'a MailerLite,
}
impl MailerLite {
    /// Workflow endpoints.
    #[must_use]
    pub fn automation_triggers(&self) -> AutomationTriggers<'_> {
        AutomationTriggers { client: self }
    }
}
/// Workflow endpoints obtained with [`MailerLite::automation_templates`].
#[derive(Clone, Copy, Debug)]
pub struct AutomationTemplates<'a> {
    client: &'a MailerLite,
}
impl MailerLite {
    /// Workflow endpoints.
    #[must_use]
    pub fn automation_templates(&self) -> AutomationTemplates<'_> {
        AutomationTemplates { client: self }
    }
}
impl Automations<'_> {
    /// `POST /automations/{id}/copy`: copy.
    /// Copying builds the workflow in the background.
    pub async fn copy(self, id: impl AsRef<str>) -> Result<Automation> {
        self.client
            .call(Method::POST, &["automations", id.as_ref(), "copy"])
            .data()
            .await
    }
}
impl Automations<'_> {
    /// `POST /automations/{id}/enable`: enable.
    /// Enabling requires a complete workflow; disabling stops new entries.
    pub async fn enable(self, id: impl AsRef<str>) -> Result<Automation> {
        self.client
            .call(Method::POST, &["automations", id.as_ref(), "enable"])
            .data()
            .await
    }
}
impl Automations<'_> {
    /// `POST /automations/{id}/disable`: disable.
    /// Enabling requires a complete workflow; disabling stops new entries.
    pub async fn disable(self, id: impl AsRef<str>) -> Result<Automation> {
        self.client
            .call(Method::POST, &["automations", id.as_ref(), "disable"])
            .data()
            .await
    }
}
impl Automations<'_> {
    /// `PUT /automations/{id}`: update.
    ///
    pub async fn update(self, id: impl AsRef<str>, body: &UpdateAutomation) -> Result<Automation> {
        self.client
            .call(Method::PUT, &["automations", id.as_ref()])
            .json(body)
            .data()
            .await
    }
}
impl Automations<'_> {
    /// `POST /automations/{id}/test`: test.
    /// Sends every sendable email to a verified address; limited to 10 requests per hour.
    pub async fn test(self, id: impl AsRef<str>, email: &str) -> Result<()> {
        self.client
            .call(Method::POST, &["automations", id.as_ref(), "test"])
            .json(&serde_json::json!({"email": email}))
            .send_empty()
            .await
    }
}
impl Automations<'_> {
    /// `POST /automations/{id}/add-subscribers`: add subscribers.
    /// Runs in the background. Cancel and skip-delay require an active status filter; re-add requires an enabled automation.
    pub async fn add_subscribers(
        self,
        id: impl AsRef<str>,
        body: &AddAutomationSubscribers,
    ) -> Result<()> {
        self.client
            .call(
                Method::POST,
                &["automations", id.as_ref(), "add-subscribers"],
            )
            .json(body)
            .send_empty()
            .await
    }
}
impl Automations<'_> {
    /// `POST /automations/{id}/readd`: readd subscribers.
    /// Runs in the background. Cancel and skip-delay require an active status filter; re-add requires an enabled automation.
    pub async fn readd_subscribers(
        self,
        id: impl AsRef<str>,
        body: &ReaddAutomationSubscribers,
    ) -> Result<()> {
        self.client
            .call(Method::POST, &["automations", id.as_ref(), "readd"])
            .json(body)
            .send_empty()
            .await
    }
}
impl Automations<'_> {
    /// `POST /automations/{id}/activity/add-to-group`: add subscribers to group.
    /// Runs in the background. Cancel and skip-delay require an active status filter; re-add requires an enabled automation.
    pub async fn add_subscribers_to_group(
        self,
        id: impl AsRef<str>,
        body: &CopyAutomationSubscribers,
    ) -> Result<()> {
        self.client
            .call(
                Method::POST,
                &["automations", id.as_ref(), "activity", "add-to-group"],
            )
            .json(body)
            .send_empty()
            .await
    }
}
impl Automations<'_> {
    /// `POST /automations/{id}/activity/cancel-automation`: cancel subscribers.
    /// Runs in the background. Cancel and skip-delay require an active status filter; re-add requires an enabled automation.
    pub async fn cancel_subscribers(
        self,
        id: impl AsRef<str>,
        body: &AutomationRunAction,
    ) -> Result<()> {
        self.client
            .call(
                Method::POST,
                &["automations", id.as_ref(), "activity", "cancel-automation"],
            )
            .json(body)
            .send_empty()
            .await
    }
}
impl Automations<'_> {
    /// `POST /automations/{id}/activity/skip-delay`: skip delay.
    /// Runs in the background. Cancel and skip-delay require an active status filter; re-add requires an enabled automation.
    pub async fn skip_delay(self, id: impl AsRef<str>, body: &AutomationRunAction) -> Result<()> {
        self.client
            .call(
                Method::POST,
                &["automations", id.as_ref(), "activity", "skip-delay"],
            )
            .json(body)
            .send_empty()
            .await
    }
}
impl Automations<'_> {
    /// `GET /automations/{id}/emails/{email_id}/content`: email content.
    ///
    pub async fn email_content(
        self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
    ) -> Result<AutomationEmailContent> {
        self.client
            .call(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "emails",
                    email_id.as_ref(),
                    "content",
                ],
            )
            .data()
            .await
    }
}
impl Automations<'_> {
    /// `PUT /automations/{id}/emails/{email_id}/content`: update email content.
    /// Replaces the full HTML body; the automation must be inactive.
    pub async fn update_email_content(
        self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
        html: &str,
    ) -> Result<crate::campaigns::Email> {
        self.client
            .call(
                Method::PUT,
                &[
                    "automations",
                    id.as_ref(),
                    "emails",
                    email_id.as_ref(),
                    "content",
                ],
            )
            .json(&serde_json::json!({"html": html}))
            .data()
            .await
    }
}
impl Automations<'_> {
    /// `GET /automations/{id}/emails/{email_id}/report-stats`: email report stats.
    /// Not covered by the public docs; returns the full JSON report. Supply both dates or neither.
    pub async fn email_report_stats(
        self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
        query: &AutomationEmailStatsQuery,
    ) -> Result<Value> {
        self.client
            .call(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "emails",
                    email_id.as_ref(),
                    "report-stats",
                ],
            )
            .query(query)
            .send()
            .await
    }
}
impl Automations<'_> {
    /// `GET /automations/{id}/monthly-totals/email`: email stats.
    /// Not covered by the public docs; returns the full JSON report. Supply both dates or neither.
    pub async fn email_stats(
        self,
        id: impl AsRef<str>,
        query: &AutomationEmailStatsQuery,
    ) -> Result<Value> {
        self.client
            .call(
                Method::GET,
                &["automations", id.as_ref(), "monthly-totals", "email"],
            )
            .query(query)
            .send()
            .await
    }
}
impl Automations<'_> {
    /// `GET /automations/{id}/emails/{email_id}/unsubscribe-reasons`: email unsubscribe reasons.
    /// Not covered by the public docs; returns the full JSON report.
    pub async fn email_unsubscribe_reasons(
        self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
    ) -> Result<Value> {
        self.client
            .call(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "emails",
                    email_id.as_ref(),
                    "unsubscribe-reasons",
                ],
            )
            .send()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `GET /automations/{id}/steps/index`: list.
    ///
    pub async fn list(self, id: impl AsRef<str>) -> Result<Vec<Step>> {
        self.client
            .call(Method::GET, &["automations", id.as_ref(), "steps", "index"])
            .data()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `GET /automations/{id}/steps/{item_id}`: get.
    ///
    pub async fn get(self, id: impl AsRef<str>, item_id: impl AsRef<str>) -> Result<Step> {
        self.client
            .call(
                Method::GET,
                &["automations", id.as_ref(), "steps", item_id.as_ref()],
            )
            .data()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `POST /automations/{id}/steps`: create.
    /// The automation must be inactive. Configuration fields depend on the kind.
    pub async fn create(self, id: impl AsRef<str>, body: &CreateAutomationStep) -> Result<Step> {
        self.client
            .call(Method::POST, &["automations", id.as_ref(), "steps"])
            .json(body)
            .data()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `PUT /automations/{id}/steps/{item_id}`: update.
    /// Replaces the complete kind-specific configuration; omitted fields are cleared.
    pub async fn update(
        self,
        id: impl AsRef<str>,
        item_id: impl AsRef<str>,
        body: &UpdateAutomationStep,
    ) -> Result<Step> {
        self.client
            .call(
                Method::PUT,
                &["automations", id.as_ref(), "steps", item_id.as_ref()],
            )
            .json(body)
            .data()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `DELETE /automations/{id}/steps/{item_id}`: delete.
    /// The automation must be inactive. Branch deletion can remove following steps.
    pub async fn delete(
        self,
        id: impl AsRef<str>,
        item_id: impl AsRef<str>,
        query: &DeleteAutomationStep,
    ) -> Result<()> {
        self.client
            .call(
                Method::DELETE,
                &["automations", id.as_ref(), "steps", item_id.as_ref()],
            )
            .query(query)
            .send_empty()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `GET /automations/{id}/steps/run-counts`: run counts.
    /// Not covered by the public docs; returns the full JSON report.
    pub async fn run_counts(self, id: impl AsRef<str>) -> Result<Value> {
        self.client
            .call(
                Method::GET,
                &["automations", id.as_ref(), "steps", "run-counts"],
            )
            .send()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `GET /automations/{id}/steps/run-counts/{item_id}`: run count.
    /// Not covered by the public docs; returns the full JSON report.
    pub async fn run_count(self, id: impl AsRef<str>, item_id: impl AsRef<str>) -> Result<Value> {
        self.client
            .call(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "steps",
                    "run-counts",
                    item_id.as_ref(),
                ],
            )
            .send()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `GET /automations/{id}/activity/steps/{item_id}/subscribers`: subscribers.
    ///
    pub async fn subscribers(
        self,
        id: impl AsRef<str>,
        item_id: impl AsRef<str>,
        query: &ListWorkflowSubscribers,
    ) -> Result<Page<WorkflowSubscriber>> {
        self.client
            .call(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "activity",
                    "steps",
                    item_id.as_ref(),
                    "subscribers",
                ],
            )
            .query(query)
            .send()
            .await
    }
}
impl AutomationTriggers<'_> {
    /// `GET /automations/{id}/triggers`: list.
    ///
    pub async fn list(self, id: impl AsRef<str>) -> Result<Vec<Trigger>> {
        self.client
            .call(Method::GET, &["automations", id.as_ref(), "triggers"])
            .data()
            .await
    }
}
impl AutomationTriggers<'_> {
    /// `GET /automations/{id}/triggers/{item_id}`: get.
    ///
    pub async fn get(self, id: impl AsRef<str>, item_id: impl AsRef<str>) -> Result<Trigger> {
        self.client
            .call(
                Method::GET,
                &["automations", id.as_ref(), "triggers", item_id.as_ref()],
            )
            .data()
            .await
    }
}
impl AutomationTriggers<'_> {
    /// `POST /automations/{id}/triggers`: create.
    /// The automation must be inactive. Configuration fields depend on the kind.
    pub async fn create(
        self,
        id: impl AsRef<str>,
        body: &CreateAutomationTrigger,
    ) -> Result<Trigger> {
        self.client
            .call(Method::POST, &["automations", id.as_ref(), "triggers"])
            .json(body)
            .data()
            .await
    }
}
impl AutomationTriggers<'_> {
    /// `PUT /automations/{id}/triggers/{item_id}`: update.
    /// Replaces the complete kind-specific configuration; omitted fields are cleared.
    pub async fn update(
        self,
        id: impl AsRef<str>,
        item_id: impl AsRef<str>,
        body: &UpdateAutomationTrigger,
    ) -> Result<AutomationTriggerUpdate> {
        self.client
            .call(
                Method::PUT,
                &["automations", id.as_ref(), "triggers", item_id.as_ref()],
            )
            .json(body)
            .data()
            .await
    }
}
impl AutomationTriggers<'_> {
    /// `DELETE /automations/{id}/triggers/{item_id}`: delete.
    /// The automation must be inactive. Branch deletion can remove following steps.
    pub async fn delete(self, id: impl AsRef<str>, item_id: impl AsRef<str>) -> Result<()> {
        self.client
            .call(
                Method::DELETE,
                &["automations", id.as_ref(), "triggers", item_id.as_ref()],
            )
            .send_empty()
            .await
    }
}
impl AutomationTriggers<'_> {
    /// `GET /automations/{id}/triggers/run-counts`: run counts.
    /// Not covered by the public docs; returns the full JSON report.
    pub async fn run_counts(self, id: impl AsRef<str>) -> Result<Value> {
        self.client
            .call(
                Method::GET,
                &["automations", id.as_ref(), "triggers", "run-counts"],
            )
            .send()
            .await
    }
}
impl AutomationTriggers<'_> {
    /// `GET /automations/{id}/triggers/run-counts/{item_id}`: run count.
    /// Not covered by the public docs; returns the full JSON report.
    pub async fn run_count(self, id: impl AsRef<str>, item_id: impl AsRef<str>) -> Result<Value> {
        self.client
            .call(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "triggers",
                    "run-counts",
                    item_id.as_ref(),
                ],
            )
            .send()
            .await
    }
}
impl AutomationTriggers<'_> {
    /// `GET /automations/{id}/activity/triggers/{item_id}/subscribers`: subscribers.
    ///
    pub async fn subscribers(
        self,
        id: impl AsRef<str>,
        item_id: impl AsRef<str>,
        query: &ListWorkflowSubscribers,
    ) -> Result<Page<WorkflowSubscriber>> {
        self.client
            .call(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "activity",
                    "triggers",
                    item_id.as_ref(),
                    "subscribers",
                ],
            )
            .query(query)
            .send()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `POST /automations/{id}/steps/{item_id}/copy`: copy.
    ///
    pub async fn copy(
        self,
        id: impl AsRef<str>,
        item_id: impl AsRef<str>,
        body: &CopyAutomationStep,
    ) -> Result<Automation> {
        self.client
            .call(
                Method::POST,
                &[
                    "automations",
                    id.as_ref(),
                    "steps",
                    item_id.as_ref(),
                    "copy",
                ],
            )
            .json(body)
            .data()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `POST /automations/{id}/steps/reorder`: reorder.
    ///
    pub async fn reorder(
        self,
        id: impl AsRef<str>,
        body: &ReorderAutomationSteps,
    ) -> Result<Automation> {
        self.client
            .call(
                Method::POST,
                &["automations", id.as_ref(), "steps", "reorder"],
            )
            .json(body)
            .data()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `PUT /automations/{id}/steps/{item_id}/select-ab-test-winner`: select ab test winner.
    /// Not covered by the public docs; returns the full JSON A/B report.
    pub async fn select_ab_test_winner(
        self,
        id: impl AsRef<str>,
        item_id: impl AsRef<str>,
        body: &SelectAutomationWinner,
    ) -> Result<Value> {
        self.client
            .call(
                Method::PUT,
                &[
                    "automations",
                    id.as_ref(),
                    "steps",
                    item_id.as_ref(),
                    "select-ab-test-winner",
                ],
            )
            .json(body)
            .send()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `GET /automations/{id}/ab-steps-stats`: ab stats.
    /// Not covered by the public docs; returns the full JSON A/B report.
    pub async fn ab_stats(self, id: impl AsRef<str>) -> Result<Value> {
        self.client
            .call(Method::GET, &["automations", id.as_ref(), "ab-steps-stats"])
            .send()
            .await
    }
}
impl AutomationSteps<'_> {
    /// `GET /automations/{id}/ab-steps-stats/{item_id}`: ab stat.
    /// Not covered by the public docs; returns the full JSON A/B report.
    pub async fn ab_stat(self, id: impl AsRef<str>, item_id: impl AsRef<str>) -> Result<Value> {
        self.client
            .call(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "ab-steps-stats",
                    item_id.as_ref(),
                ],
            )
            .send()
            .await
    }
}
impl AutomationTemplates<'_> {
    /// `GET /automation-templates`: list.
    ///
    pub async fn list(self, query: &ListAutomationTemplates) -> Result<Page<AutomationTemplate>> {
        self.client
            .call(Method::GET, &["automation-templates"])
            .query(query)
            .send()
            .await
    }
}
impl AutomationTemplates<'_> {
    /// `POST /automations/select-template/{id}`: select.
    /// Builds an inactive automation in the background; poll `trigger_data.building_in_progress` before enabling.
    pub async fn select(self, id: impl AsRef<str>) -> Result<Automation> {
        self.client
            .call(
                Method::POST,
                &["automations", "select-template", id.as_ref()],
            )
            .data()
            .await
    }
}

#[cfg(feature = "stream")]
impl<'a> AutomationSteps<'a> {
    /// Every matching subscriber, fetching pages lazily.
    pub fn subscribers_all(
        self,
        id: impl Into<String>,
        item_id: impl Into<String>,
        query: ListWorkflowSubscribers,
    ) -> impl futures_util::Stream<Item = Result<WorkflowSubscriber>> + 'a {
        let id = id.into();
        let item_id = item_id.into();
        crate::pagination::stream::paged(query, move |query| {
            let id = id.clone();
            let item_id = item_id.clone();
            async move { self.subscribers(id, item_id, &query).await }
        })
    }
}

#[cfg(feature = "stream")]
impl<'a> AutomationTriggers<'a> {
    /// Every matching subscriber, fetching pages lazily.
    pub fn subscribers_all(
        self,
        id: impl Into<String>,
        item_id: impl Into<String>,
        query: ListWorkflowSubscribers,
    ) -> impl futures_util::Stream<Item = Result<WorkflowSubscriber>> + 'a {
        let id = id.into();
        let item_id = item_id.into();
        crate::pagination::stream::paged(query, move |query| {
            let id = id.clone();
            let item_id = item_id.clone();
            async move { self.subscribers(id, item_id, &query).await }
        })
    }
}

#[cfg(feature = "stream")]
impl<'a> AutomationTemplates<'a> {
    /// Every template, fetching pages lazily.
    pub fn list_all(
        self,
        query: ListAutomationTemplates,
    ) -> impl futures_util::Stream<Item = Result<AutomationTemplate>> + 'a {
        crate::pagination::stream::paged(query, move |query| async move { self.list(&query).await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::mock::{client, fixture};
    use serde_json::json;
    #[tokio::test]
    async fn automations_copy() {
        let (client, mock) = client();
        mock.respond(200, fixture("automations_get"));
        let result = client.automations().copy("1").await.unwrap();
        assert_eq!(result.id, "57444257882965766");
        mock.request()
            .assert(Method::POST, "automations/1/copy")
            .no_body();
    }
    #[tokio::test]
    async fn automations_enable() {
        let (client, mock) = client();
        mock.respond(200, fixture("automations_get"));
        let result = client.automations().enable("1").await.unwrap();
        assert_eq!(result.id, "57444257882965766");
        mock.request()
            .assert(Method::POST, "automations/1/enable")
            .no_body();
    }
    #[tokio::test]
    async fn automations_disable() {
        let (client, mock) = client();
        mock.respond(200, fixture("automations_get"));
        let result = client.automations().disable("1").await.unwrap();
        assert_eq!(result.id, "57444257882965766");
        mock.request()
            .assert(Method::POST, "automations/1/disable")
            .no_body();
    }
    #[tokio::test]
    async fn automations_update() {
        let (client, mock) = client();
        mock.respond(200, fixture("automations_get"));
        let result = client
            .automations()
            .update("1", &UpdateAutomation::new("Updated"))
            .await
            .unwrap();
        assert_eq!(result.id, "57444257882965766");
        mock.request()
            .assert(Method::PUT, "automations/1")
            .body(&json!({"name": "Updated"}));
    }
    #[tokio::test]
    async fn automations_test() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);
        client
            .automations()
            .test("1", "ann@example.com")
            .await
            .unwrap();
        mock.request()
            .assert(Method::POST, "automations/1/test")
            .body(&json!({"email": "ann@example.com"}));
    }
    #[tokio::test]
    async fn automations_add_subscribers() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);
        client
            .automations()
            .add_subscribers(
                "1",
                &AddAutomationSubscribers::new()
                    .emails(vec!["ann@example.com".into()])
                    .groups(vec!["9".into()])
                    .step_id("2"),
            )
            .await
            .unwrap();
        mock.request()
            .assert(Method::POST, "automations/1/add-subscribers")
            .body(&json!({"emails": ["ann@example.com"], "groups": ["9"], "step_id": "2"}));
    }
    #[tokio::test]
    async fn automations_readd_subscribers() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);
        client
            .automations()
            .readd_subscribers(
                "1",
                &ReaddAutomationSubscribers::new(
                    AutomationRunFilter::new(AutomationActivityStatus::Completed),
                    ReaddTo::SelectedStep,
                )
                .selected_step_id("2"),
            )
            .await
            .unwrap();
        mock.request().assert(Method::POST, "automations/1/readd").body(&json!({"filter": {"status": "completed"}, "readd_subscribers_to": "selected_step", "selected_step_id": "2"}));
    }
    #[tokio::test]
    async fn automations_add_subscribers_to_group() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);
        client
            .automations()
            .add_subscribers_to_group(
                "1",
                &CopyAutomationSubscribers::new(
                    AutomationRunFilter::new(AutomationActivityStatus::Active),
                    vec!["9".into()],
                ),
            )
            .await
            .unwrap();
        mock.request()
            .assert(Method::POST, "automations/1/activity/add-to-group")
            .body(&json!({"filter": {"status": "active"}, "destination_groups": ["9"]}));
    }
    #[tokio::test]
    async fn automations_cancel_subscribers() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);
        client
            .automations()
            .cancel_subscribers(
                "1",
                &AutomationRunAction::new(
                    AutomationRunFilter::new(AutomationActivityStatus::Active).id(vec!["7".into()]),
                ),
            )
            .await
            .unwrap();
        mock.request()
            .assert(Method::POST, "automations/1/activity/cancel-automation")
            .body(&json!({"filter": {"status": "active", "id": ["7"]}}));
    }
    #[tokio::test]
    async fn automations_skip_delay() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);
        client
            .automations()
            .skip_delay(
                "1",
                &AutomationRunAction::new(
                    AutomationRunFilter::new(AutomationActivityStatus::Active).id(vec!["7".into()]),
                ),
            )
            .await
            .unwrap();
        mock.request()
            .assert(Method::POST, "automations/1/activity/skip-delay")
            .body(&json!({"filter": {"status": "active", "id": ["7"]}}));
    }
    #[tokio::test]
    async fn automations_email_content() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"html":"<p>Hello</p>"}}));
        let result = client.automations().email_content("1", "2").await.unwrap();
        assert_eq!(result.html.as_deref(), Some("<p>Hello</p>"));
        mock.request()
            .assert(Method::GET, "automations/1/emails/2/content")
            .no_body();
    }
    #[tokio::test]
    async fn automations_update_email_content() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"id":"2"}}));
        let result = client
            .automations()
            .update_email_content("1", "2", "<p>Hello</p>")
            .await
            .unwrap();
        let _ = result;
        mock.request()
            .assert(Method::PUT, "automations/1/emails/2/content")
            .body(&json!({"html": "<p>Hello</p>"}));
    }
    #[tokio::test]
    async fn automations_email_report_stats() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"example":true}}));
        let result = client
            .automations()
            .email_report_stats(
                "1",
                "2",
                &AutomationEmailStatsQuery::new()
                    .start_date("2026-01-01")
                    .end_date("2026-01-31"),
            )
            .await
            .unwrap();
        assert_eq!(result, json!({"data":{"example":true}}));
        mock.request()
            .assert(
                Method::GET,
                "automations/1/emails/2/report-stats?start_date=2026-01-01&end_date=2026-01-31",
            )
            .no_body();
    }
    #[tokio::test]
    async fn automations_email_stats() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"example":true}}));
        let result = client
            .automations()
            .email_stats(
                "1",
                &AutomationEmailStatsQuery::new()
                    .start_date("2026-01-01")
                    .end_date("2026-01-31"),
            )
            .await
            .unwrap();
        assert_eq!(result, json!({"data":{"example":true}}));
        mock.request()
            .assert(
                Method::GET,
                "automations/1/monthly-totals/email?start_date=2026-01-01&end_date=2026-01-31",
            )
            .no_body();
    }
    #[tokio::test]
    async fn automations_email_unsubscribe_reasons() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"example":true}}));
        let result = client
            .automations()
            .email_unsubscribe_reasons("1", "2")
            .await
            .unwrap();
        assert_eq!(result, json!({"data":{"example":true}}));
        mock.request()
            .assert(Method::GET, "automations/1/emails/2/unsubscribe-reasons")
            .no_body();
    }
    #[tokio::test]
    async fn automation_steps_list() {
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data": fixture("automations_get")["data"]["steps"].clone()}),
        );
        let result = client.automation_steps().list("1").await.unwrap();
        let _ = result;
        mock.request()
            .assert(Method::GET, "automations/1/steps/index")
            .no_body();
    }
    #[tokio::test]
    async fn automation_steps_get() {
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data": fixture("automations_get")["data"]["steps"][0].clone()}),
        );
        let result = client.automation_steps().get("1", "2").await.unwrap();
        assert_eq!(result.step_type, StepType::Delay);
        mock.request()
            .assert(Method::GET, "automations/1/steps/2")
            .no_body();
    }
    #[tokio::test]
    async fn automation_steps_create() {
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data": fixture("automations_get")["data"]["steps"][0].clone()}),
        );
        let result = client
            .automation_steps()
            .create(
                "1",
                &CreateAutomationStep::new(StepType::Delay)
                    .parent_id("2")
                    .branch(WorkflowBranch::Yes)
                    .move_to_branch(WorkflowBranch::No)
                    .data(serde_json::from_value(json!({"unit":"days","value":1})).unwrap()),
            )
            .await
            .unwrap();
        assert_eq!(result.step_type, StepType::Delay);
        mock.request().assert(Method::POST, "automations/1/steps").body(&json!({"type": "delay", "parent_id": "2", "branch": "yes", "move_to_branch": "no", "data": {"unit": "days", "value": 1}}));
    }
    #[tokio::test]
    async fn automation_steps_update() {
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data": fixture("automations_get")["data"]["steps"][0].clone()}),
        );
        let result = client
            .automation_steps()
            .update(
                "1",
                "2",
                &UpdateAutomationStep::new(
                    serde_json::from_value(json!({"unit":"days","value":2})).unwrap(),
                ),
            )
            .await
            .unwrap();
        assert_eq!(result.step_type, StepType::Delay);
        mock.request()
            .assert(Method::PUT, "automations/1/steps/2")
            .body(&json!({"data": {"unit": "days", "value": 2}}));
    }
    #[tokio::test]
    async fn automation_steps_delete() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);
        client
            .automation_steps()
            .delete(
                "1",
                "2",
                &DeleteAutomationStep::new().branch_to_delete(WorkflowBranch::All),
            )
            .await
            .unwrap();
        mock.request()
            .assert(Method::DELETE, "automations/1/steps/2?branch_to_delete=all")
            .no_body();
    }
    #[tokio::test]
    async fn automation_steps_run_counts() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"example":true}}));
        let result = client.automation_steps().run_counts("1").await.unwrap();
        assert_eq!(result, json!({"data":{"example":true}}));
        mock.request()
            .assert(Method::GET, "automations/1/steps/run-counts")
            .no_body();
    }
    #[tokio::test]
    async fn automation_steps_run_count() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"example":true}}));
        let result = client.automation_steps().run_count("1", "2").await.unwrap();
        assert_eq!(result, json!({"data":{"example":true}}));
        mock.request()
            .assert(Method::GET, "automations/1/steps/run-counts/2")
            .no_body();
    }
    #[tokio::test]
    async fn automation_steps_subscribers() {
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data":[],"meta":{"current_page":1,"last_page":1}}),
        );
        let result = client
            .automation_steps()
            .subscribers(
                "1",
                "2",
                &ListWorkflowSubscribers::new(WorkflowSubscriberStatus::Completed)
                    .sort(Sort::asc(WorkflowSubscriberSort::Date))
                    .search("ann@example.com")
                    .page(2)
                    .limit(10),
            )
            .await
            .unwrap();
        let _ = result;
        mock.request().assert(Method::GET, "automations/1/activity/steps/2/subscribers?status=completed&sort=date&filter%5Bsearch%5D=ann%40example.com&page=2&limit=10").no_body();
    }
    #[tokio::test]
    async fn automation_triggers_list() {
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data": fixture("automations_list")["data"][0]["triggers"].clone()}),
        );
        let result = client.automation_triggers().list("1").await.unwrap();
        let _ = result;
        mock.request()
            .assert(Method::GET, "automations/1/triggers")
            .no_body();
    }
    #[tokio::test]
    async fn automation_triggers_get() {
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data": fixture("automations_list")["data"][0]["triggers"][0].clone()}),
        );
        let result = client.automation_triggers().get("1", "2").await.unwrap();
        assert_eq!(result.trigger_type, TriggerType::SubscriberJoinsGroup);
        mock.request()
            .assert(Method::GET, "automations/1/triggers/2")
            .no_body();
    }
    #[tokio::test]
    async fn automation_triggers_create() {
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data": fixture("automations_list")["data"][0]["triggers"][0].clone()}),
        );
        let result = client
            .automation_triggers()
            .create(
                "1",
                &CreateAutomationTrigger::new(TriggerType::SubscriberJoinsGroup)
                    .data(serde_json::from_value(json!({"group_ids":["9"]})).unwrap()),
            )
            .await
            .unwrap();
        assert_eq!(result.trigger_type, TriggerType::SubscriberJoinsGroup);
        mock.request()
            .assert(Method::POST, "automations/1/triggers")
            .body(&json!({"type": "subscriber_joins_group", "data": {"group_ids": ["9"]}}));
    }
    #[tokio::test]
    async fn automation_triggers_update() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"trigger":fixture("automations_list")["data"][0]["triggers"][0].clone(),"trigger_data":{},"qualified_subscribers_count":2}}));
        let result = client
            .automation_triggers()
            .update(
                "1",
                "2",
                &UpdateAutomationTrigger::new(
                    serde_json::from_value(json!({"group_ids":["9"]})).unwrap(),
                ),
            )
            .await
            .unwrap();
        let _ = result;
        mock.request()
            .assert(Method::PUT, "automations/1/triggers/2")
            .body(&json!({"data": {"group_ids": ["9"]}}));
    }
    #[tokio::test]
    async fn automation_triggers_delete() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);
        client.automation_triggers().delete("1", "2").await.unwrap();
        mock.request()
            .assert(Method::DELETE, "automations/1/triggers/2")
            .no_body();
    }
    #[tokio::test]
    async fn automation_triggers_run_counts() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"example":true}}));
        let result = client.automation_triggers().run_counts("1").await.unwrap();
        assert_eq!(result, json!({"data":{"example":true}}));
        mock.request()
            .assert(Method::GET, "automations/1/triggers/run-counts")
            .no_body();
    }
    #[tokio::test]
    async fn automation_triggers_run_count() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"example":true}}));
        let result = client
            .automation_triggers()
            .run_count("1", "2")
            .await
            .unwrap();
        assert_eq!(result, json!({"data":{"example":true}}));
        mock.request()
            .assert(Method::GET, "automations/1/triggers/run-counts/2")
            .no_body();
    }
    #[tokio::test]
    async fn automation_triggers_subscribers() {
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data":[],"meta":{"current_page":1,"last_page":1}}),
        );
        let result = client
            .automation_triggers()
            .subscribers(
                "1",
                "2",
                &ListWorkflowSubscribers::new(WorkflowSubscriberStatus::Completed)
                    .sort(Sort::asc(WorkflowSubscriberSort::Date))
                    .search("ann@example.com")
                    .page(2)
                    .limit(10),
            )
            .await
            .unwrap();
        let _ = result;
        mock.request().assert(Method::GET, "automations/1/activity/triggers/2/subscribers?status=completed&sort=date&filter%5Bsearch%5D=ann%40example.com&page=2&limit=10").no_body();
    }
    #[tokio::test]
    async fn automation_steps_copy() {
        let (client, mock) = client();
        mock.respond(200, fixture("automations_get"));
        let result = client
            .automation_steps()
            .copy(
                "1",
                "2",
                &CopyAutomationStep::new()
                    .parent_id("2")
                    .branch(WorkflowBranch::Yes)
                    .move_to_branch(WorkflowBranch::No),
            )
            .await
            .unwrap();
        assert_eq!(result.id, "57444257882965766");
        mock.request()
            .assert(Method::POST, "automations/1/steps/2/copy")
            .body(&json!({"parent_id": "2", "branch": "yes", "move_to_branch": "no"}));
    }
    #[tokio::test]
    async fn automation_steps_reorder() {
        let (client, mock) = client();
        mock.respond(200, fixture("automations_get"));
        let result = client
            .automation_steps()
            .reorder(
                "1",
                &ReorderAutomationSteps::new("2".into())
                    .new_parent_step_id("3")
                    .branch(WorkflowBranch::Yes)
                    .branch_for_child(WorkflowBranch::No),
            )
            .await
            .unwrap();
        assert_eq!(result.id, "57444257882965766");
        mock.request().assert(Method::POST, "automations/1/steps/reorder").body(&json!({"dragged_step_id": "2", "new_parent_step_id": "3", "branch": "yes", "branch_for_child": "no"}));
    }
    #[tokio::test]
    async fn automation_steps_select_ab_test_winner() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"example":true}}));
        let result = client
            .automation_steps()
            .select_ab_test_winner(
                "1",
                "2",
                &SelectAutomationWinner::new(WorkflowBranch::APath),
            )
            .await
            .unwrap();
        assert_eq!(result, json!({"data":{"example":true}}));
        mock.request()
            .assert(Method::PUT, "automations/1/steps/2/select-ab-test-winner")
            .body(&json!({"ab_test_winner": "a_path"}));
    }
    #[tokio::test]
    async fn automation_steps_ab_stats() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"example":true}}));
        let result = client.automation_steps().ab_stats("1").await.unwrap();
        assert_eq!(result, json!({"data":{"example":true}}));
        mock.request()
            .assert(Method::GET, "automations/1/ab-steps-stats")
            .no_body();
    }
    #[tokio::test]
    async fn automation_steps_ab_stat() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":{"example":true}}));
        let result = client.automation_steps().ab_stat("1", "2").await.unwrap();
        assert_eq!(result, json!({"data":{"example":true}}));
        mock.request()
            .assert(Method::GET, "automations/1/ab-steps-stats/2")
            .no_body();
    }
    #[tokio::test]
    async fn automation_templates_list() {
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data":[],"meta":{"current_page":1,"last_page":1}}),
        );
        let result = client
            .automation_templates()
            .list(
                &ListAutomationTemplates::new()
                    .template_type("welcome")
                    .category("onboarding")
                    .page(2)
                    .limit(10),
            )
            .await
            .unwrap();
        let _ = result;
        mock.request().assert(Method::GET, "automation-templates?filter%5Btype%5D=welcome&filter%5Bcategory%5D=onboarding&page=2&limit=10").no_body();
    }
    #[tokio::test]
    async fn automation_templates_select() {
        let (client, mock) = client();
        mock.respond(200, fixture("automations_get"));
        let result = client.automation_templates().select("1").await.unwrap();
        assert_eq!(result.id, "57444257882965766");
        mock.request()
            .assert(Method::POST, "automations/select-template/1")
            .no_body();
    }
    #[tokio::test]
    async fn workflow_filters_encoding_and_build_state() {
        let (client, mock) = client();
        mock.respond(200, json!({"data":[{"id":42,"subscriber":{"id":8,"email":"ann@example.com"},"date":"2026-01-01 00:00:00"}]}));
        let page = client
            .automation_steps()
            .subscribers(
                "a/b",
                "x y",
                &ListWorkflowSubscribers::new(WorkflowSubscriberStatus::Queued)
                    .date_from("2026-01-01")
                    .date_to("2026-01-31")
                    .ids("42,43"),
            )
            .await
            .unwrap();
        assert_eq!(page.data[0].id.as_ref().unwrap(), "42");
        assert_eq!(
            page.data[0].subscriber.as_ref().unwrap().email,
            "ann@example.com"
        );
        mock.request().assert(Method::GET,
            "automations/a%2Fb/activity/steps/x%20y/subscribers?status=queued&filter%5Bdate_from%5D=2026-01-01&filter%5Bdate_to%5D=2026-01-31&filter%5Bid%5D=42%2C43");
        let step: Step = serde_json::from_value(json!({"id":1,"type":"future_kind"})).unwrap();
        assert_eq!(step.step_type, StepType::Other("future_kind".into()));
        let trigger_data: super::super::TriggerData =
            serde_json::from_value(json!({"building_in_progress":true})).unwrap();
        assert_eq!(trigger_data.building_in_progress, Some(true));
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn templates_stream_walks_pages() {
        use futures_util::TryStreamExt;
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data":[{"id":1,"name":"Welcome"}],"meta":{"current_page":1,"last_page":2}}),
        )
        .respond(
            200,
            json!({"data":[{"id":"2","name":"Follow up"}],"meta":{"current_page":2,"last_page":2}}),
        );
        let templates: Vec<_> = client
            .automation_templates()
            .list_all(ListAutomationTemplates::new())
            .try_collect()
            .await
            .unwrap();
        assert_eq!(templates.len(), 2);
        assert_eq!(templates[1].id.as_ref().unwrap(), "2");
        let requests = mock.requests();
        requests[0].assert(Method::GET, "automation-templates?page=1");
        requests[1].assert(Method::GET, "automation-templates?page=2");
    }
}
