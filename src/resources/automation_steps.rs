//! Automation workflow steps, counts, A/B results, and subscriber activity.
//!
//! Contracts are verified against the MailerLite backend's step controllers,
//! request validators, and JSON resources. Step edits require an inactive automation.
use crate::models::automation_configuration::{
    STEP_FIELDS, StepConfiguration, configuration_fields,
};
use crate::{
    Error, MailerLite, Response, Result,
    http::OperationPolicy,
    models::{
        Patch,
        automation_steps::{AbTestStats, StepRunCounts, StepSubscriber},
        automations::{Automation, AutomationStep},
    },
    pagination::NumberedPage,
};
use reqwest::Method;
use serde::Serialize;
use serde_json::{Map, Value};

/// Kind of automation step to create.
pub use crate::models::automation_configuration::StepKind as StepType;
/// A condition or A/B branch.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StepBranch {
    /// Matching condition branch.
    Yes,
    /// Nonmatching condition branch.
    No,
    /// A/B path A.
    APath,
    /// A/B path B.
    BPath,
    /// A/B path C.
    CPath,
}
/// A branch of a condition step.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionBranch {
    /// Matching branch.
    Yes,
    /// Nonmatching branch.
    No,
}
/// Path selected as the A/B winner.
pub use crate::models::automation_configuration::AbPath as AbWinner;
/// Parameters for creating a step.
#[derive(Clone, Serialize)]
pub struct CreateStep {
    #[serde(rename = "type")]
    kind: StepType,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<StepBranch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    move_to_branch: Option<ConditionBranch>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    data: Map<String, Value>,
}
impl CreateStep {
    /// Start a step with its required type; omit data to create an empty step.
    pub fn new(kind: StepType) -> Self {
        Self {
            kind,
            parent_id: None,
            branch: None,
            move_to_branch: None,
            data: Map::new(),
        }
    }
    /// Place the step after this parent; omission creates a first step.
    pub fn parent_id(mut self, id: impl Into<String>) -> Self {
        self.parent_id = Some(id.into());
        self
    }
    /// Select a branch of the parent. Required for condition parents.
    pub fn branch(mut self, branch: StepBranch) -> Self {
        self.branch = Some(branch);
        self
    }
    /// Move an existing child into this condition branch.
    pub fn move_to_branch(mut self, branch: ConditionBranch) -> Self {
        self.move_to_branch = Some(branch);
        self
    }
    /// Construct a step from typed configuration, selecting its matching type.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_steps::CreateStep;
    /// use mailerlite_rs::models::automation_configuration::{
    ///     DelayConfiguration, DelayUnit, StepConfiguration,
    /// };
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let configuration = StepConfiguration::Delay(
    ///     DelayConfiguration::new(DelayUnit::Days, 1),
    /// );
    /// let params = CreateStep::configured(configuration)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Serialize`] if serialization fails or [`crate::Error::Config`] if
    /// configuration does not produce an object. The type is selected from the configuration
    /// automatically; server validation still applies on submission.
    pub fn configured(configuration: StepConfiguration) -> Result<Self> {
        Self::new(configuration.kind()).configuration(configuration)
    }
    /// Replace configuration with the matching typed payload.
    ///
    /// A configuration for a different step type is rejected locally.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_steps::{CreateStep, StepType};
    /// use mailerlite_rs::models::automation_configuration::{
    ///     DelayConfiguration, DelayUnit, StepConfiguration,
    /// };
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let configuration = StepConfiguration::Delay(
    ///     DelayConfiguration::new(DelayUnit::Days, 1),
    /// );
    /// let params = CreateStep::new(StepType::Delay).configuration(configuration)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Serialize`] if configuration serialization fails, or
    /// [`crate::Error::Config`] if it does not produce an object. A configuration with a different
    /// step type also returns `Error::Config`.
    pub fn configuration(mut self, configuration: StepConfiguration) -> Result<Self> {
        if self.kind != configuration.kind() {
            return Err(Error::Config(
                "step type does not match its configuration".into(),
            ));
        }
        self.data = configuration_fields(&configuration)?;
        Ok(self)
    }
    /// Add a future configuration field. Known fields must use typed configuration.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_steps::{CreateStep, StepType};
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let params = CreateStep::new(StepType::Delay).extension_data("future_setting", true)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] if the key is a modeled step field. Unknown keys are only
    /// an escape hatch for future server fields; acceptance is checked by the API.
    pub fn extension_data(
        mut self,
        field: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Self> {
        let field = field.into();
        if STEP_FIELDS.contains(&field.as_str()) {
            return Err(Error::Config(
                "use typed configuration for known step fields".into(),
            ));
        }
        self.data.insert(field, value.into());
        Ok(self)
    }
}
/// Parameters for replacing a step's configuration.
#[derive(Clone, Default, Serialize)]
pub struct UpdateStep {
    #[serde(skip_serializing_if = "Patch::is_unset")]
    parent_id: Patch<String>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    data: Map<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reschedule_waiting_subscribers: Option<bool>,
}
impl UpdateStep {
    /// Start an update. Supply the complete data payload for the existing step type.
    pub fn new() -> Self {
        Self::default()
    }
    /// Move the step below a parent; omission leaves its position unchanged.
    pub fn parent_id(mut self, id: impl Into<String>) -> Self {
        self.parent_id = Patch::Value(id.into());
        self
    }
    /// Send explicit null for the parent identifier.
    pub fn clear_parent(mut self) -> Self {
        self.parent_id = Patch::Null;
        self
    }
    /// Set typed configuration for the existing step's type.
    ///
    /// The server knows the existing type and rejects incompatible payloads.
    /// Omission and null in this payload preserve their wire distinction; the
    /// backend replaces configuration, so omitted settings can be cleared.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_steps::UpdateStep;
    /// use mailerlite_rs::models::automation_configuration::{
    ///     DelayConfiguration, DelayUnit, StepConfiguration,
    /// };
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let configuration = StepConfiguration::Delay(
    ///     DelayConfiguration::new(DelayUnit::Days, 1),
    /// );
    /// let params = UpdateStep::new().configuration(configuration)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Serialize`] if configuration serialization fails, or
    /// [`crate::Error::Config`] if it does not produce an object. The server, not this helper,
    /// checks compatibility with the existing step type.
    pub fn configuration(mut self, configuration: StepConfiguration) -> Result<Self> {
        self.data = configuration_fields(&configuration)?;
        Ok(self)
    }
    /// Add a future field; known fields must use typed configuration.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_steps::UpdateStep;
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let params = UpdateStep::new().extension_data("future_setting", true)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] if the key is a modeled step field. Unknown keys are only
    /// an escape hatch for future server fields; acceptance is checked by the API.
    pub fn extension_data(
        mut self,
        field: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Self> {
        let field = field.into();
        if STEP_FIELDS.contains(&field.as_str()) {
            return Err(Error::Config(
                "use typed configuration for known step fields".into(),
            ));
        }
        self.data.insert(field, value.into());
        Ok(self)
    }
    /// Recalculate the wait for subscribers already queued at a delay step.
    pub fn reschedule_waiting_subscribers(mut self, value: bool) -> Self {
        self.reschedule_waiting_subscribers = Some(value);
        self
    }
}
/// Branch handling when deleting condition or A/B steps.
#[derive(Debug, Clone, Copy)]
pub enum DeleteBranch {
    /// Delete all branches.
    All,
    /// Delete the yes branch of a condition.
    Yes,
    /// Delete the no branch of a condition.
    No,
    /// Keep A/B path A and reattach it in the deleted step's place.
    APath,
    /// Keep A/B path B and reattach it in the deleted step's place.
    BPath,
    /// Keep A/B path C and reattach it in the deleted step's place.
    CPath,
}
impl DeleteBranch {
    fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Yes => "yes",
            Self::No => "no",
            Self::APath => "a_path",
            Self::BPath => "b_path",
            Self::CPath => "c_path",
        }
    }
}
/// Parameters for deleting a step.
#[derive(Debug, Clone, Default)]
pub struct DeleteStep {
    branch: Option<DeleteBranch>,
}
impl DeleteStep {
    /// Delete a step without branch selection.
    pub fn new() -> Self {
        Self::default()
    }
    /// Choose how child branches are handled.
    pub fn branch_to_delete(mut self, branch: DeleteBranch) -> Self {
        self.branch = Some(branch);
        self
    }
}
/// Parameters for copying a step.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CopyStep {
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<StepBranch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    move_to_branch: Option<ConditionBranch>,
}
impl CopyStep {
    /// Copy into the first workflow position by default.
    pub fn new() -> Self {
        Self::default()
    }
    /// Place the copy after this step.
    pub fn parent_id(mut self, id: impl Into<String>) -> Self {
        self.parent_id = Some(id.into());
        self
    }
    /// Select the parent's condition or A/B branch.
    pub fn branch(mut self, branch: StepBranch) -> Self {
        self.branch = Some(branch);
        self
    }
    /// Select which condition branch receives an existing child.
    pub fn move_to_branch(mut self, branch: ConditionBranch) -> Self {
        self.move_to_branch = Some(branch);
        self
    }
}
/// Parameters for moving a step and its descendants.
#[derive(Debug, Clone, Serialize)]
pub struct ReorderSteps {
    dragged_step_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_parent_step_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<StepBranch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_for_child: Option<ConditionBranch>,
}
impl ReorderSteps {
    /// Select the step to move; omission of a parent moves it to the first position.
    pub fn new(step: impl Into<String>) -> Self {
        Self {
            dragged_step_id: step.into(),
            new_parent_step_id: None,
            branch: None,
            branch_for_child: None,
        }
    }
    /// Move below this new parent.
    pub fn new_parent_step_id(mut self, id: impl Into<String>) -> Self {
        self.new_parent_step_id = Some(id.into());
        self
    }
    /// Select a branch of the new parent.
    pub fn branch(mut self, branch: StepBranch) -> Self {
        self.branch = Some(branch);
        self
    }
    /// Select where an existing child is moved when moving a condition step.
    pub fn branch_for_child(mut self, branch: ConditionBranch) -> Self {
        self.branch_for_child = Some(branch);
        self
    }
}
/// Which step execution rows to list.
#[derive(Debug, Clone, Copy)]
pub enum StepSubscriberStatus {
    /// Subscribers waiting at the step.
    Queued,
    /// Subscribers who completed the step.
    Completed,
}
/// Parameters for one numbered page of step subscriber executions.
#[derive(Debug, Clone)]
pub struct StepSubscribers {
    query: Vec<(String, String)>,
}
impl StepSubscribers {
    /// Select the required execution status.
    pub fn new(status: StepSubscriberStatus) -> Self {
        Self {
            query: vec![(
                "status".into(),
                match status {
                    StepSubscriberStatus::Queued => "queued",
                    StepSubscriberStatus::Completed => "completed",
                }
                .into(),
            )],
        }
    }
    fn set(mut self, key: &str, value: String) -> Self {
        self.query.retain(|(k, _)| k != key);
        self.query.push((key.into(), value));
        self
    }
    /// Filter by exact subscriber email.
    pub fn search(self, value: impl Into<String>) -> Self {
        self.set("filter[search]", value.into())
    }
    /// Set the page number, starting at one.
    pub fn page(self, value: u64) -> Self {
        self.set("page", value.to_string())
    }
    /// Set the page size, from 1 through 1000.
    pub fn limit(self, value: u64) -> Self {
        self.set("limit", value.to_string())
    }
    /// Sort by `date`, `-date`, `email` or `-email`.
    pub fn sort(self, value: impl Into<String>) -> Self {
        self.set("sort", value.into())
    }
    /// Include dates on or after this `YYYY-MM-DD` date.
    pub fn date_from(self, value: impl Into<String>) -> Self {
        self.set("filter[date_from]", value.into())
    }
    /// Include dates on or before this `YYYY-MM-DD` date.
    pub fn date_to(self, value: impl Into<String>) -> Self {
        self.set("filter[date_to]", value.into())
    }
    /// Filter by execution-row identifiers, sent as a comma-separated list.
    pub fn ids(self, ids: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.set(
            "filter[id]",
            ids.into_iter()
                .map(|id| id.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        )
    }
    fn validate(&self) -> Result<()> {
        for (key, value) in &self.query {
            if key == "page" && value == "0" {
                return Err(Error::Config("page must be at least 1".into()));
            }
            if key == "limit" && !matches!(value.parse::<u64>(), Ok(1..=1000)) {
                return Err(Error::Config("limit must be between 1 and 1000".into()));
            }
            if key == "sort" && !matches!(value.as_str(), "date" | "-date" | "email" | "-email") {
                return Err(Error::Config(
                    "sort must be date, -date, email or -email".into(),
                ));
            }
        }
        Ok(())
    }
}
/// Resource handle for automation workflow steps.
#[derive(Debug, Clone, Copy)]
pub struct AutomationSteps<'a> {
    client: &'a MailerLite,
}
impl<'a> AutomationSteps<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List every step in workflow order.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().list("123").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn list(&self, automation: impl AsRef<str>) -> Result<Response<Vec<AutomationStep>>> {
        self.client
            .data(
                Method::GET,
                &["automations", automation.as_ref(), "steps", "index"],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Fetch one step with its type-specific configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().get("123", "456").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn get(
        &self,
        automation: impl AsRef<str>,
        step: impl AsRef<str>,
    ) -> Result<Response<AutomationStep>> {
        self.client
            .data(
                Method::GET,
                &["automations", automation.as_ref(), "steps", step.as_ref()],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Create a step in an inactive automation.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_steps::CreateStep;
    /// use mailerlite_rs::models::automation_configuration::{
    ///     DelayConfiguration, DelayUnit, StepConfiguration,
    /// };
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().create(
    ///     "123", CreateStep::configured(StepConfiguration::Delay(DelayConfiguration::new(DelayUnit::Days, 1)))?,
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. Request serialization can return [`crate::Error::Serialize`]. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn create(
        &self,
        automation: impl AsRef<str>,
        params: CreateStep,
    ) -> Result<Response<AutomationStep>> {
        self.client
            .data(
                Method::POST,
                &["automations", automation.as_ref(), "steps"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Replace step configuration. Send the complete data for the step's type.
    ///
    /// The automation must be inactive. Omitted settings in the replacement configuration can be
    /// cleared by the server. This write is never automatically retried. A timeout or transport
    /// failure after dispatch does not establish whether the server applied it; inspect the result
    /// before repeating the operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_steps::UpdateStep;
    /// use mailerlite_rs::models::automation_configuration::{
    ///     DelayConfiguration, DelayUnit, StepConfiguration,
    /// };
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().update(
    ///     "123", "456",
    ///     UpdateStep::new().configuration(StepConfiguration::Delay(
    ///         DelayConfiguration::new(DelayUnit::Days, 1),
    ///     ))?,
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. Request serialization can return [`crate::Error::Serialize`]. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn update(
        &self,
        automation: impl AsRef<str>,
        step: impl AsRef<str>,
        params: UpdateStep,
    ) -> Result<Response<AutomationStep>> {
        self.client
            .data(
                Method::PUT,
                &["automations", automation.as_ref(), "steps", step.as_ref()],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Delete a step, with explicit branch handling for condition and A/B steps.
    ///
    /// The automation must be inactive. Branch selection can remove child workflow branches; choose
    /// it explicitly for condition or A/B steps. This write is never automatically retried. A
    /// timeout or transport failure after dispatch does not establish whether the server applied
    /// it; inspect the result before repeating the operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_steps::DeleteStep;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().delete("123", "456", DeleteStep::new()).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn delete(
        &self,
        automation: impl AsRef<str>,
        step: impl AsRef<str>,
        params: DeleteStep,
    ) -> Result<Response<()>> {
        let query = params
            .branch
            .map(|branch| vec![("branch_to_delete".into(), branch.as_str().into())])
            .unwrap_or_default();
        self.client
            .empty(
                Method::DELETE,
                &["automations", automation.as_ref(), "steps", step.as_ref()],
                &query,
                None,
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Copy a step and return the updated automation.
    ///
    /// The automation must be inactive; this adds a new workflow step. This write is never
    /// automatically retried. A timeout or transport failure after dispatch does not establish
    /// whether the server applied it; inspect the result before repeating the operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_steps::CopyStep;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().copy(
    ///     "123", "456", CopyStep::new().parent_id("789"),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. Request serialization can return [`crate::Error::Serialize`]. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn copy(
        &self,
        automation: impl AsRef<str>,
        step: impl AsRef<str>,
        params: CopyStep,
    ) -> Result<Response<Automation>> {
        self.client
            .data(
                Method::POST,
                &[
                    "automations",
                    automation.as_ref(),
                    "steps",
                    step.as_ref(),
                    "copy",
                ],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Move a step and return the updated automation.
    ///
    /// The automation must be inactive. Moving a step changes the workflow path; supply branch
    /// handling when relevant. This write is never automatically retried. A timeout or transport
    /// failure after dispatch does not establish whether the server applied it; inspect the result
    /// before repeating the operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_steps::ReorderSteps;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().reorder(
    ///     "123", ReorderSteps::new("456").new_parent_step_id("789"),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. Request serialization can return [`crate::Error::Serialize`]. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn reorder(
        &self,
        automation: impl AsRef<str>,
        params: ReorderSteps,
    ) -> Result<Response<Automation>> {
        self.client
            .data(
                Method::POST,
                &["automations", automation.as_ref(), "steps", "reorder"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Select an A/B winning path and return its statistics.
    ///
    /// This changes the selected workflow path. The server validates the step type and allowable
    /// state. This write is never automatically retried. A timeout or transport failure after
    /// dispatch does not establish whether the server applied it; inspect the result before
    /// repeating the operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_steps::AbWinner;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().select_ab_test_winner("123", "456", AbWinner::APath)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn select_ab_test_winner(
        &self,
        automation: impl AsRef<str>,
        step: impl AsRef<str>,
        winner: AbWinner,
    ) -> Result<Response<AbTestStats>> {
        #[derive(serde::Serialize)]
        struct Payload {
            ab_test_winner: AbWinner,
        }
        self.client
            .data(
                Method::PUT,
                &[
                    "automations",
                    automation.as_ref(),
                    "steps",
                    step.as_ref(),
                    "select-ab-test-winner",
                ],
                &[],
                Some(&Payload {
                    ab_test_winner: winner,
                }),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Fetch execution counts for every step.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().run_counts("123").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn run_counts(
        &self,
        automation: impl AsRef<str>,
    ) -> Result<Response<Vec<StepRunCounts>>> {
        self.client
            .data(
                Method::GET,
                &["automations", automation.as_ref(), "steps", "run-counts"],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Fetch execution counts for one step.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().run_count("123", "456").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn run_count(
        &self,
        automation: impl AsRef<str>,
        step: impl AsRef<str>,
    ) -> Result<Response<StepRunCounts>> {
        self.client
            .data(
                Method::GET,
                &[
                    "automations",
                    automation.as_ref(),
                    "steps",
                    "run-counts",
                    step.as_ref(),
                ],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Fetch statistics for every A/B step.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().ab_stats("123").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn ab_stats(
        &self,
        automation: impl AsRef<str>,
    ) -> Result<Response<Vec<AbTestStats>>> {
        self.client
            .data(
                Method::GET,
                &["automations", automation.as_ref(), "ab-steps-stats"],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Fetch statistics for one A/B step.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().ab_stat("123", "456").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn ab_stat(
        &self,
        automation: impl AsRef<str>,
        step: impl AsRef<str>,
    ) -> Result<Response<AbTestStats>> {
        self.client
            .data(
                Method::GET,
                &[
                    "automations",
                    automation.as_ref(),
                    "ab-steps-stats",
                    step.as_ref(),
                ],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Fetch one numbered page of step subscriber executions.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_steps::{StepSubscribers, StepSubscriberStatus};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_steps().subscribers(
    ///     "123", "456", StepSubscribers::new(StepSubscriberStatus::Queued).limit(25),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// A zero page, limit outside 1 through 1000, or sort other than `date`, `-date`, `email`, or
    /// `-email` returns [`crate::Error::Config`]. Empty path identifiers and dot path segments (`.` or
    /// `..`) return [`crate::Error::Config`] before dispatch. API rejection returns
    /// [`crate::Error::Api`], including server-side validation and account restrictions. HTTP
    /// execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn subscribers(
        &self,
        automation: impl AsRef<str>,
        step: impl AsRef<str>,
        params: StepSubscribers,
    ) -> Result<Response<NumberedPage<StepSubscriber>>> {
        params.validate()?;
        self.client
            .request(
                Method::GET,
                &[
                    "automations",
                    automation.as_ref(),
                    "activity",
                    "steps",
                    step.as_ref(),
                    "subscribers",
                ],
                &params.query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
}

impl std::fmt::Debug for CreateStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateStep")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
impl std::fmt::Debug for UpdateStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateStep").finish_non_exhaustive()
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn create_ab_step_omits_unset_configuration() {
        assert_eq!(
            serde_json::to_value(CreateStep::new(StepType::AbTest)).unwrap(),
            json!({"type":"abtest"})
        );
    }

    #[test]
    fn copy_omits_unset_position_and_branch() {
        assert_eq!(serde_json::to_value(CopyStep::new()).unwrap(), json!({}));
    }

    #[test]
    fn update_clear_parent_serializes_explicit_null() {
        assert_eq!(
            serde_json::to_value(UpdateStep::new().clear_parent()).unwrap(),
            json!({"parent_id":null})
        );
    }
}
