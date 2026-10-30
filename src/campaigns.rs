//! Campaigns: <https://developers.mailerlite.com/api/campaigns> and
//! <https://developers.mailerlite.com/api/campaign-languages>

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    MailerLite, Page, Result,
    reports::{EcommerceSummary, ListReportOrders, ReportOrder, ShopQuery},
    subscribers::Subscriber,
    types::{Id, Rate, Sort, Timestamp, de, string_enum},
};

string_enum! {
    /// Campaign status.
    pub enum CampaignStatus {
        /// Being edited; not scheduled.
        Draft => "draft",
        /// Scheduled or queued for sending.
        Ready => "ready",
        /// Sent.
        Sent => "sent",
    }
}

string_enum! {
    /// Campaign type.
    pub enum CampaignType {
        /// A single email.
        Regular => "regular",
        /// A/B split test.
        Ab => "ab",
        /// Auto resend to subscribers who did not open or click.
        Resend => "resend",
        /// RSS campaign (cannot be created through the API).
        Rss => "rss",
        /// Multivariate test.
        Multivariate => "multivariate",
    }
}

string_enum! {
    /// How a campaign is delivered.
    pub enum Delivery {
        /// Send right away.
        Instant => "instant",
        /// Send at a fixed date and time.
        Scheduled => "scheduled",
        /// Send at the given local time in each subscriber's timezone.
        TimezoneBased => "timezone_based",
        /// Send when each subscriber is most likely to engage.
        SmartSending => "smart_sending",
    }
}

string_enum! {
    /// What an A/B test varies.
    pub enum AbTestType {
        /// The subject line.
        Subject => "subject",
        /// The sender name and address.
        Sender => "sender",
        /// The sending time.
        SendingTime => "sending_time",
    }
}

string_enum! {
    /// Metric for picking an A/B winner (or, for auto resend, which recipients get the
    /// second email).
    pub enum WinnerMetric {
        /// Opens (auto resend: subscribers who did not open).
        Opens => "o",
        /// Clicks (auto resend: subscribers who did not click).
        Clicks => "c",
    }
}

string_enum! {
    /// Unit of an A/B test wait time.
    pub enum TimeUnit {
        /// Hours.
        Hours => "h",
        /// Days.
        Days => "d",
    }
}

string_enum! {
    /// What an auto resend varies. The API only supports the subject.
    pub enum ResendTestType {
        /// The subject line.
        Subject => "subject",
    }
}

string_enum! {
    /// When an auto resend goes out.
    pub enum ResendDelivery {
        /// A day after the original campaign.
        Day => "day",
        /// At a fixed date and time.
        Scheduled => "scheduled",
    }
}

string_enum! {
    /// Recipient activity filter for [`Campaigns::subscriber_activity`].
    pub enum CampaignActivityType {
        /// Opened the campaign.
        Opened => "opened",
        /// Did not open the campaign.
        Unopened => "unopened",
        /// Clicked a link.
        Clicked => "clicked",
        /// Unsubscribed.
        Unsubscribed => "unsubscribed",
        /// Forwarded the campaign.
        Forwarded => "forwarded",
        /// Hard bounced.
        Hardbounced => "hardbounced",
        /// Soft bounced.
        Softbounced => "softbounced",
        /// Marked the campaign as junk.
        Junk => "junk",
    }
}

string_enum! {
    /// Sort field for [`ListCampaignActivity`].
    pub enum CampaignActivitySort {
        /// Activity id (API default).
        Id => "id",
        /// Last update time.
        UpdatedAt => "updated_at",
        /// Number of clicks.
        ClicksCount => "clicks_count",
        /// Number of opens.
        OpensCount => "opens_count",
    }
}

string_enum! {
    /// Related resources to embed in [`Campaigns::subscriber_activity`] results.
    pub enum CampaignActivityInclude {
        /// The subscriber.
        Subscriber => "subscriber",
        /// The subscriber with their groups.
        SubscriberGroups => "subscriber.groups",
    }
}

string_enum! {
    /// Writing direction of a [`Language`].
    pub enum TextDirection {
        /// Left to right.
        Ltr => "ltr",
        /// Right to left.
        Rtl => "rtl",
    }
}

/// A campaign.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
pub struct Campaign {
    /// Campaign id.
    pub id: Id,
    /// Account id.
    #[serde(default)]
    pub account_id: Option<Id>,
    /// Campaign name.
    #[serde(default)]
    pub name: String,
    /// Campaign type.
    #[serde(rename = "type")]
    pub campaign_type: CampaignType,
    /// Campaign status.
    pub status: CampaignStatus,
    /// Settings that still need to be filled in before the campaign can be sent.
    #[serde(default, deserialize_with = "de::null_default")]
    pub missing_data: Vec<Value>,
    /// Tracking settings.
    #[serde(default, deserialize_with = "de::object_or_default")]
    pub settings: CampaignSettings,
    /// Recipient filter in MailerLite's internal format: a list of OR-ed groups of AND-ed
    /// `{"operator": ..., "args": [...]}` rules. Kept as raw JSON; see
    /// [`Campaign::filter_for_humans`] for a readable version.
    #[serde(default)]
    pub filter: Value,
    /// Readable recipient filter, e.g. `[["In any group: VIP"]]`.
    #[serde(default, deserialize_with = "de::null_default")]
    pub filter_for_humans: Vec<Vec<String>>,
    /// Delivery type.
    #[serde(default)]
    pub delivery_schedule: Option<Delivery>,
    /// Language of the unsubscribe template (see [`Campaigns::languages`]).
    #[serde(default)]
    pub language_id: Option<Id>,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
    /// When the campaign is or was scheduled to go out.
    #[serde(default)]
    pub scheduled_for: Option<Timestamp>,
    /// When the campaign was queued.
    #[serde(default)]
    pub queued_at: Option<Timestamp>,
    /// When sending started.
    #[serde(default)]
    pub started_at: Option<Timestamp>,
    /// When sending finished.
    #[serde(default)]
    pub finished_at: Option<Timestamp>,
    /// When sending was stopped.
    #[serde(default)]
    pub stopped_at: Option<Timestamp>,
    /// Id of the campaign's main email.
    #[serde(default)]
    pub default_email_id: Option<Id>,
    /// The campaign's emails (several for A/B, resend and multivariate campaigns).
    #[serde(default, deserialize_with = "de::null_default")]
    pub emails: Vec<Email>,
    /// Whether an automation uses the campaign.
    #[serde(default)]
    pub used_in_automations: bool,
    /// Readable type, e.g. `"Regular"`.
    #[serde(default)]
    pub type_for_humans: String,
    /// Aggregated statistics. Only included in list responses.
    #[serde(default)]
    pub stats: Option<EmailStats>,
    /// Whether sending was stopped.
    #[serde(default)]
    pub is_stopped: bool,
    /// Whether an A/B test has a winner (`None` for non-test campaigns).
    #[serde(default)]
    pub has_winner: Option<bool>,
    /// Readable winning version of a test.
    #[serde(default)]
    pub winner_version_for_human: Option<String>,
    /// Readable winning sending time of a sending time test.
    #[serde(default)]
    pub winner_sending_time_for_humans: Option<String>,
    /// When a test winner was picked manually.
    #[serde(default)]
    pub winner_selected_manually_at: Option<Timestamp>,
    /// Whether the campaign uses e-commerce blocks.
    #[serde(default)]
    pub uses_ecommerce: bool,
    /// Whether the campaign contains a survey.
    #[serde(default)]
    pub uses_survey: bool,
    /// Whether the campaign is complete enough to be scheduled.
    #[serde(default)]
    pub can_be_scheduled: bool,
    /// Warnings about the campaign's content or settings.
    #[serde(default, deserialize_with = "de::null_default")]
    pub warnings: Vec<Value>,
    /// Creation time of the original campaign this one was copied from.
    #[serde(default)]
    pub initial_created_at: Option<Timestamp>,
    /// Whether the campaign is being sent right now.
    #[serde(default)]
    pub is_currently_sending_out: bool,
}

