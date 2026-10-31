//! E-commerce statistics and reporting responses.
use super::wire;
use crate::models::SubscriberId;
use rust_decimal::Decimal;
use serde::Deserialize;

/// EcommerceStats returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EcommerceStats {
    /// Number of orders recorded for this customer or report.
    pub orders_count: u64,
    /// Attributed revenue as an exact decimal amount.
    #[serde(deserialize_with = "wire::decimal")]
    pub revenue: Decimal,
    /// Average attributed order amount, retained as an exact decimal.
    #[serde(deserialize_with = "wire::decimal")]
    pub average_order_value: Decimal,
    /// Conversion rate in the server-provided representation.
    pub conversion_rate: String,
}

/// StatsReport returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct StatsReport {
    /// Aggregate order and revenue statistics for the report.
    pub ecommerce: EcommerceStats,
    /// Email selected by the server as best performing, when available.
    pub best_performing_email: Option<crate::models::campaigns::CampaignEmail>,
    /// Time-series data points, when included.
    pub series: Option<Vec<StatsPoint>>,
}

/// StatsPoint returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct StatsPoint {
    /// Date label of this reporting bucket, retained as supplied.
    pub date: String,
    /// Orders attributed to campaigns in this reporting bucket.
    pub campaign_orders_count: u64,
    /// Orders attributed to automations in this reporting bucket.
    pub automation_orders_count: u64,
    /// Revenue attributed to automations in this bucket, as an exact decimal.
    #[serde(deserialize_with = "wire::decimal")]
    pub automation_revenue: Decimal,
    /// Revenue attributed to campaigns in this bucket, as an exact decimal.
    #[serde(deserialize_with = "wire::decimal")]
    pub campaign_revenue: Decimal,
}

/// ReportOrder returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ReportOrder {
    /// Subscriber associated with the reported order.
    pub subscriber: ReportSubscriber,
    /// Exact decimal order total without conversion through floating point.
    #[serde(deserialize_with = "wire::decimal")]
    pub total: Decimal,
    /// Last modification timestamp as returned by the server.
    pub updated_at: String,
    /// Order reference supplied by the reporting endpoint.
    pub order: String,
    /// Attributed campaign or automation email, when included.
    pub email: Option<crate::models::campaigns::CampaignEmail>,
}

/// ReportSubscriber returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ReportSubscriber {
    /// MailerLite identifier, preserved as text.
    #[serde(deserialize_with = "wire::id")]
    pub id: SubscriberId,
    /// Email address of the subscriber associated with the report.
    pub email: String,
}
