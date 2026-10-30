//! E-commerce report types shared by subscribers, campaigns and automations.

use serde::{Deserialize, Serialize};

use crate::types::{Id, Timestamp, de};

/// `{"ecommerce": {...}}` summary returned by the `reports/ecommerce` endpoints.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EcommerceReport {
    /// Aggregated order statistics.
    #[serde(default)]
    pub ecommerce: EcommerceSummary,
}

/// Order statistics attributed to a subscriber, campaign or shop.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EcommerceSummary {
    /// Number of orders.
    #[serde(default)]
    pub orders_count: u64,
    /// Total revenue in the shop's currency.
    #[serde(default, deserialize_with = "de::number")]
    pub revenue: f64,
    /// Average order value in the shop's currency.
    #[serde(default, deserialize_with = "de::number")]
    pub average_order_value: f64,
    /// Conversion rate as displayed by MailerLite, e.g. `"2%"`.
    #[serde(default)]
    pub conversion_rate: String,
}

/// One attributed order in a `reports/ecommerce/orders` listing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReportOrder {
    /// The ordering subscriber.
    pub subscriber: ReportSubscriber,
    /// Order total in the shop's currency.
    #[serde(default, deserialize_with = "de::number")]
    pub total: f64,
    /// Human readable order summary, e.g. `"2 X T-shirt, 1 X Mug"`.
    #[serde(default)]
    pub order: String,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
    /// The email the order is attributed to (subscriber reports only). Kept as raw JSON
    /// because MailerLite returns a reduced, endpoint-specific email object here.
    #[serde(default)]
    pub email: Option<serde_json::Value>,
}

/// Subscriber reference inside a [`ReportOrder`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReportSubscriber {
    /// Subscriber id.
    pub id: Id,
    /// Subscriber email.
    pub email: String,
}

/// Query for `reports/ecommerce/orders` endpoints.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListReportOrders {
    /// Shop to report on (required).
    pub shop_id: Id,
    /// Page size (API default 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl ListReportOrders {
    /// Orders for `shop_id`.
    pub fn new(shop_id: impl Into<Id>) -> Self {
        Self {
            shop_id: shop_id.into(),
            limit: None,
            page: None,
        }
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

#[derive(Serialize)]
pub(crate) struct ShopQuery<'a> {
    pub(crate) shop_id: &'a str,
}