/// Tracking settings of a [`Campaign`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
pub struct CampaignSettings {
    /// Whether opens are tracked.
    #[serde(default)]
    pub track_opens: bool,
    /// Whether Google Analytics tags are added to links.
    #[serde(default)]
    pub use_google_analytics: bool,
    /// Whether e-commerce link tracking is on.
    #[serde(default)]
    pub ecommerce_tracking: bool,
}

/// An email of a campaign or automation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
pub struct Email {
    /// Email id.
    pub id: Id,
    /// Account id.
    #[serde(default)]
    pub account_id: Option<Id>,
    /// Id of the owning campaign or automation.
    #[serde(default)]
    pub emailable_id: Option<Id>,
    /// Owner kind: `campaigns` or `automations`.
    #[serde(default)]
    pub emailable_type: Option<String>,
    /// Editor used to build the email, e.g. `builder`.
    #[serde(default, rename = "type")]
    pub email_type: Option<String>,
    /// Sender address.
    #[serde(default)]
    pub from: String,
    /// Sender name.
    #[serde(default)]
    pub from_name: String,
    /// Reply-to address.
    #[serde(default)]
    pub reply_to: Option<String>,
    /// Internal email name.
    #[serde(default)]
    pub name: Option<String>,
    /// Subject line.
    #[serde(default)]
    pub subject: String,
    /// Plain text version.
    #[serde(default)]
    pub plain_text: Option<String>,
    /// Screenshot URL.
    #[serde(default)]
    pub screenshot_url: Option<String>,
    /// Preview URL.
    #[serde(default)]
    pub preview_url: Option<String>,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
    /// Whether the content has been designed.
    #[serde(default)]
    pub is_designed: bool,
    /// Language id (a string or a number depending on the endpoint).
    #[serde(default)]
    pub language_id: Option<Id>,
    /// Whether this email won its test.
    #[serde(default)]
    pub is_winner: bool,
    /// Delivery statistics.
    #[serde(default, deserialize_with = "de::null_default")]
    pub stats: EmailStats,
    /// Delay before the email is sent (test variants). Its shape is undocumented, so it is
    /// kept as raw JSON.
    #[serde(default)]
    pub send_after: Option<Value>,
    /// Whether opens are tracked.
    #[serde(default)]
    pub track_opens: bool,
}

/// Delivery statistics of an [`Email`] or [`Campaign`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmailStats {
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
    /// Forwards.
    #[serde(default, deserialize_with = "de::null_default")]
    pub forwards_count: u64,
    /// Click-to-open rate. Only present on campaign-level statistics.
    #[serde(default)]
    pub click_to_open_rate: Option<Rate>,
}

/// Campaign counts per status returned with [`Campaigns::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CampaignAggregations {
    /// All campaigns.
    #[serde(default, deserialize_with = "de::null_default")]
    pub sum: u64,
    /// Draft campaigns.
    #[serde(default, deserialize_with = "de::null_default")]
    pub draft: u64,
    /// Ready campaigns.
    #[serde(default, deserialize_with = "de::null_default")]
    pub ready: u64,
    /// Sent campaigns.
    #[serde(default, deserialize_with = "de::null_default")]
    pub sent: u64,
}

/// Query for [`Campaigns::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListCampaigns {
    /// Only campaigns with this status. **The API defaults to `ready`**, so drafts and sent
    /// campaigns are only listed when asked for.
    #[serde(rename = "filter[status]", skip_serializing_if = "Option::is_none")]
    pub status: Option<CampaignStatus>,
    /// Only campaigns of this type (`regular`, `ab`, `resend` or `rss`). Defaults to all.
    #[serde(rename = "filter[type]", skip_serializing_if = "Option::is_none")]
    pub campaign_type: Option<CampaignType>,
    /// Page size (API default 25).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl ListCampaigns {
    /// Default query (ready campaigns of every type).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Only campaigns with `status`.
    #[must_use]
    pub fn status(mut self, status: CampaignStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Only campaigns of `campaign_type`.
    #[must_use]
    pub fn campaign_type(mut self, campaign_type: CampaignType) -> Self {
        self.campaign_type = Some(campaign_type);
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

/// An email in a [`CreateCampaign`] or [`UpdateCampaign`] body.
///
/// ```
/// use mailerlite_rs::campaigns::CampaignEmail;
///
/// let email = CampaignEmail::new("Hello", "Ann", "ann@example.com")
///     .reply_to("support@example.com")
///     .content("<p>Hi {$name}</p>");
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CampaignEmail {
    /// Subject line (at most 255 characters). Required except for multivariate campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Sender name (at most 255 characters). Required except for multivariate campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Sender address, verified in MailerLite. Required except for multivariate campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Reply-to address, verified in MailerLite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// HTML content (Advanced plan only). Without an unsubscribe link, account name,
    /// address and country, MailerLite appends its default footer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl CampaignEmail {
    /// An email with subject and sender.
    pub fn new(
        subject: impl Into<String>,
        from_name: impl Into<String>,
        from: impl Into<String>,
    ) -> Self {
        Self {
            subject: Some(subject.into()),
            from_name: Some(from_name.into()),
            from: Some(from.into()),
            reply_to: None,
            content: None,
        }
    }

    /// An email without subject or sender, for multivariate campaigns where those come from
    /// [`MultivariateSettings`] (typically one per content variation, with [`content`](Self::content)).
    #[must_use]
    pub fn variation() -> Self {
        Self::default()
    }

    /// Sets the subject line.
    #[must_use]
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Sets the sender name and address.
    #[must_use]
    pub fn sender(mut self, from_name: impl Into<String>, from: impl Into<String>) -> Self {
        self.from_name = Some(from_name.into());
        self.from = Some(from.into());
        self
    }

    /// Sets the reply-to address.
    #[must_use]
    pub fn reply_to(mut self, reply_to: impl Into<String>) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }

    /// Sets the HTML content.
    #[must_use]
    pub fn content(mut self, html: impl Into<String>) -> Self {
        self.content = Some(html.into());
        self
    }
}

/// How the winner of a test is picked: by `select_winner_by` after waiting
/// `after_time_amount` `after_time_unit`s.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct WinnerSelection {
    /// Winning metric.
    pub select_winner_by: WinnerMetric,
    /// How long to wait before picking the winner.
    pub after_time_amount: u32,
    /// Unit of [`WinnerSelection::after_time_amount`].
    pub after_time_unit: TimeUnit,
}

