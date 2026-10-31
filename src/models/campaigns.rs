//! Campaigns, messages, and engagement reports.
use crate::models::CampaignId;

pub use crate::models::common::Rate;

/// Campaign returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct Campaign {
    /// MailerLite identifier, preserved as text.
    pub id: CampaignId,
    /// Display name assigned to this resource.
    pub name: String,
    /// Server type discriminator, preserving unrecognized values as text.
    #[serde(rename = "type")]
    pub kind: String,
    /// Server-reported state, preserving unrecognized values as text.
    pub status: String,
    /// Identifier of the account that owns the campaign.
    pub account_id: Option<String>,
    /// Creation timestamp as returned by the server; no timezone is inferred.
    pub created_at: Option<String>,
    /// Last modification timestamp as returned by the server.
    pub updated_at: Option<String>,
    /// Scheduled start time, retained in the server's timestamp format.
    pub scheduled_for: Option<String>,
    /// Timestamp when the campaign entered the sending queue.
    pub queued_at: Option<String>,
    /// Timestamp when campaign sending began.
    pub started_at: Option<String>,
    /// Timestamp when campaign sending finished.
    pub finished_at: Option<String>,
    /// Timestamp when campaign sending was stopped.
    pub stopped_at: Option<String>,
    /// Email versions included with this campaign; absent when not loaded.
    pub emails: Option<Vec<CampaignEmail>>,
    /// Default email to edit after campaign creation.
    pub default_email_id: Option<String>,
    /// Tracking settings included with this campaign.
    pub settings: Option<CampaignSettings>,
    /// Whether the server currently permits scheduling this campaign.
    pub can_be_scheduled: Option<bool>,
    /// Whether the campaign is currently sending messages.
    pub is_currently_sending_out: Option<bool>,
}
/// CampaignSettings returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct CampaignSettings {
    /// Whether open tracking is enabled for the campaign.
    pub track_opens: Option<bool>,
    /// Whether campaign links include Google Analytics tracking.
    pub use_google_analytics: Option<bool>,
    /// Whether e-commerce attribution is enabled for the campaign.
    pub ecommerce_tracking: Option<bool>,
}
/// CampaignEmail returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct CampaignEmail {
    /// Server editor type, such as `builder_inline_email`.
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// MailerLite identifier, preserved as text.
    pub id: String,
    /// Email subject line, when included in the response.
    pub subject: Option<String>,
    /// Sender email address shown on the message.
    pub from: Option<String>,
    /// Sender display name shown on the message.
    pub from_name: Option<String>,
    /// Email address used for replies, when configured.
    pub reply_to: Option<String>,
    /// Plain-text version of the email body, when included.
    pub plain_text: Option<String>,
    /// URL where the rendered email can be previewed.
    pub preview_url: Option<String>,
    /// URL of the generated visual preview, when available.
    pub screenshot_url: Option<String>,
    /// Engagement statistics included for this email.
    pub stats: Option<EmailStats>,
}
/// EmailStats returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct EmailStats {
    /// Number of emails sent, rather than the number successfully delivered.
    pub sent: Option<u64>,
    /// Unique recipient opens reported by the email statistics resource.
    pub opens_count: Option<u64>,
    /// Number of distinct recipients who opened an email, when reported.
    pub unique_opens_count: Option<u64>,
    /// Unique recipient clicks reported by the email statistics resource.
    pub clicks_count: Option<u64>,
    /// Number of distinct recipients who clicked a link, when reported.
    pub unique_clicks_count: Option<u64>,
    /// Number of attributed unsubscriptions.
    pub unsubscribes_count: Option<u64>,
    /// Number of attributed spam complaints.
    pub spam_count: Option<u64>,
    /// Number of permanent delivery failures.
    pub hard_bounces_count: Option<u64>,
    /// Number of temporary delivery failures.
    pub soft_bounces_count: Option<u64>,
    /// Number of tracked email forwards.
    pub forwards_count: Option<u64>,
    /// Email open rate in the server-provided rate representation.
    pub open_rate: Option<Rate>,
    /// Email click rate in the server-provided rate representation.
    pub click_rate: Option<Rate>,
    /// Unsubscription rate in the server-provided rate representation.
    pub unsubscribe_rate: Option<Rate>,
    /// Spam-complaint rate in the server-provided rate representation.
    pub spam_rate: Option<Rate>,
    /// Permanent-delivery-failure rate in the server-provided representation.
    pub hard_bounce_rate: Option<Rate>,
    /// Temporary-delivery-failure rate in the server-provided representation.
    pub soft_bounce_rate: Option<Rate>,
}
/// CampaignActivity returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct CampaignActivity {
    /// MailerLite identifier, preserved as text.
    pub id: String,
    /// Number of opens recorded for this activity entry.
    pub opens_count: Option<u64>,
    /// Number of clicks recorded for this activity entry.
    pub clicks_count: Option<u64>,
    /// Subscriber associated with this activity, when included.
    pub subscriber: Option<crate::models::subscribers::Subscriber>,
}
