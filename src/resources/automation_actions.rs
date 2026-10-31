//! Automation actions and reports backed by MailerLite route, request, and resource schemas.
//!
//! These methods preserve v1 capabilities through the v2 transport. The additional
//! contracts were checked against the MailerLite backend source. Availability of
//! these additional routes depends on the deployment.
use super::automations::{ActivityStatus, Automations};
use crate::{
    Error, Response, Result,
    http::OperationPolicy,
    models::{
        Patch,
        automation_reports::{
            AutomationEcommerceReport, AutomationEmailContent, AutomationEmailEcommerceReport,
            AutomationEmailReport, AutomationReportOrder, AutomationUnsubscribeReason,
        },
        automations::Automation,
        campaigns::CampaignEmail,
    },
    pagination::NumberedPage,
};
use reqwest::Method;
use serde::Serialize;

/// Automation name, shared trigger settings, and optional trigger replacement.
///
/// Omitted settings and trigger definitions are not submitted. Explicit nulls
/// are preserved, but are not synonymous with clearing: the server's trigger
/// settings logic falls back to existing values and may normalize repeatability
/// for particular trigger types. Changing settings or triggers requires an
/// inactive automation.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateAutomation {
    /// Automation name.
    name: String,
    #[serde(skip_serializing_if = "Patch::is_unset")]
    trigger_data: Patch<AutomationTriggerSettings>,
    #[serde(skip_serializing_if = "Patch::is_unset")]
    triggers: Patch<Vec<AutomationTriggerDefinition>>,
}
impl UpdateAutomation {
    /// Construct parameters with name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            trigger_data: Patch::Unset,
            triggers: Patch::Unset,
        }
    }
    /// Submit shared trigger settings, or explicitly submit null with `Patch::Null`.
    pub fn trigger_data(mut self, settings: impl Into<Patch<AutomationTriggerSettings>>) -> Self {
        self.trigger_data = settings.into();
        self
    }
    /// Submit a complete trigger list, or an explicit null with `Patch::Null`.
    ///
    /// Existing triggers omitted from a submitted replacement list are removed
    /// by the server. Omit this field when only renaming or changing settings.
    pub fn triggers(
        mut self,
        triggers: impl Into<Patch<Vec<AutomationTriggerDefinition>>>,
    ) -> Self {
        self.triggers = triggers.into();
        self
    }
}

/// Shared trigger settings accepted by the whole-automation update endpoint.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AutomationTriggerSettings {
    #[serde(skip_serializing_if = "Patch::is_unset")]
    repeatable: Patch<bool>,
    #[serde(skip_serializing_if = "Patch::is_unset")]
    track_ecommerce: Patch<bool>,
    #[serde(skip_serializing_if = "Patch::is_unset")]
    show_analytics: Patch<bool>,
    #[serde(skip_serializing_if = "Patch::is_unset")]
    repeatable_settings: Patch<AutomationRepeatableSettings>,
}
impl AutomationTriggerSettings {
    /// Start with all settings omitted.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set repeatability, preserving explicit null separately from omission.
    pub fn repeatable(mut self, value: impl Into<Patch<bool>>) -> Self {
        self.repeatable = value.into();
        self
    }
    /// Set e-commerce tracking, preserving explicit null separately from omission.
    pub fn track_ecommerce(mut self, value: impl Into<Patch<bool>>) -> Self {
        self.track_ecommerce = value.into();
        self
    }
    /// Set analytics visibility, preserving explicit null separately from omission.
    pub fn show_analytics(mut self, value: impl Into<Patch<bool>>) -> Self {
        self.show_analytics = value.into();
        self
    }
    /// Set the re-entry timing policy; the server may normalize it for the trigger type.
    pub fn repeatable_settings(
        mut self,
        value: impl Into<Patch<AutomationRepeatableSettings>>,
    ) -> Self {
        self.repeatable_settings = value.into();
        self
    }
}

/// Unit of the delay before subscribers may re-enter an automation.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationRepeatableUnit {
    /// Hours.
    Hour,
    /// Days.
    Day,
    /// Weeks.
    Week,
    /// Months.
    Month,
}

