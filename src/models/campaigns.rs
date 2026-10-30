//! Campaigns, messages, and engagement reports.
use crate::models::CampaignId;
/// Campaign returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Campaign {
    /// Id.
    pub id: CampaignId,
    /// Name.
    pub name: String,
    /// Kind.
    #[serde(rename = "type")]
    pub kind: String,
    /// Status.
    pub status: String,
    /// Account id.
    pub account_id: Option<String>,
    /// Created at.
    pub created_at: Option<String>,
    /// Updated at.
    pub updated_at: Option<String>,
    /// Scheduled for.
    pub scheduled_for: Option<String>,
    /// Queued at.
    pub queued_at: Option<String>,
    /// Started at.
    pub started_at: Option<String>,
    /// Finished at.
    pub finished_at: Option<String>,
    /// Stopped at.
    pub stopped_at: Option<String>,
    /// Emails.
    pub emails: Option<Vec<CampaignEmail>>,
    /// Settings.
    pub settings: Option<CampaignSettings>,
    /// Can be scheduled.
    pub can_be_scheduled: Option<bool>,
    /// Is currently sending out.
    pub is_currently_sending_out: Option<bool>,
}
/// CampaignSettings returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CampaignSettings {
    /// Track opens.
    pub track_opens: Option<bool>,
    /// Use google analytics.
    pub use_google_analytics: Option<bool>,
    /// Ecommerce tracking.
    pub ecommerce_tracking: Option<bool>,
}
/// CampaignEmail returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CampaignEmail {
    /// Id.
    pub id: String,
    /// Subject.
    pub subject: Option<String>,
    /// From.
    pub from: Option<String>,
    /// From name.
    pub from_name: Option<String>,
    /// Reply to.
    pub reply_to: Option<String>,
    /// Plain text.
    pub plain_text: Option<String>,
    /// Preview url.
    pub preview_url: Option<String>,
    /// Screenshot url.
    pub screenshot_url: Option<String>,
    /// Stats.
    pub stats: Option<EmailStats>,
}
/// Rate returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Rate {
    /// Float.
    pub float: f64,
    /// String.
    pub string: String,
}
/// EmailStats returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct EmailStats {
    /// Sent.
    pub sent: Option<u64>,
    /// Opens count.
    pub opens_count: Option<u64>,
    /// Unique opens count.
    pub unique_opens_count: Option<u64>,
    /// Clicks count.
    pub clicks_count: Option<u64>,
    /// Unique clicks count.
    pub unique_clicks_count: Option<u64>,
    /// Unsubscribes count.
    pub unsubscribes_count: Option<u64>,
    /// Spam count.
    pub spam_count: Option<u64>,
    /// Hard bounces count.
    pub hard_bounces_count: Option<u64>,
    /// Soft bounces count.
    pub soft_bounces_count: Option<u64>,
    /// Forwards count.
    pub forwards_count: Option<u64>,
    /// Open rate.
    pub open_rate: Option<Rate>,
    /// Click rate.
    pub click_rate: Option<Rate>,
    /// Unsubscribe rate.
    pub unsubscribe_rate: Option<Rate>,
    /// Spam rate.
    pub spam_rate: Option<Rate>,
    /// Hard bounce rate.
    pub hard_bounce_rate: Option<Rate>,
    /// Soft bounce rate.
    pub soft_bounce_rate: Option<Rate>,
}
/// CampaignActivity returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CampaignActivity {
    /// Id.
    pub id: String,
    /// Opens count.
    pub opens_count: Option<u64>,
    /// Clicks count.
    pub clicks_count: Option<u64>,
    /// Subscriber.
    pub subscriber: Option<crate::models::subscribers::Subscriber>,
}
