//! Automation report schemas verified against the MailerLite backend resources.
use crate::models::{
    campaigns::CampaignEmail,
    ecommerce::{ReportSubscriber, wire},
};
use rust_decimal::Decimal;
use serde::Deserialize;

/// Automation email HTML.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationEmailContent {
    /// Email HTML content.
    pub html: String,
}

/// Formatted percentage used by automation reports.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationReportRate {
    /// Percentage including its percent sign.
    pub string: String,
}

/// Engagement and deliverability statistics.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationEmailReport {
    /// Engagement counts and rates.
    pub engagement: AutomationEngagement,
    /// Delivery counts and rates.
    pub deliverability: AutomationDeliverability,
}

/// Automation engagement statistics.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationEngagement {
    /// Sent.
    pub sent: u64,
    /// Opens count.
    pub opens_count: u64,
    /// Clicks count.
    pub clicks_count: u64,
    /// Total opens count.
    pub total_opens_count: u64,
    /// Total clicks count.
    pub total_clicks_count: u64,
    /// Forwards count.
    pub forwards_count: u64,
    /// Social interactions count.
    pub social_interactions_count: u64,
    /// Open rate.
    pub open_rate: AutomationReportRate,
    /// Click rate.
    pub click_rate: AutomationReportRate,
    /// Click to open rate.
    pub click_to_open_rate: AutomationReportRate,
    /// Forward rate.
    pub forward_rate: AutomationReportRate,
    /// Social interaction rate.
    pub social_interaction_rate: AutomationReportRate,
}

/// Automation delivery statistics.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationDeliverability {
    /// Sent.
    pub sent: u64,
    /// Delivered count.
    pub delivered_count: u64,
    /// Hard bounces count.
    pub hard_bounces_count: u64,
    /// Soft bounces count.
    pub soft_bounces_count: u64,
    /// Spam count.
    pub spam_count: u64,
    /// Unsubscribes count.
    pub unsubscribes_count: u64,
    /// Delivered rate.
    pub delivered_rate: AutomationReportRate,
    /// Hard bounce rate.
    pub hard_bounce_rate: AutomationReportRate,
    /// Soft bounce rate.
    pub soft_bounce_rate: AutomationReportRate,
    /// Spam rate.
    pub spam_rate: AutomationReportRate,
    /// Unsubscribe rate.
    pub unsubscribe_rate: AutomationReportRate,
}

/// Attributed e-commerce totals.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationEcommerceMetrics {
    /// Number of orders.
    pub orders_count: u64,
    /// Attributed revenue.
    #[serde(deserialize_with = "wire::decimal")]
    pub revenue: Decimal,
    /// Average order value.
    #[serde(deserialize_with = "wire::decimal")]
    pub average_order_value: Decimal,
    /// Formatted conversion rate, when supplied.
    pub conversion_rate: Option<String>,
}

/// E-commerce performance for one email.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationEmailEcommerceReport {
    /// Attributed e-commerce totals.
    pub ecommerce: AutomationEcommerceMetrics,
}

/// E-commerce performance for an automation.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationEcommerceReport {
    /// Attributed e-commerce totals.
    pub ecommerce: AutomationEcommerceMetrics,
    /// Highest-revenue email, when available.
    pub best_performing_email: Option<AutomationBestPerformingEmail>,
}

/// Highest-revenue email and its attribution statistics.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationBestPerformingEmail {
    /// Email identifier.
    pub email_id: String,
    /// Number of orders.
    pub orders_count: u64,
    /// Attributed revenue.
    #[serde(deserialize_with = "wire::decimal")]
    pub revenue: Decimal,
    /// Number of deliveries attempted.
    pub sent_count: u64,
    /// Numeric conversion ratio.
    pub conversion_rate: f64,
    /// Average order value.
    #[serde(deserialize_with = "wire::decimal")]
    pub average_order_value: Decimal,
    /// Email screenshot URL.
    pub screenshot_url: Option<String>,
    /// Email subject.
    pub subject: Option<String>,
    /// Email creation timestamp.
    pub created_at: Option<String>,
}

/// A subscriber-provided unsubscribe reason.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationUnsubscribeReason {
    /// Reason label.
    pub label: String,
    /// Number of occurrences.
    pub count: u64,
    /// Percentage of all reasons.
    pub percentage: f64,
}

/// An order attributed to an automation email.
#[derive(Debug, Clone, Deserialize)]
pub struct AutomationReportOrder {
    /// Customer subscriber identity.
    pub subscriber: ReportSubscriber,
    /// Order total.
    #[serde(deserialize_with = "wire::decimal")]
    pub total: Decimal,
    /// Last update timestamp.
    pub updated_at: String,
    /// Cart item summary; absent when no cart is attached.
    pub order: Option<String>,
    /// Attributing email when loaded.
    pub email: Option<CampaignEmail>,
}