/// Timing policy for repeated automation enrollment.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum AutomationRepeatableSettings {
    /// Permit immediate re-entry.
    Immediately,
    /// Require a positive waiting interval before re-entry.
    Wait {
        /// Positive number of units.
        value: std::num::NonZeroU64,
        /// Time unit.
        unit: AutomationRepeatableUnit,
    },
}
impl AutomationRepeatableSettings {
    /// Construct a positive waiting interval, rejecting zero locally.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_actions::{AutomationRepeatableSettings, AutomationRepeatableUnit};
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let params = AutomationRepeatableSettings::wait(2, AutomationRepeatableUnit::Day)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] when `value` is zero. No account or trigger policy is
    /// checked until the parameters are submitted.
    pub fn wait(value: u64, unit: AutomationRepeatableUnit) -> Result<Self> {
        let value = std::num::NonZeroU64::new(value)
            .ok_or_else(|| Error::Config("repeatable waiting interval must be positive".into()))?;
        Ok(Self::Wait { value, unit })
    }
}

/// A trigger definition in a whole-automation replacement list.
///
/// This endpoint expects flattened type-specific fields, unlike the nested
/// `data` payload on trigger-specific create/update operations. The server
/// validates subtype-specific requirements, referenced objects, and plan limits.
#[derive(Clone, Serialize)]
pub struct AutomationTriggerDefinition {
    #[serde(skip_serializing_if = "Patch::is_unset")]
    id: Patch<String>,
    #[serde(rename = "type")]
    kind: String,
    #[serde(skip_serializing_if = "Patch::is_unset")]
    group_ids: Patch<Vec<String>>,
    #[serde(skip_serializing_if = "Patch::is_unset")]
    exclude_group_ids: Patch<Vec<String>>,
    #[serde(flatten)]
    settings: serde_json::Map<String, serde_json::Value>,
}
impl AutomationTriggerDefinition {
    /// Set the trigger type, for example `subscriber_joins_group`.
    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            id: Patch::Unset,
            kind: kind.into(),
            group_ids: Patch::Unset,
            exclude_group_ids: Patch::Unset,
            settings: serde_json::Map::new(),
        }
    }
    /// Identify an existing trigger to retain and update in the replacement list.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Patch::Value(id.into());
        self
    }
    /// Submit an explicit null identifier to request a new trigger.
    pub fn null_id(mut self) -> Self {
        self.id = Patch::Null;
        self
    }
    /// Set included groups for a group trigger.
    pub fn group_ids(mut self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.settings.remove("group_ids");
        self.group_ids = Patch::Value(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set excluded groups for a group trigger, including explicit null if needed.
    pub fn exclude_group_ids(mut self, ids: impl Into<Patch<Vec<String>>>) -> Self {
        self.settings.remove("exclude_group_ids");
        self.exclude_group_ids = ids.into();
        self
    }
    /// Set typed subtype fields, preserving the flattened replacement-list format.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_actions::AutomationTriggerDefinition;
    /// use mailerlite_rs::models::automation_configuration::{
    ///     GroupTriggerConfiguration, TriggerConfiguration,
    /// };
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let configuration = TriggerConfiguration::Group(
    ///     GroupTriggerConfiguration::new(["789"]),
    /// );
    /// let params = AutomationTriggerDefinition::new("subscriber_joins_group").configuration(configuration)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Serialize`] if serialization fails or [`crate::Error::Config`] if
    /// configuration does not produce an object. A discriminator mismatch also returns
    /// `Error::Config`.
    pub fn configuration(
        mut self,
        configuration: crate::models::automation_configuration::TriggerConfiguration,
    ) -> Result<Self> {
        if self.kind != configuration.kind() {
            return Err(Error::Config(
                "trigger type does not match its configuration".into(),
            ));
        }
        self.group_ids = Patch::Unset;
        self.exclude_group_ids = Patch::Unset;
        self.settings =
            crate::models::automation_configuration::configuration_fields(&configuration)?;
        Ok(self)
    }
    /// Set an additional polymorphic trigger field using its exact JSON key.
    ///
    /// Reserved envelope fields must use their typed setters. All remaining
    /// subtype-specific validation is performed by the server.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_actions::AutomationTriggerDefinition;
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let params = AutomationTriggerDefinition::new("subscriber_joins_group")
    ///     .extension_setting("future_setting", true)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for a modeled or reserved trigger key. Use typed setters
    /// for known settings. Unknown keys are intended only for future server fields and remain
    /// subject to server validation.
    pub fn extension_setting(
        mut self,
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Result<Self> {
        let key = key.into();
        if crate::models::automation_configuration::TRIGGER_FIELDS.contains(&key.as_str()) {
            return Err(Error::Config(
                "use typed trigger setters and flattened subtype fields".into(),
            ));
        }
        self.settings.insert(key, value.into());
        Ok(self)
    }
}

/// An automation test recipient.
#[derive(Debug, Clone, Serialize)]
pub struct TestAutomation {
    /// Verified recipient email address.
    email: String,
}
impl TestAutomation {
    /// Construct parameters with email.
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }
}

/// Replacement HTML for an automation email.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateAutomationEmailContent {
    /// Complete email HTML.
    html: String,
}
impl UpdateAutomationEmailContent {
    /// Construct parameters with html.
    pub fn new(html: impl Into<String>) -> Self {
        Self { html: html.into() }
    }
}

/// Subscribers to enqueue in an active automation.
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddAutomationSubscribers {
    /// Subscriber email addresses.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    emails: Vec<String>,
    /// Source group identifiers.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    groups: Vec<String>,
    /// Optional entry step.
    #[serde(skip_serializing_if = "Option::is_none")]
    step_id: Option<String>,
}
impl AddAutomationSubscribers {
    /// Construct parameters.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set optional entry step.
    pub fn step_id(mut self, value: impl Into<String>) -> Self {
        self.step_id = Some(value.into());
        self
    }
    /// Append a email.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.emails.push(value.into());
        self
    }
    /// Append a group.
    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.groups.push(value.into());
        self
    }
}

