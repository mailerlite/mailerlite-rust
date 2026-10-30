//! Orders and their attribution.
use super::{Cart, Customer, Shop, wire};
use crate::models::OrderId;
use rust_decimal::Decimal;
use serde::Deserialize;

/// Order returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    /// Id.
    #[serde(deserialize_with = "wire::id")]
    pub id: OrderId,
    /// Resource id.
    pub resource_id: Option<String>,
    /// Shop.
    pub shop: Option<Shop>,
    /// Customer.
    pub customer: Option<Customer>,
    /// Cart.
    pub cart: Option<Cart>,
    /// Total.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub total: Option<Decimal>,
    /// Status.
    pub status: String,
    /// Attribution.
    pub attribution: Option<Attribution>,
    /// Created at.
    pub created_at: Option<String>,
    /// Updated at.
    pub updated_at: Option<String>,
    /// Completed at.
    pub completed_at: Option<String>,
}

/// Attribution returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct Attribution {
    /// Email id.
    pub email_id: String,
    /// Type.
    #[serde(rename = "type")]
    pub kind: String,
}
