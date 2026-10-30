//! Shopping carts and their items.
use super::{Customer, Product, wire};
use crate::models::{CartId, CartItemId};
use rust_decimal::Decimal;
use serde::Deserialize;

/// Cart returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct Cart {
    /// Id.
    #[serde(deserialize_with = "wire::id")]
    pub id: CartId,
    /// Resource id.
    pub resource_id: Option<String>,
    /// Ecommerce order id.
    pub ecommerce_order_id: Option<String>,
    /// Checkout url.
    pub checkout_url: Option<String>,
    /// Cart total.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub cart_total: Option<Decimal>,
    /// Customer.
    pub customer: Option<Customer>,
    /// Items.
    pub items: Option<Vec<CartItem>>,
}

/// CartItem returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct CartItem {
    /// Id.
    #[serde(deserialize_with = "wire::id")]
    pub id: CartItemId,
    /// Resource id.
    pub resource_id: Option<String>,
    /// Product.
    pub product: Option<Product>,
    /// Ecommerce product variant id.
    pub ecommerce_product_variant_id: Option<String>,
    /// Variant.
    pub variant: Option<String>,
    /// Quantity.
    pub quantity: Option<u64>,
    /// Price.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub price: Option<Decimal>,
}