/// Select automation runs for an activity action.
#[derive(Debug, Clone, Serialize)]
pub struct AutomationActivityFilter {
    /// Run status.
    status: ActivityStatus,
    /// Run identifiers.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    id: Vec<String>,
    /// search.
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    /// date from.
    #[serde(skip_serializing_if = "Option::is_none")]
    date_from: Option<String>,
    /// date to.
    #[serde(skip_serializing_if = "Option::is_none")]
    date_to: Option<String>,
    /// scheduled from.
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_from: Option<String>,
    /// scheduled to.
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_to: Option<String>,
    /// next step.
    #[serde(skip_serializing_if = "Option::is_none")]
    next_step: Option<String>,
    /// step.
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<String>,
}
impl AutomationActivityFilter {
    /// Construct parameters with status.
    pub fn new(status: ActivityStatus) -> Self {
        Self {
            status,
            id: Vec::new(),
            search: None,
            date_from: None,
            date_to: None,
            scheduled_from: None,
            scheduled_to: None,
            next_step: None,
            step: None,
        }
    }
    /// Set search.
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }
    /// Set date from.
    pub fn date_from(mut self, value: impl Into<String>) -> Self {
        self.date_from = Some(value.into());
        self
    }
    /// Set date to.
    pub fn date_to(mut self, value: impl Into<String>) -> Self {
        self.date_to = Some(value.into());
        self
    }
    /// Set scheduled from.
    pub fn scheduled_from(mut self, value: impl Into<String>) -> Self {
        self.scheduled_from = Some(value.into());
        self
    }
    /// Set scheduled to.
    pub fn scheduled_to(mut self, value: impl Into<String>) -> Self {
        self.scheduled_to = Some(value.into());
        self
    }
    /// Set next step.
    pub fn next_step(mut self, value: impl Into<String>) -> Self {
        self.next_step = Some(value.into());
        self
    }
    /// Set step.
    pub fn step(mut self, value: impl Into<String>) -> Self {
        self.step = Some(value.into());
        self
    }
    /// Append a run identifier.
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id.push(value.into());
        self
    }
}

/// Date range and shop selection for automation reports.
#[derive(Debug, Clone, Serialize, Default)]
pub struct AutomationReportQuery {
    /// start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    /// end date.
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<String>,
    /// shop id.
    #[serde(skip_serializing_if = "Option::is_none")]
    shop_id: Option<String>,
}
impl AutomationReportQuery {
    /// Construct parameters.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set start date.
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }
    /// Set end date.
    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }
    /// Set shop id.
    pub fn shop_id(mut self, value: impl Into<String>) -> Self {
        self.shop_id = Some(value.into());
        self
    }
    fn into_query(self) -> Result<Vec<(String, String)>> {
        if self.start_date.is_some() != self.end_date.is_some() {
            return Err(Error::Config(
                "start_date and end_date must be supplied together".into(),
            ));
        }
        let mut query = Vec::new();
        if let Some(value) = self.start_date {
            query.push(("start_date".into(), value.to_string()));
        }
        if let Some(value) = self.end_date {
            query.push(("end_date".into(), value.to_string()));
        }
        if let Some(value) = self.shop_id {
            query.push(("shop_id".into(), value.to_string()));
        }
        Ok(query)
    }
}