impl WinnerSelection {
    /// Picks the winner by `metric` after `amount` `unit`s.
    #[must_use]
    pub fn new(metric: WinnerMetric, amount: u32, unit: TimeUnit) -> Self {
        Self {
            select_winner_by: metric,
            after_time_amount: amount,
            after_time_unit: unit,
        }
    }
}

/// Values for the B version of an A/B test or for an auto resend.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct BValue {
    /// Subject line (subject tests and auto resend).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Sender name (sender tests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Sender address (sender tests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}

/// `ab_settings` of an A/B campaign. Build with [`AbSettings::subject`],
/// [`AbSettings::sender`] or [`AbSettings::sending_time`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct AbSettings {
    /// What the test varies.
    pub test_type: AbTestType,
    /// How the winner is picked (not used for sending time tests).
    #[serde(flatten)]
    pub winner: Option<WinnerSelection>,
    /// Percentage of recipients in the test: 5 to 25 for subject and sender tests, 100 for
    /// sending time tests.
    pub test_split: u32,
    /// The B version (not used for sending time tests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_value: Option<BValue>,
}

impl AbSettings {
    /// Tests the subject line against `b_subject`.
    pub fn subject(b_subject: impl Into<String>, winner: WinnerSelection, test_split: u32) -> Self {
        Self {
            test_type: AbTestType::Subject,
            winner: Some(winner),
            test_split,
            b_value: Some(BValue {
                subject: Some(b_subject.into()),
                ..BValue::default()
            }),
        }
    }

    /// Tests the sender against `b_from_name` / `b_from`.
    pub fn sender(
        b_from_name: impl Into<String>,
        b_from: impl Into<String>,
        winner: WinnerSelection,
        test_split: u32,
    ) -> Self {
        Self {
            test_type: AbTestType::Sender,
            winner: Some(winner),
            test_split,
            b_value: Some(BValue {
                from_name: Some(b_from_name.into()),
                from: Some(b_from.into()),
                subject: None,
            }),
        }
    }

    /// Tests two sending times (`test_split` 100). Set the times with
    /// [`ScheduleCampaign::sending_time_test`].
    #[must_use]
    pub fn sending_time() -> Self {
        Self {
            test_type: AbTestType::SendingTime,
            winner: None,
            test_split: 100,
            b_value: None,
        }
    }
}

/// `resend_settings` of an auto resend campaign.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ResendSettings {
    /// What the resend varies (always the subject).
    pub test_type: ResendTestType,
    /// Who gets the second email: [`WinnerMetric::Opens`] for subscribers who did not open,
    /// [`WinnerMetric::Clicks`] for those who did not click.
    pub select_winner_by: WinnerMetric,
    /// The resend's subject.
    pub b_value: BValue,
}

impl ResendSettings {
    /// Resends with `subject` to subscribers who did not open or click (`resend_to`).
    pub fn new(subject: impl Into<String>, resend_to: WinnerMetric) -> Self {
        Self {
            test_type: ResendTestType::Subject,
            select_winner_by: resend_to,
            b_value: BValue {
                subject: Some(subject.into()),
                ..BValue::default()
            },
        }
    }
}

/// A sender variant of a multivariate test.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct Sender {
    /// Sender address, verified in MailerLite.
    pub from: String,
    /// Sender name.
    pub from_name: String,
}

impl Sender {
    /// A sender.
    pub fn new(from_name: impl Into<String>, from: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            from_name: from_name.into(),
        }
    }
}

/// Variants of a multivariate test. All four are sent; the number of combinations
/// (subjects × senders × content × sending times) may not exceed 8.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct TestVariants {
    /// Subject variants (at least 1).
    pub subject: Vec<String>,
    /// Sender variants (at least 1).
    pub sender: Vec<Sender>,
    /// Number of content variations, 1 to 8. Pass one [`CampaignEmail`] per variation.
    pub content: u8,
    /// Number of sending times, 1 to 8.
    pub sending_time: u8,
}

/// `multivariate_settings` of a multivariate campaign.
///
/// ```
/// use mailerlite_rs::campaigns::{
///     MultivariateSettings, Sender, TimeUnit, WinnerMetric, WinnerSelection,
/// };
///
/// let settings = MultivariateSettings::new("Hello", Sender::new("Ann", "ann@example.com"), 20)
///     .subject("Hi there")
///     .content(2)
///     .winner(WinnerSelection::new(WinnerMetric::Clicks, 4, TimeUnit::Hours));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct MultivariateSettings {
    /// The variants.
    pub test_variants: TestVariants,
    /// How the winner is picked (not used when the sending time is tested).
    #[serde(flatten)]
    pub winner: Option<WinnerSelection>,
    /// Percentage of recipients in the test: 10 to 50, or 100 when the sending time is tested.
    pub test_split: u32,
}

impl MultivariateSettings {
    /// A test starting from one subject and one sender, with a single content version and
    /// sending time. Add variants with the setters.
    pub fn new(subject: impl Into<String>, sender: Sender, test_split: u32) -> Self {
        Self {
            test_variants: TestVariants {
                subject: vec![subject.into()],
                sender: vec![sender],
                content: 1,
                sending_time: 1,
            },
            winner: None,
            test_split,
        }
    }

    /// Adds a subject variant.
    #[must_use]
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.test_variants.subject.push(subject.into());
        self
    }

    /// Adds a sender variant.
    #[must_use]
    pub fn sender(mut self, sender: Sender) -> Self {
        self.test_variants.sender.push(sender);
        self
    }

    /// Sets the number of content variations (1 to 8).
    #[must_use]
    pub fn content(mut self, variations: u8) -> Self {
        self.test_variants.content = variations;
        self
    }

    /// Sets the number of sending times (1 to 8).
    #[must_use]
    pub fn sending_time(mut self, times: u8) -> Self {
        self.test_variants.sending_time = times;
        self
    }

    /// Sets how the winner is picked.
    #[must_use]
    pub fn winner(mut self, winner: WinnerSelection) -> Self {
        self.winner = Some(winner);
        self
    }
}

/// `settings` of a [`CreateCampaign`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct NewCampaignSettings {
    /// Enables e-commerce link tracking when the content links to a connected shop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_tracking: Option<bool>,
}

