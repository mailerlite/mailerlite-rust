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
#[non_exhaustive]
pub struct AutomationEmailReport {
    /// Engagement counts and rates.
    pub engagement: AutomationEngagement,
    /// Delivery counts and rates.
    pub deliverability: AutomationDeliverability,
}

/// Automation engagement statistics.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct AutomationEngagement {
    /// Number of emails sent, rather than the number successfully delivered.
    pub sent: u64,
    /// Server-reported email open count.
    pub opens_count: u64,
    /// Server-reported email click count.
    pub clicks_count: u64,
    /// Total opens, including repeat opens by the same recipient.
    pub total_opens_count: u64,
    /// Total clicks, including repeat clicks by the same recipient.
    pub total_clicks_count: u64,
    /// Number of tracked email forwards.
    pub forwards_count: u64,
    /// Number of tracked social interactions.
    pub social_interactions_count: u64,
    /// Email open rate in the server-provided rate representation.
    pub open_rate: AutomationReportRate,
    /// Email click rate in the server-provided rate representation.
    pub click_rate: AutomationReportRate,
    /// Server-formatted click-to-open percentage.
    pub click_to_open_rate: AutomationReportRate,
    /// Server-formatted email-forward percentage.
    pub forward_rate: AutomationReportRate,
    /// Server-formatted social-interaction percentage.
    pub social_interaction_rate: AutomationReportRate,
}

/// Automation delivery statistics.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct AutomationDeliverability {
    /// Number of emails sent, rather than the number successfully delivered.
    pub sent: u64,
    /// Number of emails reported as delivered.
    pub delivered_count: u64,
    /// Number of permanent delivery failures.
    pub hard_bounces_count: u64,
    /// Number of temporary delivery failures.
    pub soft_bounces_count: u64,
    /// Number of attributed spam complaints.
    pub spam_count: u64,
    /// Number of attributed unsubscriptions.
    pub unsubscribes_count: u64,
    /// Server-formatted email-delivery percentage.
    pub delivered_rate: AutomationReportRate,
    /// Permanent-delivery-failure rate in the server-provided representation.
    pub hard_bounce_rate: AutomationReportRate,
    /// Temporary-delivery-failure rate in the server-provided representation.
    pub soft_bounce_rate: AutomationReportRate,
    /// Spam-complaint rate in the server-provided rate representation.
    pub spam_rate: AutomationReportRate,
    /// Unsubscription rate in the server-provided rate representation.
    pub unsubscribe_rate: AutomationReportRate,
}

/// Attributed e-commerce totals.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct AutomationEcommerceMetrics {
    /// Number of orders.
    pub orders_count: u64,
    /// Attributed revenue.
    #[serde(deserialize_with = "wire::decimal")]
    pub revenue: Decimal,
    /// Average attributed order amount, retained as an exact decimal.
    #[serde(deserialize_with = "wire::decimal")]
    pub average_order_value: Decimal,
    /// Formatted conversion rate, when supplied.
    pub conversion_rate: Option<String>,
}

/// E-commerce performance for one email.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct AutomationEmailEcommerceReport {
    /// Attributed e-commerce totals.
    pub ecommerce: AutomationEcommerceMetrics,
}

/// E-commerce performance for an automation.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct AutomationEcommerceReport {
    /// Attributed e-commerce totals.
    pub ecommerce: AutomationEcommerceMetrics,
    /// Highest-revenue email, when available.
    pub best_performing_email: Option<AutomationBestPerformingEmail>,
}

/// Highest-revenue email and its attribution statistics.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
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
    /// Average attributed order amount, retained as an exact decimal.
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
#[non_exhaustive]
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
#[non_exhaustive]
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