/// Reporting period accepted by the automation order endpoint.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationOrderTimeFrame {
    /// Current week.
    Week,
    /// Current month.
    Month,
    /// Two months.
    TwoMonths,
    /// Three months.
    ThreeMonths,
    /// Six months.
    SixMonths,
    /// Current year.
    Year,
    /// Previous year.
    LastYear,
    /// All available history.
    AllTime,
    /// Explicit start and end dates.
    Custom,
}
impl AutomationOrderTimeFrame {
    fn as_str(self) -> &'static str {
        match self {
            Self::Week => "week",
            Self::Month => "month",
            Self::TwoMonths => "two_months",
            Self::ThreeMonths => "three_months",
            Self::SixMonths => "six_months",
            Self::Year => "year",
            Self::LastYear => "last_year",
            Self::AllTime => "all_time",
            Self::Custom => "custom",
        }
    }
}

/// Page and time-frame selection for attributed orders.
#[derive(Debug, Clone, Serialize)]
pub struct AutomationOrderQuery {
    /// Time frame such as all_time, month, or custom.
    time_frame: AutomationOrderTimeFrame,
    /// Custom period start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    time_frame_start: Option<String>,
    /// Custom period end date.
    #[serde(skip_serializing_if = "Option::is_none")]
    time_frame_end: Option<String>,
    /// Connected shop identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    ecommerce_shop_id: Option<String>,
    /// Number of orders per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    /// One-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u64>,
}
impl AutomationOrderQuery {
    /// Construct parameters with time frame.
    pub fn new(time_frame: AutomationOrderTimeFrame) -> Self {
        Self {
            time_frame,
            time_frame_start: None,
            time_frame_end: None,
            ecommerce_shop_id: None,
            limit: None,
            page: None,
        }
    }
    /// Set custom period start date.
    pub fn time_frame_start(mut self, value: impl Into<String>) -> Self {
        self.time_frame_start = Some(value.into());
        self
    }
    /// Set custom period end date.
    pub fn time_frame_end(mut self, value: impl Into<String>) -> Self {
        self.time_frame_end = Some(value.into());
        self
    }
    /// Set connected shop identifier.
    pub fn shop_id(mut self, value: impl Into<String>) -> Self {
        self.ecommerce_shop_id = Some(value.into());
        self
    }
    /// Set number of orders per page.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
    /// Set one-based page number.
    pub fn page(mut self, value: u64) -> Self {
        self.page = Some(value);
        self
    }
    fn into_query(self) -> Result<Vec<(String, String)>> {
        if self.page == Some(0) || self.limit == Some(0) {
            return Err(Error::Config("page and limit must be positive".into()));
        }
        if matches!(self.time_frame, AutomationOrderTimeFrame::Custom)
            && (self.time_frame_start.is_none() || self.time_frame_end.is_none())
        {
            return Err(Error::Config(
                "custom time frame requires start and end dates".into(),
            ));
        }
        let mut query = Vec::new();
        query.push(("time_frame".into(), self.time_frame.as_str().into()));
        if let Some(value) = self.time_frame_start {
            query.push(("time_frame_start".into(), value.to_string()));
        }
        if let Some(value) = self.time_frame_end {
            query.push(("time_frame_end".into(), value.to_string()));
        }
        if let Some(value) = self.ecommerce_shop_id {
            query.push(("ecommerce_shop_id".into(), value.to_string()));
        }
        if let Some(value) = self.limit {
            query.push(("limit".into(), value.to_string()));
        }
        if let Some(value) = self.page {
            query.push(("page".into(), value.to_string()));
        }
        Ok(query)
    }
}

/// Where subscribers resume when re-enrolled.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReaddDestination {
    /// Continue after the last completed step.
    LastCompletedStep,
    /// Retry the last failed step.
    LastFailedStep,
    /// Enter at the separately supplied selected step identifier.
    SelectedStep,
}

