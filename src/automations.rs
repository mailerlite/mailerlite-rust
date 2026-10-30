//! Automations: <https://developers.mailerlite.com/api/automations>

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{
    MailerLite, Page, Result,
    campaigns::Email,
    subscribers::Subscriber,
    types::{Id, NameBody, Rate, Timestamp, de, string_enum},
};

string_enum! {
    /// Kind of automation step. Only the documented kinds are listed; others decode as
    /// `Other`.
    pub enum StepType {
        /// Waits before the next step.
        Delay => "delay",
        /// Sends an email.
        Email => "email",
        /// Branches on a condition.
        Condition => "condition",
    }
}

string_enum! {
    /// What starts an automation. Only the documented kind is listed; others decode as
    /// `Other`.
    pub enum TriggerType {
        /// A subscriber joins one of the trigger's groups.
        SubscriberJoinsGroup => "subscriber_joins_group",
    }
}

string_enum! {
    /// Subscriber state filter for [`Automations::activity`].
    pub enum AutomationActivityStatus {
        /// Went through the whole automation.
        Completed => "completed",
        /// Still in the automation.
        Active => "active",
        /// Removed from the automation.
        Canceled => "canceled",
        /// A step failed.
        Failed => "failed",
    }
}

/// An automation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
pub struct Automation {
    /// Automation id.
    pub id: Id,
    /// Automation name.
    #[serde(default)]
    pub name: String,
    /// Whether the automation is running.
    #[serde(default, deserialize_with = "de::null_default")]
    pub enabled: bool,
    /// Trigger settings.
    #[serde(default, deserialize_with = "de::object_or_default")]
    pub trigger_data: TriggerData,
    /// Workflow steps.
    #[serde(default, deserialize_with = "de::null_default")]
    pub steps: Vec<Step>,
    /// What starts the automation.
    #[serde(default, deserialize_with = "de::null_default")]
    pub triggers: Vec<Trigger>,
    /// Whether every step is fully configured.
    #[serde(default, deserialize_with = "de::null_default")]
    pub complete: bool,
    /// Whether a step or trigger is broken (e.g. refers to a deleted group).
    #[serde(default, deserialize_with = "de::null_default")]
    pub broken: bool,
    /// Warnings about the automation.
    #[serde(default, deserialize_with = "de::null_default")]
    pub warnings: Vec<Value>,
    /// Number of email steps.
    #[serde(default, deserialize_with = "de::null_default")]
    pub emails_count: u64,
    /// Screenshot of the first email.
    #[serde(default)]
    pub first_email_screenshot_url: Option<String>,
    /// Delivery statistics.
    #[serde(default, deserialize_with = "de::null_default")]
    pub stats: AutomationStats,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Whether the content was flagged as banned.
    #[serde(default, deserialize_with = "de::null_default")]
    pub has_banned_content: bool,
    /// Subscribers that currently qualify for the trigger.
    #[serde(default, deserialize_with = "de::null_default")]
    pub qualified_subscribers_count: u64,
}

/// Trigger settings of an [`Automation`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TriggerData {
    /// Whether e-commerce activity is tracked.
    #[serde(default, deserialize_with = "de::null_default")]
    pub track_ecommerce: bool,
    /// Whether a subscriber can go through the automation more than once.
    #[serde(default)]
    pub repeatable: Option<bool>,
    /// Whether the trigger configuration is valid.
    #[serde(default)]
    pub valid: Option<bool>,
}

/// What starts an [`Automation`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Trigger {
    /// Trigger id.
    pub id: Id,
    /// Trigger kind.
    #[serde(rename = "type")]
    pub trigger_type: TriggerType,
    /// Groups that start the automation.
    #[serde(default, deserialize_with = "de::null_default")]
    pub group_ids: Vec<Id>,
    /// Details of [`Trigger::group_ids`].
    #[serde(default, deserialize_with = "de::null_default")]
    pub groups: Vec<TriggerGroup>,
    /// Groups whose members are excluded.
    #[serde(default, deserialize_with = "de::null_default")]
    pub exclude_group_ids: Vec<Id>,
    /// Details of [`Trigger::exclude_group_ids`].
    #[serde(default, deserialize_with = "de::null_default")]
    pub excluded_groups: Vec<TriggerGroup>,
    /// Whether the trigger is broken.
    #[serde(default, deserialize_with = "de::null_default")]
    pub broken: bool,
}

