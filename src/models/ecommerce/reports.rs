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
    /// Conversion rate in the server-provided representation, when available.
    ///
    /// Shop and subscriber reports may omit this field. Absence does not mean zero.
    pub conversion_rate: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::EcommerceStats;
    use serde_json::json;

    #[test]
    fn null_conversion_rate_is_absent() {
        let result = serde_json::from_value::<EcommerceStats>(json!({
            "orders_count": 0, "revenue": 0, "average_order_value": 0,
            "conversion_rate": null
        }));

        assert!(matches!(result, Ok(stats) if stats.conversion_rate.is_none()));
    }

    #[test]
    fn malformed_conversion_rate_is_rejected() {
        for value in [json!(2), json!(false), json!([]), json!({"rate": "2%"})] {
            let result = serde_json::from_value::<EcommerceStats>(json!({
                "orders_count": 2, "revenue": 40, "average_order_value": 20,
                "conversion_rate": value
            }));

            assert!(result.is_err());
        }
    }
}
