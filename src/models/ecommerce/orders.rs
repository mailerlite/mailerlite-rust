//! Orders and their attribution.
use super::{Cart, Customer, Shop, wire};
use crate::models::OrderId;
use rust_decimal::Decimal;
use serde::Deserialize;

/// Order returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Order {
    /// MailerLite identifier, preserved as text.
    #[serde(deserialize_with = "wire::id")]
    pub id: OrderId,
    /// Identifier assigned by the connected shop, distinct from the MailerLite ID.
    pub resource_id: Option<String>,
    /// Shop associated with this order, when included.
    pub shop: Option<Shop>,
    /// Purchasing customer, when included.
    pub customer: Option<Customer>,
    /// Cart associated with this order, when included.
    pub cart: Option<Cart>,
    /// Exact decimal order total without conversion through floating point.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub total: Option<Decimal>,
    /// Server-reported state, preserving unrecognized values as text.
    pub status: String,
    /// Email attribution attached to this order, when available.
    pub attribution: Option<Attribution>,
    /// Creation timestamp as returned by the server; no timezone is inferred.
    pub created_at: Option<String>,
    /// Last modification timestamp as returned by the server.
    pub updated_at: Option<String>,
    /// Order completion timestamp, when the server reports completion.
    pub completed_at: Option<String>,
}

/// Attribution returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Attribution {
    /// Identifier of the email credited with the order.
    pub email_id: String,
    /// Attribution source type, preserved as supplied by the server.
    #[serde(rename = "type")]
    pub kind: String,
}