/// A group referenced by a [`Trigger`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TriggerGroup {
    /// Group id.
    pub id: Id,
    /// Group name.
    #[serde(default)]
    pub name: String,
    /// Group URL.
    #[serde(default)]
    pub url: Option<String>,
}

/// One step of an automation workflow.
///
/// The fields every step has are typed; the rest depend on [`Step::step_type`] and are kept
/// in [`Step::details`]. Keys seen in the documented examples:
///
/// * `delay`: `unit` (e.g. `"minutes"`), `value` (e.g. `"5"`).
/// * `email`: `name`, `subject`, `from`, `from_name`, `email_id`, `email` (an [`Email`],
///   see [`Step::email`]), `language_id`, `track_opens`, `google_analytics`,
///   `tracking_was_disabled`.
/// * `condition`: `yes_step_id`, `no_step_id`, `matching_type` (`"any"` / `"all"`),
///   `conditions` (a list of `{type, field_id, action, action_value, field}` rules).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Step {
    /// Step id.
    pub id: Id,
    /// Step kind.
    #[serde(rename = "type")]
    pub step_type: StepType,
    /// The preceding step, `None` for the first step.
    #[serde(default)]
    pub parent_id: Option<Id>,
    /// Whether the step is fully configured.
    #[serde(default, deserialize_with = "de::null_default")]
    pub complete: bool,
    /// Whether the step is broken (not sent for every step kind).
    #[serde(default)]
    pub broken: Option<bool>,
    /// Readable description, e.g. `"Wait for 5 minutes"`.
    #[serde(default)]
    pub description: String,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
    /// Kind-specific fields, keyed as on the wire (see the type docs).
    #[serde(flatten)]
    pub details: Map<String, Value>,
}

impl Step {
    /// A kind-specific field as text, if present and a string (e.g. `"unit"` of a delay or
    /// `"subject"` of an email step).
    #[must_use]
    pub fn detail_str(&self, key: &str) -> Option<&str> {
        self.details.get(key).and_then(Value::as_str)
    }

    /// The email of an `email` step, if present and decodable.
    #[must_use]
    pub fn email(&self) -> Option<Email> {
        serde_json::from_value(self.details.get("email")?.clone()).ok()
    }
}

/// Delivery statistics of an [`Automation`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutomationStats {
    /// Subscribers who completed the automation.
    #[serde(default, deserialize_with = "de::null_default")]
    pub completed_subscribers_count: u64,
    /// Subscribers waiting in the automation.
    #[serde(default, deserialize_with = "de::null_default")]
    pub subscribers_in_queue_count: u64,
    /// Bounce rate.
    #[serde(default, deserialize_with = "de::null_default")]
    pub bounce_rate: Rate,
    /// Click-to-open rate.
    #[serde(default, deserialize_with = "de::null_default")]
    pub click_to_open_rate: Rate,
    /// Emails sent.
    #[serde(default, deserialize_with = "de::null_default")]
    pub sent: u64,
    /// Opens.
    #[serde(default, deserialize_with = "de::null_default")]
    pub opens_count: u64,
    /// Unique opens.
    #[serde(default, deserialize_with = "de::null_default")]
    pub unique_opens_count: u64,
    /// Open rate.
    #[serde(default, deserialize_with = "de::null_default")]
    pub open_rate: Rate,
    /// Clicks.
    #[serde(default, deserialize_with = "de::null_default")]
    pub clicks_count: u64,
    /// Unique clicks.
    #[serde(default, deserialize_with = "de::null_default")]
    pub unique_clicks_count: u64,
    /// Click rate.
    #[serde(default, deserialize_with = "de::null_default")]
    pub click_rate: Rate,
    /// Unsubscribes.
    #[serde(default, deserialize_with = "de::null_default")]
    pub unsubscribes_count: u64,
    /// Unsubscribe rate.
    #[serde(default, deserialize_with = "de::null_default")]
    pub unsubscribe_rate: Rate,
    /// Spam complaints.
    #[serde(default, deserialize_with = "de::null_default")]
    pub spam_count: u64,
    /// Spam complaint rate.
    #[serde(default, deserialize_with = "de::null_default")]
    pub spam_rate: Rate,
    /// Hard bounces.
    #[serde(default, deserialize_with = "de::null_default")]
    pub hard_bounces_count: u64,
    /// Hard bounce rate.
    #[serde(default, deserialize_with = "de::null_default")]
    pub hard_bounce_rate: Rate,
    /// Soft bounces.
    #[serde(default, deserialize_with = "de::null_default")]
    pub soft_bounces_count: u64,
    /// Soft bounce rate.
    #[serde(default, deserialize_with = "de::null_default")]
    pub soft_bounce_rate: Rate,
    /// Forward rate (sent on newly created automations).
    #[serde(default)]
    pub forward_rate: Option<Rate>,
    /// Social interactions (sent on newly created automations).
    #[serde(default, deserialize_with = "de::null_default")]
    pub social_interactions_count: u64,
    /// Social interaction rate (sent on newly created automations).
    #[serde(default)]
    pub social_interaction_rate: Option<Rate>,
}

