//! Shopping carts and their items.
use super::{Customer, Product, wire};
use crate::models::{CartId, CartItemId};
use rust_decimal::Decimal;
use serde::Deserialize;

/// Cart returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Cart {
    /// MailerLite identifier, preserved as text.
    #[serde(deserialize_with = "wire::id")]
    pub id: CartId,
    /// Identifier assigned by the connected shop, distinct from the MailerLite ID.
    pub resource_id: Option<String>,
    /// MailerLite order identifier associated with this cart, when present.
    pub ecommerce_order_id: Option<String>,
    /// URL where the customer can complete checkout.
    pub checkout_url: Option<String>,
    /// Total cart amount, retained as an exact decimal.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub cart_total: Option<Decimal>,
    /// Customer associated with this cart, when included.
    pub customer: Option<Customer>,
    /// Cart line items, when included in the response.
    pub items: Option<Vec<CartItem>>,
}

/// CartItem returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct CartItem {
    /// MailerLite identifier, preserved as text.
    #[serde(deserialize_with = "wire::id")]
    pub id: CartItemId,
    /// Identifier assigned by the connected shop, distinct from the MailerLite ID.
    pub resource_id: Option<String>,
    /// Product represented by this cart line, when included.
    pub product: Option<Product>,
    /// MailerLite product-variant identifier for this line, when present.
    pub ecommerce_product_variant_id: Option<String>,
    /// Variant description supplied for this cart line.
    pub variant: Option<String>,
    /// Number of units represented by this cart line.
    pub quantity: Option<u64>,
    /// Exact decimal price without conversion through floating point.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub price: Option<Decimal>,
}