/// Body for [`Campaigns::create`]. Pick the constructor matching the campaign type.
///
/// ```
/// use mailerlite_rs::campaigns::{
///     AbSettings, CampaignEmail, CreateCampaign, TimeUnit, WinnerMetric, WinnerSelection,
/// };
///
/// let email = CampaignEmail::new("Hello", "Ann", "ann@example.com");
/// let regular = CreateCampaign::regular("Newsletter", email.clone()).groups(["42"]);
/// let ab = CreateCampaign::ab(
///     "Subject test",
///     email,
///     AbSettings::subject(
///         "Hi there",
///         WinnerSelection::new(WinnerMetric::Opens, 4, TimeUnit::Hours),
///         20,
///     ),
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CreateCampaign {
    /// Campaign name (at most 255 characters).
    pub name: String,
    /// Language id for the unsubscribe template (see [`Campaigns::languages`]). Defaults to
    /// English.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<u64>,
    /// Campaign type. `resend` and `multivariate` require a Growing or Advanced plan.
    #[serde(rename = "type")]
    pub campaign_type: CampaignType,
    /// Emails: exactly one, except multivariate content tests (one per variation).
    pub emails: Vec<CampaignEmail>,
    /// Recipient groups.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Id>,
    /// Recipient segments. If both groups and segments are set, only segments are used.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<Id>,
    /// A/B test settings (type `ab`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ab_settings: Option<AbSettings>,
    /// Auto resend settings (type `resend`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resend_settings: Option<ResendSettings>,
    /// Multivariate settings (type `multivariate`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multivariate_settings: Option<MultivariateSettings>,
    /// Extra settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<NewCampaignSettings>,
}

impl CreateCampaign {
    fn with(name: String, campaign_type: CampaignType, emails: Vec<CampaignEmail>) -> Self {
        Self {
            name,
            language_id: None,
            campaign_type,
            emails,
            groups: Vec::new(),
            segments: Vec::new(),
            ab_settings: None,
            resend_settings: None,
            multivariate_settings: None,
            settings: None,
        }
    }

    /// A regular campaign sending `email`.
    pub fn regular(name: impl Into<String>, email: CampaignEmail) -> Self {
        Self::with(name.into(), CampaignType::Regular, vec![email])
    }

    /// An A/B test of `email` (version A) with `settings` describing version B.
    pub fn ab(name: impl Into<String>, email: CampaignEmail, settings: AbSettings) -> Self {
        Self {
            ab_settings: Some(settings),
            ..Self::with(name.into(), CampaignType::Ab, vec![email])
        }
    }

    /// An auto resend campaign: `email` first, then a resend described by `settings`.
    pub fn resend(name: impl Into<String>, email: CampaignEmail, settings: ResendSettings) -> Self {
        Self {
            resend_settings: Some(settings),
            ..Self::with(name.into(), CampaignType::Resend, vec![email])
        }
    }

    /// A multivariate test. Pass one email per content variation
    /// ([`CampaignEmail::variation`]); subjects and senders come from `settings`.
    pub fn multivariate(
        name: impl Into<String>,
        emails: impl IntoIterator<Item = CampaignEmail>,
        settings: MultivariateSettings,
    ) -> Self {
        Self {
            multivariate_settings: Some(settings),
            ..Self::with(
                name.into(),
                CampaignType::Multivariate,
                emails.into_iter().collect(),
            )
        }
    }

    /// Sets the unsubscribe template language.
    #[must_use]
    pub fn language_id(mut self, language_id: u64) -> Self {
        self.language_id = Some(language_id);
        self
    }

    /// Sends to these groups.
    #[must_use]
    pub fn groups<I: Into<Id>>(mut self, groups: impl IntoIterator<Item = I>) -> Self {
        self.groups.extend(groups.into_iter().map(Into::into));
        self
    }

    /// Sends to these segments (takes precedence over groups).
    #[must_use]
    pub fn segments<I: Into<Id>>(mut self, segments: impl IntoIterator<Item = I>) -> Self {
        self.segments.extend(segments.into_iter().map(Into::into));
        self
    }

    /// Turns e-commerce link tracking on or off.
    #[must_use]
    pub fn ecommerce_tracking(mut self, enabled: bool) -> Self {
        self.settings = Some(NewCampaignSettings {
            ecommerce_tracking: Some(enabled),
        });
        self
    }
}

/// Body for [`Campaigns::update`] (draft campaigns only). The campaign type cannot change;
/// pass the A/B or resend settings again for those types.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct UpdateCampaign {
    /// Campaign name (at most 255 characters).
    pub name: String,
    /// Language id for the unsubscribe template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<u64>,
    /// Emails (exactly one).
    pub emails: Vec<CampaignEmail>,
    /// Recipient groups.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Id>,
    /// Recipient segments. If both groups and segments are set, only segments are used.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<Id>,
    /// A/B test settings (A/B campaigns).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ab_settings: Option<AbSettings>,
    /// Auto resend settings (resend campaigns).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resend_settings: Option<ResendSettings>,
}

impl UpdateCampaign {
    /// Sets the name and the campaign's email.
    pub fn new(name: impl Into<String>, email: CampaignEmail) -> Self {
        Self {
            name: name.into(),
            language_id: None,
            emails: vec![email],
            groups: Vec::new(),
            segments: Vec::new(),
            ab_settings: None,
            resend_settings: None,
        }
    }

    /// Sets the unsubscribe template language.
    #[must_use]
    pub fn language_id(mut self, language_id: u64) -> Self {
        self.language_id = Some(language_id);
        self
    }

    /// Sends to these groups.
    #[must_use]
    pub fn groups<I: Into<Id>>(mut self, groups: impl IntoIterator<Item = I>) -> Self {
        self.groups.extend(groups.into_iter().map(Into::into));
        self
    }

    /// Sends to these segments (takes precedence over groups).
    #[must_use]
    pub fn segments<I: Into<Id>>(mut self, segments: impl IntoIterator<Item = I>) -> Self {
        self.segments.extend(segments.into_iter().map(Into::into));
        self
    }

    /// Sets the A/B test settings.
    #[must_use]
    pub fn ab(mut self, settings: AbSettings) -> Self {
        self.ab_settings = Some(settings);
        self
    }

    /// Sets the auto resend settings.
    #[must_use]
    pub fn resend(mut self, settings: ResendSettings) -> Self {
        self.resend_settings = Some(settings);
        self
    }
}

/// A send date and time.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct Schedule {
    /// Date, `YYYY-MM-DD`, in the future.
    pub date: String,
    /// Hour, `HH`.
    pub hours: String,
    /// Minute, `mm`.
    pub minutes: String,
    /// Timezone id (see the timezones endpoint). Defaults to the account's timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_id: Option<u64>,
}

impl Schedule {
    /// `date` (`YYYY-MM-DD`) at `hours`:`minutes` (`HH`, `mm`).
    pub fn new(
        date: impl Into<String>,
        hours: impl Into<String>,
        minutes: impl Into<String>,
    ) -> Self {
        Self {
            date: date.into(),
            hours: hours.into(),
            minutes: minutes.into(),
            timezone_id: None,
        }
    }

    /// Interprets the time in this timezone.
    #[must_use]
    pub fn timezone(mut self, timezone_id: u64) -> Self {
        self.timezone_id = Some(timezone_id);
        self
    }
}