/// Aggregates returned with [`Automations::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutomationAggregations {
    /// Total number of automations in the account.
    #[serde(default, deserialize_with = "de::null_default")]
    pub sum: u64,
}

/// Query for [`Automations::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListAutomations {
    /// `true` for active automations, `false` for inactive ones. Defaults to all.
    #[serde(rename = "filter[enabled]", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Name filter.
    #[serde(rename = "filter[name]", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Only automations whose trigger uses this group.
    #[serde(rename = "filter[group]", skip_serializing_if = "Option::is_none")]
    pub group: Option<Id>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Page size (API default 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl ListAutomations {
    /// Default query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Only active (`true`) or inactive (`false`) automations.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Only automations matching `name`.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Only automations triggered by `group_id`.
    #[must_use]
    pub fn group(mut self, group_id: impl Into<Id>) -> Self {
        self.group = Some(group_id.into());
        self
    }

    /// 1-based page number.
    #[must_use]
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Page size.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query for [`Automations::activity`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListAutomationActivity {
    /// Subscriber state (required).
    #[serde(rename = "filter[status]")]
    pub status: AutomationActivityStatus,
    /// From date, `YYYY-MM-DD` (completed, canceled and failed).
    #[serde(rename = "filter[date_from]", skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// To date, `YYYY-MM-DD` (completed, canceled and failed).
    #[serde(rename = "filter[date_to]", skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Scheduled from, `YYYY-MM-DD` (active).
    #[serde(
        rename = "filter[scheduled_from]",
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_from: Option<String>,
    /// Scheduled to, `YYYY-MM-DD` (active).
    #[serde(
        rename = "filter[scheduled_to]",
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_to: Option<String>,
    /// Only the subscriber with this email.
    #[serde(rename = "filter[search]", skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Page size (API default 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl ListAutomationActivity {
    /// Subscribers in `status`.
    #[must_use]
    pub fn new(status: AutomationActivityStatus) -> Self {
        Self {
            status,
            date_from: None,
            date_to: None,
            scheduled_from: None,
            scheduled_to: None,
            search: None,
            page: None,
            limit: None,
        }
    }

    /// From date (`YYYY-MM-DD`); for completed, canceled and failed.
    #[must_use]
    pub fn date_from(mut self, date: impl Into<String>) -> Self {
        self.date_from = Some(date.into());
        self
    }

    /// To date (`YYYY-MM-DD`); for completed, canceled and failed.
    #[must_use]
    pub fn date_to(mut self, date: impl Into<String>) -> Self {
        self.date_to = Some(date.into());
        self
    }

    /// Scheduled from (`YYYY-MM-DD`); for active.
    #[must_use]
    pub fn scheduled_from(mut self, date: impl Into<String>) -> Self {
        self.scheduled_from = Some(date.into());
        self
    }

    /// Scheduled to (`YYYY-MM-DD`); for active.
    #[must_use]
    pub fn scheduled_to(mut self, date: impl Into<String>) -> Self {
        self.scheduled_to = Some(date.into());
        self
    }

    /// Only the subscriber with `email`.
    #[must_use]
    pub fn search(mut self, email: impl Into<String>) -> Self {
        self.search = Some(email.into());
        self
    }

    /// 1-based page number.
    #[must_use]
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Page size.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// A subscriber's run through an automation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutomationActivity {
    /// Run id.
    pub id: Id,
    /// Readable status, e.g. `"Completed"`.
    #[serde(default)]
    pub status: String,
    /// When the run reached its status.
    #[serde(default)]
    pub date: Option<Timestamp>,
    /// Reason code for canceled or failed runs.
    #[serde(default)]
    pub reason: Option<String>,
    /// Readable reason.
    #[serde(default)]
    pub reason_description: String,
    /// Steps the subscriber went through.
    #[serde(default, rename = "stepRuns", deserialize_with = "de::null_default")]
    pub step_runs: Vec<StepRun>,
    /// The next step.
    #[serde(default, rename = "nextStep")]
    pub next_step: Option<Step>,
    /// The current step.
    #[serde(default, rename = "currentStep")]
    pub current_step: Option<Step>,
    /// The subscriber, when the API includes it.
    #[serde(default)]
    pub subscriber: Option<Subscriber>,
}

/// One executed step of an [`AutomationActivity`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StepRun {
    /// Run id.
    pub id: Id,
    /// The step's id.
    pub step_id: Id,
    /// Readable step description.
    #[serde(default)]
    pub description: String,
    /// When the step is or was scheduled to run.
    #[serde(default)]
    pub scheduled_for: Option<Timestamp>,
}

string_enum! {
    /// Reporting period for [`Automations::ecommerce_orders`].
    pub enum TimeFrame {
        /// The last 7 days.
        Week => "week",
        /// The last month.
        Month => "month",
        /// The last two months.
        TwoMonths => "two_months",
        /// The last three months.
        ThreeMonths => "three_months",
        /// The last six months.
        SixMonths => "six_months",
        /// The last year.
        Year => "year",
        /// The previous calendar year.
        LastYear => "last_year",
        /// Since the automation started.
        AllTime => "all_time",
        /// Your own dates; use [`ListAutomationOrders::custom`].
        Custom => "custom",
    }
}

/// Query for [`Automations::ecommerce_stats`] and [`Automations::email_ecommerce_stats`].
///
/// Without a period the whole history of the automation is reported.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct AutomationEcommerceStatsQuery {
    /// First day, `YYYY-MM-DD`. Set together with `end_date` via
    /// [`period`](Self::period).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Last day (inclusive), `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Only orders from this shop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_id: Option<Id>,
}

impl AutomationEcommerceStatsQuery {
    /// The whole history, all shops.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Reports on `start_date..=end_date` (`YYYY-MM-DD`). The API requires both or neither.
    #[must_use]
    pub fn period(mut self, start_date: impl Into<String>, end_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self.end_date = Some(end_date.into());
        self
    }

    /// Only orders from `shop_id`.
    #[must_use]
    pub fn shop(mut self, shop_id: impl Into<Id>) -> Self {
        self.shop_id = Some(shop_id.into());
        self
    }
}

/// Query for [`Automations::ecommerce_orders`]. The time frame is required.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListAutomationOrders {
    /// Reporting period.
    pub time_frame: TimeFrame,
    /// First day (`YYYY-MM-DD`); required with [`TimeFrame::Custom`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_frame_start: Option<String>,
    /// Last day (`YYYY-MM-DD`); required with [`TimeFrame::Custom`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_frame_end: Option<String>,
    /// Only orders from this shop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_shop_id: Option<Id>,
    /// Page size (API default 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl ListAutomationOrders {
    /// Orders in a preset `time_frame`. For your own dates use [`custom`](Self::custom).
    #[must_use]
    pub fn new(time_frame: TimeFrame) -> Self {
        Self {
            time_frame,
            time_frame_start: None,
            time_frame_end: None,
            ecommerce_shop_id: None,
            limit: None,
            page: None,
        }
    }

    /// Orders from `start` to `end` (`YYYY-MM-DD`).
    #[must_use]
    pub fn custom(start: impl Into<String>, end: impl Into<String>) -> Self {
        Self {
            time_frame_start: Some(start.into()),
            time_frame_end: Some(end.into()),
            ..Self::new(TimeFrame::Custom)
        }
    }

    /// Only orders from `shop_id`.
    #[must_use]
    pub fn shop(mut self, shop_id: impl Into<Id>) -> Self {
        self.ecommerce_shop_id = Some(shop_id.into());
        self
    }

    /// Page size.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// 1-based page number.
    #[must_use]
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

/// Automation endpoints. Obtained with [`MailerLite::automations`].
#[derive(Clone, Copy, Debug)]
pub struct Automations<'a> {
    client: &'a MailerLite,
}

