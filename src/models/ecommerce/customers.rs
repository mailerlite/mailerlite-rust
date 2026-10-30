//! Customer records and accepted import responses.
use super::wire;
use crate::models::CustomerId;
use rust_decimal::Decimal;
use serde::Deserialize;

/// Customer returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct Customer {
    /// Id.
    #[serde(deserialize_with = "wire::id")]
    pub id: CustomerId,
    /// Email.
    pub email: String,
    /// Resource id.
    pub resource_id: Option<String>,
    /// Accepts marketing.
    pub accepts_marketing: Option<bool>,
    /// Total spent.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub total_spent: Option<Decimal>,
    /// Orders count.
    pub orders_count: Option<u64>,
    /// Subscriber.
    pub subscriber: Option<crate::models::subscribers::Subscriber>,
}

/// AcceptedCustomer returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct AcceptedCustomer {
    /// Email.
    pub email: String,
    /// Resource id.
    pub resource_id: Option<String>,
    /// Accepts marketing.
    pub accepts_marketing: Option<bool>,
    /// Create subscriber.
    pub create_subscriber: Option<bool>,
    /// Total spent.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub total_spent: Option<Decimal>,
    /// Orders count.
    pub orders_count: Option<u64>,
}