/// When the second email of an auto resend campaign goes out.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ResendSchedule {
    /// Resend delivery type.
    pub delivery: ResendDelivery,
    /// Resend date and time.
    #[serde(flatten)]
    pub schedule: Schedule,
}

impl ResendSchedule {
    /// Resends with `delivery` at `schedule`.
    #[must_use]
    pub fn new(delivery: ResendDelivery, schedule: Schedule) -> Self {
        Self { delivery, schedule }
    }
}

/// Body for [`Campaigns::schedule`].
///
/// ```
/// use mailerlite_rs::campaigns::{Schedule, ScheduleCampaign};
///
/// let now = ScheduleCampaign::instant();
/// let later = ScheduleCampaign::scheduled(Schedule::new("2030-01-31", "09", "30").timezone(1));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ScheduleCampaign {
    /// Delivery type (required unless the campaign is an RSS campaign).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Delivery>,
    /// Date and time for `scheduled`, `timezone_based` and `smart_sending` delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// Second email of an auto resend campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resend: Option<ResendSchedule>,
    /// The two sending times of an A/B sending time test.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sending_time_test_schedule: Vec<Schedule>,
}

impl ScheduleCampaign {
    fn with(delivery: Option<Delivery>, schedule: Option<Schedule>) -> Self {
        Self {
            delivery,
            schedule,
            resend: None,
            sending_time_test_schedule: Vec::new(),
        }
    }

    /// Sends right away.
    #[must_use]
    pub fn instant() -> Self {
        Self::with(Some(Delivery::Instant), None)
    }

    /// Sends at `schedule`.
    #[must_use]
    pub fn scheduled(schedule: Schedule) -> Self {
        Self::with(Some(Delivery::Scheduled), Some(schedule))
    }

    /// Sends at `schedule`'s time in each subscriber's timezone.
    #[must_use]
    pub fn timezone_based(schedule: Schedule) -> Self {
        Self::with(Some(Delivery::TimezoneBased), Some(schedule))
    }

    /// Smart sending starting at `schedule`.
    #[must_use]
    pub fn smart_sending(schedule: Schedule) -> Self {
        Self::with(Some(Delivery::SmartSending), Some(schedule))
    }

    /// An A/B sending time test sending version A at `a` and version B at `b`.
    ///
    /// The docs do not say which `delivery` goes with a sending time test; this sends
    /// `scheduled`. Set [`ScheduleCampaign::delivery`] directly if the API expects another.
    #[must_use]
    pub fn sending_time_test(a: Schedule, b: Schedule) -> Self {
        Self {
            sending_time_test_schedule: vec![a, b],
            ..Self::with(Some(Delivery::Scheduled), None)
        }
    }

    /// An empty body, for RSS campaigns, which take no `delivery`.
    #[must_use]
    pub fn rss() -> Self {
        Self::with(None, None)
    }

    /// Schedules the second email of an auto resend campaign.
    #[must_use]
    pub fn resend(mut self, resend: ResendSchedule) -> Self {
        self.resend = Some(resend);
        self
    }
}

/// One recipient's activity in a sent campaign.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CampaignActivity {
    /// Activity id.
    pub id: Id,
    /// Opens.
    #[serde(default, deserialize_with = "de::null_default")]
    pub opens_count: u64,
    /// Clicks.
    #[serde(default, deserialize_with = "de::null_default")]
    pub clicks_count: u64,
    /// The recipient, when requested with [`ListCampaignActivity::include`].
    #[serde(default)]
    pub subscriber: Option<Subscriber>,
}

/// Recipient counts per activity type returned with [`Campaigns::subscriber_activity`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CampaignActivityAggregations {
    /// All recipients.
    #[serde(default, deserialize_with = "de::null_default")]
    pub sum: u64,
    /// Opened.
    #[serde(default, deserialize_with = "de::null_default")]
    pub opened: u64,
    /// Did not open.
    #[serde(default, deserialize_with = "de::null_default")]
    pub unopened: u64,
    /// Clicked.
    #[serde(default, deserialize_with = "de::null_default")]
    pub clicked: u64,
    /// Unsubscribed.
    #[serde(default, deserialize_with = "de::null_default")]
    pub unsubscribed: u64,
    /// Forwarded.
    #[serde(default, deserialize_with = "de::null_default")]
    pub forwarded: u64,
    /// Hard bounced.
    #[serde(default, deserialize_with = "de::null_default")]
    pub hardbounced: u64,
    /// Soft bounced.
    #[serde(default, deserialize_with = "de::null_default")]
    pub softbounced: u64,
    /// Marked as junk.
    #[serde(default, deserialize_with = "de::null_default")]
    pub junk: u64,
}

/// Query for [`Campaigns::subscriber_activity`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListCampaignActivity {
    /// Only recipients with this activity. Defaults to all recipients.
    #[serde(rename = "filter[type]", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<CampaignActivityType>,
    /// Only the recipient with this email.
    #[serde(rename = "filter[search]", skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Page size: 10, 25, 50 (API default) or 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order (API default `id`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort<CampaignActivitySort>>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Embed related resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<CampaignActivityInclude>,
}

impl ListCampaignActivity {
    /// Default query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Only recipients with `activity_type`.
    #[must_use]
    pub fn activity_type(mut self, activity_type: CampaignActivityType) -> Self {
        self.activity_type = Some(activity_type);
        self
    }

    /// Only the recipient with `email`.
    #[must_use]
    pub fn search(mut self, email: impl Into<String>) -> Self {
        self.search = Some(email.into());
        self
    }

    /// Page size (10, 25, 50 or 100).
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sort order, e.g. `Sort::desc(CampaignActivitySort::OpensCount)`.
    #[must_use]
    pub fn sort(mut self, sort: Sort<CampaignActivitySort>) -> Self {
        self.sort = Some(sort);
        self
    }

    /// 1-based page number.
    #[must_use]
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Embeds the subscriber (or the subscriber and their groups).
    #[must_use]
    pub fn include(mut self, include: CampaignActivityInclude) -> Self {
        self.include = Some(include);
        self
    }
}

/// Response of [`Campaigns::ecommerce_stats`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CampaignEcommerceReport {
    /// Orders and revenue attributed to the campaign.
    #[serde(default)]
    pub ecommerce: EcommerceSummary,
    /// The email that earned the most, only set when the campaign has more than one email
    /// (e.g. an A/B test).
    #[serde(default)]
    pub best_performing_email: Option<Email>,
}

/// A language for the unsubscribe template of a campaign.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Language {
    /// Language id, for [`CreateCampaign::language_id`].
    pub id: Id,
    /// Short code, e.g. `en`.
    #[serde(default)]
    pub shortcode: String,
    /// Locale, e.g. `en-US`.
    #[serde(default)]
    pub iso639: String,
    /// Name, e.g. `English`.
    #[serde(default)]
    pub name: String,
    /// Writing direction.
    #[serde(default)]
    pub direction: Option<TextDirection>,
}

/// Campaign endpoints. Obtained with [`MailerLite::campaigns`].
#[derive(Clone, Copy, Debug)]
pub struct Campaigns<'a> {
    client: &'a MailerLite,
}