/// Re-enrollment destination and selection of completed, canceled, or failed runs.
#[derive(Debug, Clone, Serialize)]
pub struct ReaddAutomationSubscribers {
    readd_subscribers_to: ReaddDestination,
    filter: AutomationActivityFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_step_id: Option<String>,
}
impl ReaddAutomationSubscribers {
    /// Choose last_completed_step, last_failed_step, or selected_step and a run filter.
    pub fn new(destination: ReaddDestination, filter: AutomationActivityFilter) -> Self {
        Self {
            readd_subscribers_to: destination,
            filter,
            selected_step_id: None,
        }
    }
    /// Choose the required step when the destination is selected_step.
    pub fn selected_step_id(mut self, value: impl Into<String>) -> Self {
        self.selected_step_id = Some(value.into());
        self
    }
}
/// Destination groups and automation run selection.
#[derive(Debug, Clone, Serialize)]
pub struct AddAutomationSubscribersToGroup {
    destination_groups: Vec<String>,
    filter: AutomationActivityFilter,
}
impl AddAutomationSubscribersToGroup {
    /// Select the first destination group and runs.
    pub fn new(group: impl Into<String>, filter: AutomationActivityFilter) -> Self {
        Self {
            destination_groups: vec![group.into()],
            filter,
        }
    }
    /// Append another destination group.
    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.destination_groups.push(value.into());
        self
    }
}
#[derive(Serialize)]
struct ActivitySelection {
    filter: AutomationActivityFilter,
}

