//! Customer records and accepted import responses.
use super::wire;
use crate::models::CustomerId;
use rust_decimal::Decimal;
use serde::Deserialize;

/// Customer returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Customer {
    /// MailerLite identifier, preserved as text.
    #[serde(deserialize_with = "wire::id")]
    pub id: CustomerId,
    /// Email address of the customer.
    pub email: String,
    /// Identifier assigned by the connected shop, distinct from the MailerLite ID.
    pub resource_id: Option<String>,
    /// Whether the customer has opted in to marketing.
    pub accepts_marketing: Option<bool>,
    /// Cumulative customer spending, retained as an exact decimal.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub total_spent: Option<Decimal>,
    /// Number of orders recorded for this customer or report.
    pub orders_count: Option<u64>,
    /// Linked MailerLite subscriber, when included.
    pub subscriber: Option<crate::models::subscribers::Subscriber>,
}

/// AcceptedCustomer returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct AcceptedCustomer {
    /// Email address accepted for customer import.
    pub email: String,
    /// Identifier assigned by the connected shop, distinct from the MailerLite ID.
    pub resource_id: Option<String>,
    /// Whether the customer has opted in to marketing.
    pub accepts_marketing: Option<bool>,
    /// Whether the accepted import requested creation of a subscriber.
    pub create_subscriber: Option<bool>,
    /// Customer spending amount accepted by the import.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub total_spent: Option<Decimal>,
    /// Number of orders recorded for this customer or report.
    pub orders_count: Option<u64>,
}
