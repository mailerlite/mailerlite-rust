//! E-commerce statistics and reporting responses.
use super::wire;
use crate::models::SubscriberId;
use rust_decimal::Decimal;
use serde::Deserialize;

/// EcommerceStats returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct EcommerceStats {
    /// Orders count.
    pub orders_count: u64,
    /// Revenue.
    #[serde(deserialize_with = "wire::decimal")]
    pub revenue: Decimal,
    /// Average order value.
    #[serde(deserialize_with = "wire::decimal")]
    pub average_order_value: Decimal,
    /// Conversion rate.
    pub conversion_rate: String,
}

/// StatsReport returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct StatsReport {
    /// Ecommerce.
    pub ecommerce: EcommerceStats,
    /// Best performing email.
    pub best_performing_email: Option<crate::models::campaigns::CampaignEmail>,
    /// Series.
    pub series: Option<Vec<StatsPoint>>,
}

/// StatsPoint returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct StatsPoint {
    /// Date.
    pub date: String,
    /// Campaign orders count.
    pub campaign_orders_count: u64,
    /// Automation orders count.
    pub automation_orders_count: u64,
    /// Automation revenue.
    #[serde(deserialize_with = "wire::decimal")]
    pub automation_revenue: Decimal,
    /// Campaign revenue.
    #[serde(deserialize_with = "wire::decimal")]
    pub campaign_revenue: Decimal,
}

/// ReportOrder returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportOrder {
    /// Subscriber.
    pub subscriber: ReportSubscriber,
    /// Total.
    #[serde(deserialize_with = "wire::decimal")]
    pub total: Decimal,
    /// Updated at.
    pub updated_at: String,
    /// Order.
    pub order: String,
    /// Email.
    pub email: Option<crate::models::campaigns::CampaignEmail>,
}

/// ReportSubscriber returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportSubscriber {
    /// Id.
    #[serde(deserialize_with = "wire::id")]
    pub id: SubscriberId,
    /// Email.
    pub email: String,
}