impl MailerLite {
    /// Campaign endpoints.
    #[must_use]
    pub fn campaigns(&self) -> Campaigns<'_> {
        Campaigns { client: self }
    }
}

impl<'a> Campaigns<'a> {
    /// `GET /campaigns`: one page of campaigns. Only `ready` campaigns unless
    /// [`ListCampaigns::status`] says otherwise.
    pub async fn list(
        &self,
        query: &ListCampaigns,
    ) -> Result<Page<Campaign, CampaignAggregations>> {
        self.client
            .call(Method::GET, &["campaigns"])
            .query(query)
            .send()
            .await
    }

    /// Every campaign matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        &self,
        query: ListCampaigns,
    ) -> impl futures_util::Stream<Item = Result<Campaign>> + 'a {
        let campaigns = *self;
        crate::pagination::stream::pages(query.page.unwrap_or(1), move |page| {
            let query = ListCampaigns {
                page: Some(page),
                ..query.clone()
            };
            async move { campaigns.list(&query).await }
        })
    }

    /// `GET /campaigns/{id}`: one campaign.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Campaign> {
        self.client
            .call(Method::GET, &["campaigns", id.as_ref()])
            .data()
            .await
    }

    /// `POST /campaigns`: creates a draft campaign.
    pub async fn create(&self, campaign: &CreateCampaign) -> Result<Campaign> {
        self.client
            .call(Method::POST, &["campaigns"])
            .json(campaign)
            .data()
            .await
    }

    /// `PUT /campaigns/{id}`: updates a draft campaign (`422` for other statuses).
    pub async fn update(&self, id: impl AsRef<str>, campaign: &UpdateCampaign) -> Result<Campaign> {
        self.client
            .call(Method::PUT, &["campaigns", id.as_ref()])
            .json(campaign)
            .data()
            .await
    }

    /// `POST /campaigns/{id}/schedule`: sends or schedules a campaign.
    ///
    /// ```no_run
    /// # async fn run(client: mailerlite_rs::MailerLite) -> mailerlite_rs::Result<()> {
    /// use mailerlite_rs::campaigns::ScheduleCampaign;
    ///
    /// let campaign = client
    ///     .campaigns()
    ///     .schedule("1", &ScheduleCampaign::instant())
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub async fn schedule(
        &self,
        id: impl AsRef<str>,
        schedule: &ScheduleCampaign,
    ) -> Result<Campaign> {
        self.client
            .call(Method::POST, &["campaigns", id.as_ref(), "schedule"])
            .json(schedule)
            .data()
            .await
    }

    /// `POST /campaigns/{id}/cancel`: moves a `ready` campaign back to draft (`400` for other
    /// statuses).
    pub async fn cancel(&self, id: impl AsRef<str>) -> Result<Campaign> {
        self.client
            .call(Method::POST, &["campaigns", id.as_ref(), "cancel"])
            .data()
            .await
    }

    /// `DELETE /campaigns/{id}`: deletes a campaign.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<()> {
        self.client
            .call(Method::DELETE, &["campaigns", id.as_ref()])
            .send_empty()
            .await
    }

    /// `GET /campaigns/{id}/reports/subscriber-activity`: one page of recipient activity of a
    /// sent campaign.
    pub async fn subscriber_activity(
        &self,
        id: impl AsRef<str>,
        query: &ListCampaignActivity,
    ) -> Result<Page<CampaignActivity, CampaignActivityAggregations>> {
        self.client
            .call(
                Method::GET,
                &["campaigns", id.as_ref(), "reports", "subscriber-activity"],
            )
            .query(query)
            .send()
            .await
    }

    /// Every recipient activity matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn subscriber_activity_all(
        &self,
        id: impl Into<Id>,
        query: ListCampaignActivity,
    ) -> impl futures_util::Stream<Item = Result<CampaignActivity>> + 'a {
        let campaigns = *self;
        let id = id.into();
        crate::pagination::stream::pages(query.page.unwrap_or(1), move |page| {
            let query = ListCampaignActivity {
                page: Some(page),
                ..query.clone()
            };
            let id = id.clone();
            async move { campaigns.subscriber_activity(&id, &query).await }
        })
    }

    /// `GET /campaigns/{id}/reports/ecommerce`: orders and revenue the campaign earned in a
    /// shop.
    pub async fn ecommerce_stats(
        &self,
        id: impl AsRef<str>,
        shop_id: impl AsRef<str>,
    ) -> Result<CampaignEcommerceReport> {
        self.client
            .call(
                Method::GET,
                &["campaigns", id.as_ref(), "reports", "ecommerce"],
            )
            .query(&ShopQuery {
                shop_id: shop_id.as_ref(),
            })
            .data()
            .await
    }

    /// `GET /campaigns/{id}/reports/ecommerce/orders`: orders the campaign earned in a shop.
    pub async fn ecommerce_orders(
        &self,
        id: impl AsRef<str>,
        query: &ListReportOrders,
    ) -> Result<Page<ReportOrder>> {
        self.client
            .call(
                Method::GET,
                &["campaigns", id.as_ref(), "reports", "ecommerce", "orders"],
            )
            .query(query)
            .send()
            .await
    }

    /// `GET /campaigns/languages`: languages available for the unsubscribe template.
    pub async fn languages(&self) -> Result<Vec<Language>> {
        self.client
            .call(Method::GET, &["campaigns", "languages"])
            .data()
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::{Value, json};

    use super::*;
    use crate::http::mock::{client, fixture};

    fn email() -> CampaignEmail {
        CampaignEmail::new("Hello", "Ann", "ann@example.com")
    }

    #[tokio::test]
    async fn list_with_filters_and_aggregations() {
        let (client, mock) = client();
        mock.respond(200, fixture("campaigns_list"));

        let page = client
            .campaigns()
            .list(
                &ListCampaigns::new()
                    .status(CampaignStatus::Sent)
                    .campaign_type(CampaignType::Ab)
                    .limit(10)
                    .page(2),
            )
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "campaigns?filter%5Bstatus%5D=sent&filter%5Btype%5D=ab&limit=10&page=2",
        );
        let campaign = &page.data[0];
        assert_eq!(campaign.status, CampaignStatus::Sent);
        assert_eq!(campaign.campaign_type, CampaignType::Regular);
        assert_eq!(campaign.delivery_schedule, Some(Delivery::Instant));
        assert_eq!(campaign.language_id.as_deref(), Some("4"));
        assert!(campaign.settings.track_opens);
        assert_eq!(campaign.filter[0][0]["operator"], "in_any");
        assert_eq!(
            campaign.filter_for_humans[0][0],
            "In any group: dummy group"
        );
        assert_eq!(campaign.emails[0].stats.unique_opens_count, 5);
        assert_eq!(
            campaign.emails[0].reply_to.as_deref(),
            Some("reply-to@mailerlite.io")
        );
        let stats = campaign.stats.as_ref().unwrap();
        assert_eq!(stats.click_rate.string, "20%");
        assert_eq!(stats.click_to_open_rate.as_ref().unwrap().string, "100%");
        assert_eq!(campaign.has_winner, None);
        assert!(campaign.uses_survey);
        let aggregations = page.aggregations.unwrap();
        assert_eq!(
            (aggregations.sum, aggregations.draft, aggregations.sent),
            (2, 1, 1)
        );
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn list_all_walks_pages() {
        use futures_util::TryStreamExt;

        let (client, mock) = client();
        let campaign = fixture("campaigns_list")["data"][0].clone();
        mock.respond(
            200,
            json!({"data": [campaign], "meta": {"current_page": 1, "last_page": 2}}),
        )
        .respond(
            200,
            json!({"data": [campaign], "meta": {"current_page": 2, "last_page": 2}}),
        );

        let all: Vec<Campaign> = client
            .campaigns()
            .list_all(ListCampaigns::new().status(CampaignStatus::Draft))
            .try_collect()
            .await
            .unwrap();

        assert_eq!(all.len(), 2);
        let requests = mock.requests();
        requests[0].assert(Method::GET, "campaigns?filter%5Bstatus%5D=draft&page=1");
        requests[1].assert(Method::GET, "campaigns?filter%5Bstatus%5D=draft&page=2");
    }

    #[tokio::test]
    async fn get() {
        let (client, mock) = client();
        mock.respond(200, fixture("campaigns_get"));

        let campaign = client.campaigns().get("1").await.unwrap();

        mock.request().assert(Method::GET, "campaigns/1").no_body();
        assert_eq!(campaign.id, "1");
        assert_eq!(campaign.status, CampaignStatus::Draft);
        assert_eq!(campaign.stats, None);
        assert_eq!(
            campaign.default_email_id.as_deref(),
            Some("61706700654118223")
        );
        assert_eq!(campaign.emails[0].email_type.as_deref(), Some("builder"));
        assert_eq!(campaign.emails[0].language_id, None);
    }

    #[tokio::test]
    async fn create_regular() {
        let (client, mock) = client();
        mock.respond(200, fixture("campaigns_create"));

        let campaign = client
            .campaigns()
            .create(
                &CreateCampaign::regular(
                    "Newsletter",
                    email().reply_to("support@example.com").content("<p>Hi</p>"),
                )
                .language_id(4)
                .groups(["42"])
                .segments(["7"])
                .ecommerce_tracking(true),
            )
            .await
            .unwrap();

        mock.request()
            .assert(Method::POST, "campaigns")
            .body(&json!({
                "name": "Newsletter",
                "language_id": 4,
                "type": "regular",
                "emails": [{
                    "subject": "Hello",
                    "from_name": "Ann",
                    "from": "ann@example.com",
                    "reply_to": "support@example.com",
                    "content": "<p>Hi</p>"
                }],
                "groups": ["42"],
                "segments": ["7"],
                "settings": {"ecommerce_tracking": true}
            }));
        assert_eq!(campaign.name, "Dummy campaign");
    }

    #[tokio::test]
    async fn create_ab_and_resend() {
        let (client, mock) = client();
        for _ in 0..4 {
            mock.respond(200, fixture("campaigns_create"));
        }
        let winner = WinnerSelection::new(WinnerMetric::Opens, 4, TimeUnit::Hours);

        let campaigns = client.campaigns();
        campaigns
            .create(&CreateCampaign::ab(
                "A/B",
                email(),
                AbSettings::subject("Hi there", winner.clone(), 20),
            ))
            .await
            .unwrap();
        campaigns
            .create(&CreateCampaign::ab(
                "A/B sender",
                email(),
                AbSettings::sender("Bob", "bob@example.com", winner.clone(), 10),
            ))
            .await
            .unwrap();
        campaigns
            .create(&CreateCampaign::ab(
                "A/B time",
                email(),
                AbSettings::sending_time(),
            ))
            .await
            .unwrap();
        campaigns
            .create(&CreateCampaign::resend(
                "Resend",
                email(),
                ResendSettings::new("Did you miss this?", WinnerMetric::Clicks),
            ))
            .await
            .unwrap();

        let email = json!({"subject": "Hello", "from_name": "Ann", "from": "ann@example.com"});
        let requests = mock.requests();
        requests[0].assert(Method::POST, "campaigns").body(&json!({
            "name": "A/B",
            "type": "ab",
            "emails": [email],
            "ab_settings": {
                "test_type": "subject",
                "select_winner_by": "o",
                "after_time_amount": 4,
                "after_time_unit": "h",
                "test_split": 20,
                "b_value": {"subject": "Hi there"}
            }
        }));
        requests[1].body(&json!({
            "name": "A/B sender",
            "type": "ab",
            "emails": [email],
            "ab_settings": {
                "test_type": "sender",
                "select_winner_by": "o",
                "after_time_amount": 4,
                "after_time_unit": "h",
                "test_split": 10,
                "b_value": {"from_name": "Bob", "from": "bob@example.com"}
            }
        }));
        requests[2].body(&json!({
            "name": "A/B time",
            "type": "ab",
            "emails": [email],
            "ab_settings": {"test_type": "sending_time", "test_split": 100}
        }));
        requests[3].body(&json!({
            "name": "Resend",
            "type": "resend",
            "emails": [email],
            "resend_settings": {
                "test_type": "subject",
                "select_winner_by": "c",
                "b_value": {"subject": "Did you miss this?"}
            }
        }));
    }

    #[tokio::test]
    async fn create_multivariate() {
        let (client, mock) = client();
        mock.respond(200, fixture("campaigns_create"));

        client
            .campaigns()
            .create(&CreateCampaign::multivariate(
                "Multi",
                [
                    CampaignEmail::variation().content("<p>A</p>"),
                    CampaignEmail::variation().content("<p>B</p>"),
                ],
                MultivariateSettings::new("Hello", Sender::new("Ann", "ann@example.com"), 20)
                    .subject("Hi")
                    .content(2)
                    .winner(WinnerSelection::new(
                        WinnerMetric::Clicks,
                        1,
                        TimeUnit::Days,
                    )),
            ))
            .await
            .unwrap();

        mock.request()
            .assert(Method::POST, "campaigns")
            .body(&json!({
                "name": "Multi",
                "type": "multivariate",
                "emails": [{"content": "<p>A</p>"}, {"content": "<p>B</p>"}],
                "multivariate_settings": {
                    "test_variants": {
                        "subject": ["Hello", "Hi"],
                        "sender": [{"from": "ann@example.com", "from_name": "Ann"}],
                        "content": 2,
                        "sending_time": 1
                    },
                    "select_winner_by": "c",
                    "after_time_amount": 1,
                    "after_time_unit": "d",
                    "test_split": 20
                }
            }));
    }

    #[tokio::test]
    async fn update() {
        let (client, mock) = client();
        mock.respond(200, fixture("campaigns_update"));

        let campaign = client
            .campaigns()
            .update(
                "1",
                &UpdateCampaign::new("Renamed", email())
                    .language_id(4)
                    .groups(["42"])
                    .resend(ResendSettings::new("Again", WinnerMetric::Opens)),
            )
            .await
            .unwrap();

        mock.request()
            .assert(Method::PUT, "campaigns/1")
            .body(&json!({
                "name": "Renamed",
                "language_id": 4,
                "emails": [{"subject": "Hello", "from_name": "Ann", "from": "ann@example.com"}],
                "groups": ["42"],
                "resend_settings": {
                    "test_type": "subject",
                    "select_winner_by": "o",
                    "b_value": {"subject": "Again"}
                }
            }));
        assert_eq!(campaign.status, CampaignStatus::Draft);
    }

    #[tokio::test]
    async fn update_non_draft_error_is_a_bare_string() {
        let (client, mock) = client();
        mock.respond(422, json!("Campaign is not with status draft"));

        let err = client
            .campaigns()
            .update("1", &UpdateCampaign::new("Renamed", email()))
            .await
            .unwrap_err();

        assert_eq!(
            err.as_api().unwrap().message,
            "Campaign is not with status draft"
        );
    }

    #[tokio::test]
    async fn schedule() {
        let (client, mock) = client();
        for _ in 0..5 {
            mock.respond(200, fixture("campaigns_schedule"));
        }

        let campaigns = client.campaigns();
        let sent = campaigns
            .schedule("1", &ScheduleCampaign::instant())
            .await
            .unwrap();
        campaigns
            .schedule(
                "1",
                &ScheduleCampaign::scheduled(Schedule::new("2030-01-31", "09", "30").timezone(1))
                    .resend(ResendSchedule::new(
                        ResendDelivery::Day,
                        Schedule::new("2030-02-01", "10", "00"),
                    )),
            )
            .await
            .unwrap();
        campaigns
            .schedule(
                "1",
                &ScheduleCampaign::timezone_based(Schedule::new("2030-01-31", "09", "30")),
            )
            .await
            .unwrap();
        campaigns
            .schedule(
                "1",
                &ScheduleCampaign::sending_time_test(
                    Schedule::new("2030-01-31", "09", "00").timezone(1),
                    Schedule::new("2030-01-31", "18", "00").timezone(1),
                ),
            )
            .await
            .unwrap();
        campaigns
            .schedule("1", &ScheduleCampaign::rss())
            .await
            .unwrap();

        assert_eq!(sent.status, CampaignStatus::Sent);
        let requests = mock.requests();
        requests[0]
            .assert(Method::POST, "campaigns/1/schedule")
            .body(&json!({"delivery": "instant"}));
        requests[1].body(&json!({
            "delivery": "scheduled",
            "schedule": {"date": "2030-01-31", "hours": "09", "minutes": "30", "timezone_id": 1},
            "resend": {"delivery": "day", "date": "2030-02-01", "hours": "10", "minutes": "00"}
        }));
        requests[2].body(&json!({
            "delivery": "timezone_based",
            "schedule": {"date": "2030-01-31", "hours": "09", "minutes": "30"}
        }));
        requests[3].body(&json!({
            "delivery": "scheduled",
            "sending_time_test_schedule": [
                {"date": "2030-01-31", "hours": "09", "minutes": "00", "timezone_id": 1},
                {"date": "2030-01-31", "hours": "18", "minutes": "00", "timezone_id": 1}
            ]
        }));
        requests[4].body(&json!({}));
    }

    #[tokio::test]
    async fn cancel_and_delete() {
        let (client, mock) = client();
        mock.respond(200, fixture("campaigns_cancel"))
            .respond(204, Value::Null);

        let campaign = client.campaigns().cancel("1").await.unwrap();
        client.campaigns().delete("1").await.unwrap();

        let requests = mock.requests();
        requests[0]
            .assert(Method::POST, "campaigns/1/cancel")
            .no_body();
        requests[1].assert(Method::DELETE, "campaigns/1").no_body();
        assert_eq!(campaign.status, CampaignStatus::Draft);
        assert_eq!(campaign.scheduled_for, None);
    }

    #[tokio::test]
    async fn subscriber_activity() {
        let (client, mock) = client();
        mock.respond(200, fixture("campaigns_subscriber_activity"));

        let page = client
            .campaigns()
            .subscriber_activity(
                "1",
                &ListCampaignActivity::new()
                    .activity_type(CampaignActivityType::Opened)
                    .search("dummy@mailerlite.com")
                    .limit(25)
                    .sort(Sort::desc(CampaignActivitySort::OpensCount))
                    .page(2)
                    .include(CampaignActivityInclude::SubscriberGroups),
            )
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "campaigns/1/reports/subscriber-activity?filter%5Btype%5D=opened&filter%5Bsearch%5D=dummy%40mailerlite.com&limit=25&sort=-opens_count&page=2&include=subscriber.groups",
        );
        let activity = &page.data[0];
        assert_eq!(activity.opens_count, 1);
        assert_eq!(
            activity.subscriber.as_ref().unwrap().email,
            "dummy@mailerlite.com"
        );
        let aggregations = page.aggregations.unwrap();
        assert_eq!((aggregations.sum, aggregations.unopened), (11, 10));
    }

    #[tokio::test]
    async fn ecommerce_reports() {
        let (client, mock) = client();
        mock.respond(200, fixture("campaigns_ecommerce_stats"))
            .respond(200, fixture("campaigns_ecommerce_orders"));

        let stats = client.campaigns().ecommerce_stats("1", "9").await.unwrap();
        let orders = client
            .campaigns()
            .ecommerce_orders("1", &ListReportOrders::new("9").limit(5).page(2))
            .await
            .unwrap();

        let requests = mock.requests();
        requests[0].assert(Method::GET, "campaigns/1/reports/ecommerce?shop_id=9");
        requests[1].assert(
            Method::GET,
            "campaigns/1/reports/ecommerce/orders?shop_id=9&limit=5&page=2",
        );
        assert_eq!(stats.ecommerce.orders_count, 2);
        assert_eq!(stats.ecommerce.conversion_rate, "2%");
        assert_eq!(stats.best_performing_email, None);
        assert_eq!(orders.data[0].subscriber.id, "56152210168350162");
        assert!((orders.data[0].total - 20.0).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn languages() {
        let (client, mock) = client();
        mock.respond(200, fixture("campaigns_languages"));

        let languages = client.campaigns().languages().await.unwrap();

        mock.request().assert(Method::GET, "campaigns/languages");
        assert_eq!(languages[0].id, "4");
        assert_eq!(languages[0].iso639, "en-US");
        assert_eq!(languages[0].direction, Some(TextDirection::Ltr));
    }

    #[test]
    fn settings_accept_php_empty_array() {
        let mut campaign = fixture("campaigns_get")["data"].clone();
        campaign["settings"] = json!([]);
        campaign["emails"][0]["language_id"] = json!(4);

        let campaign: Campaign = serde_json::from_value(campaign).unwrap();

        assert_eq!(campaign.settings, CampaignSettings::default());
        assert_eq!(campaign.emails[0].language_id.as_deref(), Some("4"));
    }
}
