//! Campaign sending, open, and click events.
use super::WebhookSubscriber;
use crate::models::CampaignId;
use serde::Deserialize;
use std::fmt;

/// Campaign fields flattened beside the `event` discriminator.
#[derive(Clone, Deserialize)]
pub struct CampaignSentEvent {
    /// Campaign whose sending completed.
    #[serde(flatten)]
    pub campaign: WebhookCampaign,
    /// Account identifier, when included.
    pub account_id: Option<u64>,
}
impl fmt::Debug for CampaignSentEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CampaignSentEvent(<payload omitted>)")
    }
}

/// A subscriber opened an email belonging to a campaign.
#[derive(Clone, Deserialize)]
pub struct CampaignOpenEvent {
    /// Subscriber who opened the email.
    pub subscriber: WebhookSubscriber,
    /// Opened campaign identity and delivery summary.
    pub campaign: WebhookCampaign,
    /// Account identifier, when included.
    pub account_id: Option<u64>,
}
impl fmt::Debug for CampaignOpenEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CampaignOpenEvent(<payload omitted>)")
    }
}

/// A subscriber clicked a campaign link.
#[derive(Clone, Deserialize)]
pub struct CampaignClickEvent {
    /// Subscriber who clicked the link.
    pub subscriber: WebhookSubscriber,
    /// Clicked campaign identity and delivery summary.
    pub campaign: WebhookCampaign,
    /// Link destination supplied by MailerLite, possibly empty.
    pub link_url: String,
    /// Account identifier, when included.
    pub account_id: Option<u64>,
}
impl fmt::Debug for CampaignClickEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CampaignClickEvent(<payload omitted>)")
    }
}

/// Campaign identity and delivery summary used by webhook events.
#[derive(Clone, Deserialize)]
pub struct WebhookCampaign {
    /// Stable campaign identifier.
    pub id: CampaignId,
    /// Campaign name.
    pub name: String,
    /// Number of recipients sent the campaign.
    pub total_recipients: u64,
    /// Campaign preview destination, absent when no email preview is available.
    pub preview_url: Option<String>,
    /// Campaign completion date or event date, as supplied by MailerLite.
    pub date: String,
}
impl fmt::Debug for WebhookCampaign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WebhookCampaign(<payload omitted>)")
    }
}