impl Automations<'_> {
    /// Update an automation's name, shared trigger settings, or replacement triggers.
    ///
    /// Supplied trigger lists replace existing triggers; omitted fields are not submitted. Changes
    /// to settings or triggers require an inactive automation. This write is never automatically
    /// retried. A timeout or transport failure after dispatch does not establish whether the server
    /// applied it; inspect the result before repeating the operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::UpdateAutomation;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().update("123", UpdateAutomation::new("Welcome"))
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
    /// dispatch. Request serialization can return [`crate::Error::Serialize`]. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: UpdateAutomation,
    ) -> Result<Response<Automation>> {
        self.client
            .data(
                Method::PUT,
                &["automations", id.as_ref()],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Copy an automation.
    ///
    /// The copy is inactive and its workflow is built in the background. Poll
    /// the returned automation until `building_in_progress` clears before editing or enabling it.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().copy("123").await?;
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
    pub async fn copy(&self, id: impl AsRef<str>) -> Result<Response<Automation>> {
        self.client
            .data(
                Method::POST,
                &["automations", id.as_ref(), "copy"],
                &[],
                None,
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Enable an automation; this can send email.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().enable("123").await?;
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
    pub async fn enable(&self, id: impl AsRef<str>) -> Result<Response<Automation>> {
        self.client
            .data(
                Method::POST,
                &["automations", id.as_ref(), "enable"],
                &[],
                None,
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Disable an automation.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().disable("123").await?;
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
    pub async fn disable(&self, id: impl AsRef<str>) -> Result<Response<Automation>> {
        self.client
            .data(
                Method::POST,
                &["automations", id.as_ref(), "disable"],
                &[],
                None,
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Enqueue every sendable automation email for a verified recipient.
    ///
    /// Requires an active account and at least one email with content. Delays and
    /// conditions are not simulated. The server limits this operation to 10
    /// requests per hour. A successful empty response confirms job acceptance,
    /// not completed delivery; the SDK never retries this operation.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::TestAutomation;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().test("123", TestAutomation::new("jane@example.com"))
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
    /// dispatch. Request serialization can return [`crate::Error::Serialize`]. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn test(&self, id: impl AsRef<str>, params: TestAutomation) -> Result<Response<()>> {
        self.client
            .empty(
                Method::POST,
                &["automations", id.as_ref(), "test"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Enroll subscribers; this can send email.
    /// Completion means the background job was accepted, not that every subscriber was processed.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::AddAutomationSubscribers;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().add_subscribers(
    ///     "123", AddAutomationSubscribers::new().email("jane@example.com"),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// An empty email and group selection returns [`crate::Error::Config`]. Empty identifiers and
    /// dot path segments (`.` or `..`) return [`crate::Error::Config`] before dispatch. Request
    /// serialization can return [`crate::Error::Serialize`]. API rejection returns
    /// [`crate::Error::Api`], including server-side validation and account restrictions. HTTP
    /// execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn add_subscribers(
        &self,
        id: impl AsRef<str>,
        params: AddAutomationSubscribers,
    ) -> Result<Response<()>> {
        if params.emails.is_empty() && params.groups.is_empty() {
            return Err(Error::Config(
                "at least one subscriber email or group is required".into(),
            ));
        }
        self.client
            .empty(
                Method::POST,
                &["automations", id.as_ref(), "add-subscribers"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Re-enroll selected subscribers.
    /// Completion means the background job was accepted, not that every subscriber was processed.
    ///
    /// Re-enrollment can execute workflow actions and send email. This write is never automatically
    /// retried. A timeout or transport failure after dispatch does not establish whether the server
    /// applied it; inspect the result before repeating the operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::{
    ///     AutomationActivityFilter, ReaddAutomationSubscribers, ReaddDestination,
    /// };
    /// use mailerlite_rs::resources::automations::ActivityStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().readd_subscribers(
    ///     "123",
    ///     ReaddAutomationSubscribers::new(
    ///         ReaddDestination::LastCompletedStep,
    ///         AutomationActivityFilter::new(ActivityStatus::Completed).id("456"),
    ///     ),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Selecting active runs, or a selected-step destination without a step ID, returns
    /// [`crate::Error::Config`]. Empty path identifiers and dot path segments (`.` or `..`) return
    /// [`crate::Error::Config`] before dispatch. Request serialization can return
    /// [`crate::Error::Serialize`]. API rejection returns [`crate::Error::Api`], including server-
    /// side validation and account restrictions. HTTP execution can also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn readd_subscribers(
        &self,
        id: impl AsRef<str>,
        params: ReaddAutomationSubscribers,
    ) -> Result<Response<()>> {
        if matches!(params.filter.status, ActivityStatus::Active) {
            return Err(Error::Config(
                "re-enrollment requires completed, canceled, or failed runs".into(),
            ));
        }
        if matches!(params.readd_subscribers_to, ReaddDestination::SelectedStep)
            && params.selected_step_id.is_none()
        {
            return Err(Error::Config(
                "selected_step requires selected_step_id".into(),
            ));
        }
        self.client
            .empty(
                Method::POST,
                &["automations", id.as_ref(), "readd"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Assign selected automation subscribers to groups.
    /// Completion means the background job was accepted, not that every subscriber was processed.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::{AddAutomationSubscribersToGroup, AutomationActivityFilter};
    /// use mailerlite_rs::resources::automations::ActivityStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().add_subscribers_to_group(
    ///     "123",
    ///     AddAutomationSubscribersToGroup::new(
    ///         "789", AutomationActivityFilter::new(ActivityStatus::Completed).id("456"),
    ///     ),
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
    pub async fn add_subscribers_to_group(
        &self,
        id: impl AsRef<str>,
        params: AddAutomationSubscribersToGroup,
    ) -> Result<Response<()>> {
        self.client
            .empty(
                Method::POST,
                &["automations", id.as_ref(), "activity", "add-to-group"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Cancel selected subscriber executions.
    /// Completion means the background job was accepted, not that every subscriber was processed.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::AutomationActivityFilter;
    /// use mailerlite_rs::resources::automations::ActivityStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().cancel_subscribers(
    ///     "123", AutomationActivityFilter::new(ActivityStatus::Active).id("456"),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Selecting a status other than active returns [`crate::Error::Config`]. Empty identifiers and
    /// dot path segments (`.` or `..`) return [`crate::Error::Config`] before dispatch. Request
    /// serialization can return [`crate::Error::Serialize`]. API rejection returns
    /// [`crate::Error::Api`], including server-side validation and account restrictions. HTTP
    /// execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn cancel_subscribers(
        &self,
        id: impl AsRef<str>,
        params: AutomationActivityFilter,
    ) -> Result<Response<()>> {
        if !matches!(params.status, ActivityStatus::Active) {
            return Err(Error::Config(
                "only active automation runs can be selected".into(),
            ));
        }
        let params = ActivitySelection { filter: params };
        self.client
            .empty(
                Method::POST,
                &["automations", id.as_ref(), "activity", "cancel-automation"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Skip delays for selected executions.
    /// Completion means the background job was accepted, not that every subscriber was processed.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::AutomationActivityFilter;
    /// use mailerlite_rs::resources::automations::ActivityStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().skip_delay(
    ///     "123", AutomationActivityFilter::new(ActivityStatus::Active).id("456"),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Selecting a status other than active returns [`crate::Error::Config`]. Empty identifiers and
    /// dot path segments (`.` or `..`) return [`crate::Error::Config`] before dispatch. Request
    /// serialization can return [`crate::Error::Serialize`]. API rejection returns
    /// [`crate::Error::Api`], including server-side validation and account restrictions. HTTP
    /// execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn skip_delay(
        &self,
        id: impl AsRef<str>,
        params: AutomationActivityFilter,
    ) -> Result<Response<()>> {
        if !matches!(params.status, ActivityStatus::Active) {
            return Err(Error::Config(
                "only active automation runs can be selected".into(),
            ));
        }
        let params = ActivitySelection { filter: params };
        self.client
            .empty(
                Method::POST,
                &["automations", id.as_ref(), "activity", "skip-delay"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Read email content.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().email_content("123", "456").await?;
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
    pub async fn email_content(
        &self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
    ) -> Result<Response<AutomationEmailContent>> {
        self.client
            .data(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "emails",
                    email_id.as_ref(),
                    "content",
                ],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Update email content.
    ///
    /// The supplied HTML replaces the email content; it is not a partial patch. This write is never
    /// automatically retried. A timeout or transport failure after dispatch does not establish
    /// whether the server applied it; inspect the result before repeating the operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::UpdateAutomationEmailContent;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().update_email_content(
    ///     "123", "456", UpdateAutomationEmailContent::new("<p>Hello Jane</p>"),
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
    pub async fn update_email_content(
        &self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
        params: UpdateAutomationEmailContent,
    ) -> Result<Response<CampaignEmail>> {
        self.client
            .data(
                Method::PUT,
                &[
                    "automations",
                    id.as_ref(),
                    "emails",
                    email_id.as_ref(),
                    "content",
                ],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Read email report statistics.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::AutomationReportQuery;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().email_report_stats(
    ///     "123", "456", AutomationReportQuery::new(),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Supplying only one of `start_date` and `end_date` returns [`crate::Error::Config`]; date
    /// syntax and account access are validated by the server. Empty path identifiers and dot path
    /// segments (`.` or `..`) return [`crate::Error::Config`] before dispatch. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn email_report_stats(
        &self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
        params: AutomationReportQuery,
    ) -> Result<Response<AutomationEmailReport>> {
        let query = params.into_query()?;
        self.client
            .data(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "emails",
                    email_id.as_ref(),
                    "report-stats",
                ],
                &query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Read email unsubscribe reasons.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().email_unsubscribe_reasons("123", "456").await?;
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
    pub async fn email_unsubscribe_reasons(
        &self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
    ) -> Result<Response<Vec<AutomationUnsubscribeReason>>> {
        self.client
            .data(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "emails",
                    email_id.as_ref(),
                    "unsubscribe-reasons",
                ],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Read automation email monthly totals.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::AutomationReportQuery;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().email_stats(
    ///     "123", AutomationReportQuery::new(),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Supplying only one of `start_date` and `end_date` returns [`crate::Error::Config`]; date
    /// syntax and account access are validated by the server. Empty path identifiers and dot path
    /// segments (`.` or `..`) return [`crate::Error::Config`] before dispatch. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn email_stats(
        &self,
        id: impl AsRef<str>,
        params: AutomationReportQuery,
    ) -> Result<Response<AutomationEmailReport>> {
        let query = params.into_query()?;
        self.client
            .data(
                Method::GET,
                &["automations", id.as_ref(), "monthly-totals", "email"],
                &query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Read automation e-commerce monthly totals.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::AutomationReportQuery;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().ecommerce_stats(
    ///     "123", AutomationReportQuery::new(),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Supplying only one of `start_date` and `end_date` returns [`crate::Error::Config`]; date
    /// syntax and account access are validated by the server. Empty path identifiers and dot path
    /// segments (`.` or `..`) return [`crate::Error::Config`] before dispatch. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn ecommerce_stats(
        &self,
        id: impl AsRef<str>,
        params: AutomationReportQuery,
    ) -> Result<Response<AutomationEcommerceReport>> {
        let query = params.into_query()?;
        self.client
            .data(
                Method::GET,
                &["automations", id.as_ref(), "monthly-totals", "ecommerce"],
                &query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Read email e-commerce overview statistics.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::AutomationReportQuery;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().email_ecommerce_stats(
    ///     "123", "456", AutomationReportQuery::new(),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Supplying only one of `start_date` and `end_date` returns [`crate::Error::Config`]; date
    /// syntax and account access are validated by the server. Empty path identifiers and dot path
    /// segments (`.` or `..`) return [`crate::Error::Config`] before dispatch. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn email_ecommerce_stats(
        &self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
        params: AutomationReportQuery,
    ) -> Result<Response<AutomationEmailEcommerceReport>> {
        let query = params.into_query()?;
        self.client
            .data(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "emails",
                    email_id.as_ref(),
                    "ecommerce-overview-stats",
                ],
                &query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Read a page of automation order activity.
    ///
    /// Use the returned numbered-page metadata for subsequent explicit requests.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_actions::{AutomationOrderQuery, AutomationOrderTimeFrame};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().ecommerce_orders(
    ///     "123", AutomationOrderQuery::new(AutomationOrderTimeFrame::AllTime).limit(25),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// A zero page or limit, or a custom time frame without both dates, returns
    /// [`crate::Error::Config`]. Empty path identifiers and dot path segments (`.` or `..`) return
    /// [`crate::Error::Config`] before dispatch. API rejection returns [`crate::Error::Api`],
    /// including server-side validation and account restrictions. HTTP execution can also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn ecommerce_orders(
        &self,
        id: impl AsRef<str>,
        params: AutomationOrderQuery,
    ) -> Result<Response<NumberedPage<AutomationReportOrder>>> {
        let query = params.into_query()?;
        self.client
            .request(
                Method::GET,
                &["automations", id.as_ref(), "order-activity"],
                &query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
}

impl std::fmt::Debug for AutomationTriggerDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutomationTriggerDefinition")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn update_omits_unset_trigger_configuration() {
        assert_eq!(
            serde_json::to_value(UpdateAutomation::new("Name")).unwrap(),
            json!({"name":"Name"})
        );
    }

    #[test]
    fn update_can_explicitly_clear_trigger_configuration() {
        assert_eq!(
            serde_json::to_value(
                UpdateAutomation::new("Name")
                    .trigger_data(Patch::Null)
                    .triggers(Patch::Null)
            )
            .unwrap(),
            json!({"name":"Name", "trigger_data":null, "triggers":null})
        );
    }

    #[test]
    fn update_keeps_empty_trigger_objects_and_arrays() {
        assert_eq!(
            serde_json::to_value(
                UpdateAutomation::new("Name")
                    .trigger_data(AutomationTriggerSettings::new())
                    .triggers(Vec::new())
            )
            .unwrap(),
            json!({"name":"Name", "trigger_data":{}, "triggers":[]})
        );
    }

    #[test]
    fn trigger_settings_preserve_false_and_explicit_nulls() {
        assert_eq!(
            serde_json::to_value(
                UpdateAutomation::new("Name").trigger_data(
                    AutomationTriggerSettings::new()
                        .repeatable(false)
                        .track_ecommerce(Patch::Null)
                        .repeatable_settings(Patch::Null)
                )
            )
            .unwrap(),
            json!({"name":"Name", "trigger_data":{"repeatable":false,"track_ecommerce":null,"repeatable_settings":null}})
        );
    }

    #[test]
    fn repeatable_wait_rejects_zero_duration() {
        assert!(AutomationRepeatableSettings::wait(0, AutomationRepeatableUnit::Day).is_err());
    }

    #[test]
    fn repeatable_immediately_omits_wait_settings() {
        assert_eq!(
            serde_json::to_value(AutomationRepeatableSettings::Immediately).unwrap(),
            json!({"mode":"immediately"})
        );
    }

    #[test]
    fn trigger_definition_can_explicitly_clear_its_identifier() {
        assert_eq!(
            serde_json::to_value(
                AutomationTriggerDefinition::new("subscriber_joins_group")
                    .null_id()
                    .group_ids(["004"])
            )
            .unwrap(),
            json!({"id":null,"type":"subscriber_joins_group","group_ids":["004"]})
        );
    }

    #[test]
    fn trigger_setting_rejects_reserved_id() {
        assert!(
            AutomationTriggerDefinition::new("subscriber_joins_group")
                .extension_setting("id", true)
                .is_err()
        );
    }

    #[test]
    fn trigger_setting_rejects_reserved_type() {
        assert!(
            AutomationTriggerDefinition::new("subscriber_joins_group")
                .extension_setting("type", true)
                .is_err()
        );
    }

    #[test]
    fn trigger_setting_rejects_reserved_data() {
        assert!(
            AutomationTriggerDefinition::new("subscriber_joins_group")
                .extension_setting("data", true)
                .is_err()
        );
    }

    #[test]
    fn trigger_setting_rejects_reserved_group_ids() {
        assert!(
            AutomationTriggerDefinition::new("subscriber_joins_group")
                .extension_setting("group_ids", true)
                .is_err()
        );
    }

    #[test]
    fn trigger_setting_rejects_reserved_exclude_group_ids() {
        assert!(
            AutomationTriggerDefinition::new("subscriber_joins_group")
                .extension_setting("exclude_group_ids", true)
                .is_err()
        );
    }
}