impl MailerLite {
    /// Automation endpoints.
    #[must_use]
    pub fn automations(&self) -> Automations<'_> {
        Automations { client: self }
    }
}

impl<'a> Automations<'a> {
    /// `GET /automations`: one page of automations.
    pub async fn list(
        &self,
        query: &ListAutomations,
    ) -> Result<Page<Automation, AutomationAggregations>> {
        self.client
            .call(Method::GET, &["automations"])
            .query(query)
            .send()
            .await
    }

    /// Every automation matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        &self,
        query: ListAutomations,
    ) -> impl futures_util::Stream<Item = Result<Automation>> + 'a {
        let automations = *self;
        crate::pagination::stream::paged(query, move |query| async move {
            automations.list(&query).await
        })
    }

    /// `GET /automations/{id}`: one automation.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Automation> {
        self.client
            .call(Method::GET, &["automations", id.as_ref()])
            .data()
            .await
    }

    /// `POST /automations`: creates an empty, disabled draft automation.
    pub async fn create(&self, name: &str) -> Result<Automation> {
        self.client
            .call(Method::POST, &["automations"])
            .json(&NameBody { name })
            .data()
            .await
    }

    /// `DELETE /automations/{id}`: deletes an automation.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<()> {
        self.client
            .call(Method::DELETE, &["automations", id.as_ref()])
            .send_empty()
            .await
    }

    /// `GET /automations/{id}/activity`: one page of subscriber runs through the automation.
    pub async fn activity(
        &self,
        id: impl AsRef<str>,
        query: &ListAutomationActivity,
    ) -> Result<Page<AutomationActivity>> {
        self.client
            .call(Method::GET, &["automations", id.as_ref(), "activity"])
            .query(query)
            .send()
            .await
    }

    /// `GET /automations/{id}/monthly-totals/ecommerce`: e-commerce totals of the automation.
    ///
    /// **Not in the public API docs**; kept from SDK v1, so the response is untyped and the
    /// endpoint may change without notice. The query follows the API's own validation rules.
    pub async fn ecommerce_stats(
        &self,
        id: impl AsRef<str>,
        query: &AutomationEcommerceStatsQuery,
    ) -> Result<Value> {
        self.client
            .call(
                Method::GET,
                &["automations", id.as_ref(), "monthly-totals", "ecommerce"],
            )
            .query(query)
            .send()
            .await
    }

    /// `GET /automations/{id}/emails/{email_id}/ecommerce-overview-stats`: e-commerce stats of
    /// one automation email.
    ///
    /// **Not in the public API docs**; kept from SDK v1, so the response is untyped and the
    /// endpoint may change without notice. Takes the same query as
    /// [`ecommerce_stats`](Self::ecommerce_stats).
    pub async fn email_ecommerce_stats(
        &self,
        id: impl AsRef<str>,
        email_id: impl AsRef<str>,
        query: &AutomationEcommerceStatsQuery,
    ) -> Result<Value> {
        self.client
            .call(
                Method::GET,
                &[
                    "automations",
                    id.as_ref(),
                    "emails",
                    email_id.as_ref(),
                    "ecommerce-overview-stats",
                ],
            )
            .query(query)
            .send()
            .await
    }

    /// `GET /automations/{id}/order-activity`: orders attributed to the automation.
    ///
    /// **Not in the public API docs**; kept from SDK v1, so the response is untyped and the
    /// endpoint may change without notice. A time frame is required (see
    /// [`ListAutomationOrders`]).
    ///
    /// ```no_run
    /// # async fn run(client: mailerlite_rs::MailerLite) -> mailerlite_rs::Result<()> {
    /// use mailerlite_rs::automations::{ListAutomationOrders, TimeFrame};
    ///
    /// let automations = client.automations();
    /// let last_month = automations
    ///     .ecommerce_orders("123", &ListAutomationOrders::new(TimeFrame::Month).limit(25))
    ///     .await?;
    /// let q1 = automations
    ///     .ecommerce_orders("123", &ListAutomationOrders::custom("2026-01-01", "2026-03-31"))
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub async fn ecommerce_orders(
        &self,
        id: impl AsRef<str>,
        query: &ListAutomationOrders,
    ) -> Result<Value> {
        self.client
            .call(Method::GET, &["automations", id.as_ref(), "order-activity"])
            .query(query)
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::{Value, json};

    use super::*;
    use crate::http::mock::{client, fixture};

    #[tokio::test]
    async fn list_with_filters_and_top_level_aggregations() {
        let (client, mock) = client();
        mock.respond(200, fixture("automations_list"));

        let page = client
            .automations()
            .list(
                &ListAutomations::new()
                    .enabled(false)
                    .name("Welcome")
                    .group("42")
                    .page(2)
                    .limit(5),
            )
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "automations?filter%5Benabled%5D=false&filter%5Bname%5D=Welcome&filter%5Bgroup%5D=42&page=2&limit=5",
        );
        assert_eq!(page.aggregations.unwrap().sum, 24);
        let automation = &page.data[0];
        assert!(!automation.enabled);
        assert_eq!(automation.trigger_data.valid, Some(true));
        assert_eq!(
            automation.triggers[0].trigger_type,
            TriggerType::SubscriberJoinsGroup
        );
        assert_eq!(automation.triggers[0].groups[0].name, "My Group");
        assert_eq!(automation.stats.unique_opens_count, 0);
        assert_eq!(automation.qualified_subscribers_count, 63);

        let delay = &automation.steps[0];
        assert_eq!(delay.step_type, StepType::Delay);
        assert_eq!(delay.parent_id, None);
        assert_eq!(delay.detail_str("unit"), Some("minutes"));
        assert_eq!(delay.detail_str("value"), Some("5"));
        assert!(!delay.details.contains_key("id"));

        let email_step = &automation.steps[1];
        assert_eq!(email_step.step_type, StepType::Email);
        assert_eq!(email_step.description, "Send \"mail {$name}\" email");
        let email = email_step.email().unwrap();
        assert_eq!(email.subject.as_deref(), Some("mail {$name}"));
        assert_eq!(email.language_id.as_deref(), Some("4"));
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn list_all_walks_pages() {
        use futures_util::TryStreamExt;

        let (client, mock) = client();
        let automation = fixture("automations_get")["data"].clone();
        mock.respond(
            200,
            json!({"data": [automation], "meta": {"current_page": 1, "last_page": 2}}),
        )
        .respond(
            200,
            json!({"data": [automation], "meta": {"current_page": 2, "last_page": 2}}),
        );

        let all: Vec<Automation> = client
            .automations()
            .list_all(ListAutomations::new().enabled(true))
            .try_collect()
            .await
            .unwrap();

        assert_eq!(all.len(), 2);
        let requests = mock.requests();
        requests[0].assert(Method::GET, "automations?filter%5Benabled%5D=true&page=1");
        requests[1].assert(Method::GET, "automations?filter%5Benabled%5D=true&page=2");
    }

    #[tokio::test]
    async fn get() {
        let (client, mock) = client();
        mock.respond(200, fixture("automations_get"));

        let automation = client.automations().get("57444257882965766").await.unwrap();

        mock.request()
            .assert(Method::GET, "automations/57444257882965766")
            .no_body();
        assert_eq!(automation.name, "Automation 1");
        assert_eq!(automation.trigger_data.repeatable, Some(false));
        assert_eq!(automation.steps.len(), 2);
        assert_eq!(automation.emails_count, 1);
    }

    #[tokio::test]
    async fn create_and_delete() {
        let (client, mock) = client();
        mock.respond(201, fixture("automations_create"))
            .respond(204, Value::Null);

        let created = client
            .automations()
            .create("Created_automation")
            .await
            .unwrap();
        client
            .automations()
            .delete("100000000000000000")
            .await
            .unwrap();

        let requests = mock.requests();
        requests[0]
            .assert(Method::POST, "automations")
            .body(&json!({"name": "Created_automation"}));
        requests[1]
            .assert(Method::DELETE, "automations/100000000000000000")
            .no_body();
        assert_eq!(created.id, "100000000000000000");
        assert!(created.steps.is_empty());
        assert!(created.triggers.is_empty());
        assert_eq!(created.trigger_data.repeatable, None);
        assert_eq!(created.stats.sent, 0);
        assert_eq!(created.stats.forward_rate.unwrap().string, "0%");
    }

    #[tokio::test]
    async fn activity() {
        let (client, mock) = client();
        mock.respond(200, fixture("automations_activity"));

        let page = client
            .automations()
            .activity(
                "1",
                &ListAutomationActivity::new(AutomationActivityStatus::Completed)
                    .date_from("2022-08-01")
                    .date_to("2022-08-31")
                    .scheduled_from("2022-09-01")
                    .scheduled_to("2022-09-30")
                    .search("ann@example.com")
                    .page(2)
                    .limit(25),
            )
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "automations/1/activity?filter%5Bstatus%5D=completed&filter%5Bdate_from%5D=2022-08-01&filter%5Bdate_to%5D=2022-08-31&filter%5Bscheduled_from%5D=2022-09-01&filter%5Bscheduled_to%5D=2022-09-30&filter%5Bsearch%5D=ann%40example.com&page=2&limit=25",
        );
        let run = &page.data[0];
        assert_eq!(run.status, "Completed");
        assert_eq!(run.reason, None);
        assert_eq!(run.step_runs.len(), 2);
        assert_eq!(run.step_runs[1].scheduled_for, None);
        let next = run.next_step.as_ref().unwrap();
        assert_eq!(next.step_type, StepType::Condition);
        assert_eq!(next.broken, Some(false));
        assert_eq!(next.detail_str("matching_type"), Some("any"));
        assert_eq!(next.details["conditions"][0]["field"]["name"], "City");
        let current = run.current_step.as_ref().unwrap();
        assert_eq!(current.email().unwrap().stats.sent, 3);
        assert_eq!(page.meta.total, 3);
    }

    #[tokio::test]
    async fn undocumented_ecommerce_endpoints() {
        let (client, mock) = client();
        mock.respond(200, json!({"data": []}))
            .respond(200, json!({"data": {}}))
            .respond(200, json!({"data": []}))
            .respond(200, json!({"data": []}));

        let automations = client.automations();
        automations
            .ecommerce_stats("1", &AutomationEcommerceStatsQuery::new().shop("9"))
            .await
            .unwrap();
        automations
            .email_ecommerce_stats(
                "1",
                "2",
                &AutomationEcommerceStatsQuery::new().period("2026-01-01", "2026-01-31"),
            )
            .await
            .unwrap();
        let orders = automations
            .ecommerce_orders("1", &ListAutomationOrders::new(TimeFrame::Month).page(2))
            .await
            .unwrap();
        automations
            .ecommerce_orders(
                "1",
                &ListAutomationOrders::custom("2026-01-01", "2026-03-31").shop("9"),
            )
            .await
            .unwrap();

        let requests = mock.requests();
        requests[0].assert(
            Method::GET,
            "automations/1/monthly-totals/ecommerce?shop_id=9",
        );
        requests[1].assert(
            Method::GET,
            "automations/1/emails/2/ecommerce-overview-stats?start_date=2026-01-01&end_date=2026-01-31",
        );
        requests[2].assert(
            Method::GET,
            "automations/1/order-activity?time_frame=month&page=2",
        );
        requests[3].assert(
            Method::GET,
            "automations/1/order-activity?time_frame=custom&time_frame_start=2026-01-01&time_frame_end=2026-03-31&ecommerce_shop_id=9",
        );
        assert_eq!(orders, json!({"data": []}));
    }
}
