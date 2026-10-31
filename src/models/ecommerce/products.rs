//! Products, variants, and discounts.
use super::{Category, ProductAsset, ProductOrderStats, ProductSettings, product_details, wire};
use crate::models::ProductId;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::BTreeMap;

/// Product returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Product {
    /// Id.
    #[serde(deserialize_with = "wire::id")]
    pub id: ProductId,
    /// Name.
    pub name: String,
    /// Resource id.
    pub resource_id: Option<String>,
    /// Price.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub price: Option<Decimal>,
    /// Url.
    pub url: Option<String>,
    /// Image.
    pub image: Option<String>,
    /// Exclude from automations.
    pub exclude_from_automations: Option<bool>,
    /// Enabled.
    pub enabled: Option<bool>,
    /// Categories.
    pub categories: Option<Vec<Category>>,
    /// Variants.
    pub variants: Option<Vec<ProductVariant>>,
    /// Short description.
    pub short_description: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Type.
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// Currency.
    pub currency: Option<String>,
    /// Currency symbol.
    pub currency_symbol: Option<String>,
    /// Discounts.
    pub discounts: Option<Vec<Discount>>,
    /// Assets.
    pub assets: Option<Vec<ProductAsset>>,
    /// Settings.
    #[serde(default, deserialize_with = "product_details::optional_settings")]
    pub settings: Option<ProductSettings>,
    /// Stripe minimum.
    pub stripe_minimum: Option<bool>,
    /// Has checkout.
    pub has_checkout: Option<bool>,
    /// Checkout url.
    pub checkout_url: Option<String>,
    /// Order stats.
    pub order_stats: Option<ProductOrderStats>,
    /// Created at.
    pub created_at: Option<String>,
}

/// ProductVariant returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ProductVariant {
    /// Id.
    #[serde(deserialize_with = "wire::id")]
    pub id: String,
    /// Name.
    pub name: String,
    /// Attributes.
    pub attributes: BTreeMap<String, String>,
    /// Price.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub price: Option<Decimal>,
    /// Price type.
    pub price_type: Option<String>,
    /// Resource id.
    pub resource_id: Option<String>,
    /// Image url.
    pub image_url: Option<String>,
}

/// Discount returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Discount {
    /// Id.
    #[serde(deserialize_with = "wire::id")]
    pub id: String,
    /// Name.
    pub name: Option<String>,
    /// Value.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub value: Option<Decimal>,
    /// Is percentage.
    pub is_percentage: Option<bool>,
    /// Enabled.
    pub enabled: Option<bool>,
    /// Start date.
    pub start_date: Option<String>,
    /// End date.
    pub end_date: Option<String>,
    /// Created at.
    pub created_at: Option<String>,
    /// Updated at.
    pub updated_at: Option<String>,
    /// Price after discount.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub price_after_discount: Option<Decimal>,
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use super::Product;
    use rust_decimal::Decimal;

    #[test]
    fn numeric_identifier_and_decimal_response_are_lossless() {
        let product: Product = serde_json::from_str(
            r#"{"id":18446744073709551615,"name":"Product","price":1234567890.123456789012345678}"#,
        )
        .unwrap();
        assert_eq!(product.id.as_ref(), "18446744073709551615");
        assert_eq!(
            product.price.unwrap().to_string(),
            "1234567890.123456789012345678"
        );
    }

    #[test]
    fn fractional_identifier_is_rejected() {
        assert!(serde_json::from_str::<Product>(r#"{"id":1.5,"name":"Product"}"#).is_err());
    }

    #[test]
    fn malformed_decimal_string_is_rejected() {
        assert!(
            serde_json::from_str::<Product>(
                r#"{"id":"1","name":"Product","price":"not a decimal"}"#
            )
            .is_err()
        );
    }

    #[test]
    fn scientific_price_exceeding_decimal_precision_is_rejected() {
        assert!(
            serde_json::from_str::<Product>(
                r#"{"id":"1","name":"Product","price":1.123456789012345678901234567891e0}"#
            )
            .is_err()
        );
    }

    #[test]
    fn scientific_price_with_exact_representation_is_accepted() {
        let product: Product =
            serde_json::from_str(r#"{"id":"1","name":"Product","price":1.25e2}"#).unwrap();
        assert_eq!(product.price.unwrap(), Decimal::from(125));
    }

    #[test]
    fn negative_numeric_identifier_is_rejected() {
        assert!(serde_json::from_str::<Product>(r#"{"id":-1,"name":"Product"}"#).is_err());
    }
}
