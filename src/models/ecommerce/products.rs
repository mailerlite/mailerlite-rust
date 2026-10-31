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
    /// MailerLite identifier, preserved as text.
    #[serde(deserialize_with = "wire::id")]
    pub id: ProductId,
    /// Display name assigned to this resource.
    pub name: String,
    /// Identifier assigned by the connected shop, distinct from the MailerLite ID.
    pub resource_id: Option<String>,
    /// Exact decimal price without conversion through floating point.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub price: Option<Decimal>,
    /// URL of the product page in the connected shop.
    pub url: Option<String>,
    /// Product image location supplied by the shop.
    pub image: Option<String>,
    /// Whether this resource is excluded from automation processing.
    pub exclude_from_automations: Option<bool>,
    /// Whether this resource is enabled on the server.
    pub enabled: Option<bool>,
    /// Assigned categories, when included in the response.
    pub categories: Option<Vec<Category>>,
    /// Product variants, when included in the response.
    pub variants: Option<Vec<ProductVariant>>,
    /// Short product description used in listings.
    pub short_description: Option<String>,
    /// Full product description supplied by the shop.
    pub description: Option<String>,
    /// Server type discriminator, preserving unrecognized values as text.
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// Currency identifier reported for this resource.
    pub currency: Option<String>,
    /// Display symbol for the currency, distinct from its identifier.
    pub currency_symbol: Option<String>,
    /// Discounts associated with this product, when included.
    pub discounts: Option<Vec<Discount>>,
    /// Digital delivery or booking assets, when included.
    pub assets: Option<Vec<ProductAsset>>,
    /// Resolved digital or booking settings; an empty array becomes empty settings.
    #[serde(default, deserialize_with = "product_details::optional_settings")]
    pub settings: Option<ProductSettings>,
    /// Whether the product price is below the Stripe minimum.
    pub stripe_minimum: Option<bool>,
    /// Whether a checkout is available for this product.
    pub has_checkout: Option<bool>,
    /// URL where the customer can complete checkout.
    pub checkout_url: Option<String>,
    /// Aggregate completed-order counts and amounts, when included.
    pub order_stats: Option<ProductOrderStats>,
    /// Creation timestamp as returned by the server; no timezone is inferred.
    pub created_at: Option<String>,
}

/// ProductVariant returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ProductVariant {
    /// MailerLite identifier, preserved as text.
    #[serde(deserialize_with = "wire::id")]
    pub id: String,
    /// Display name assigned to this resource.
    pub name: String,
    /// Variant attribute names mapped to their selected values.
    pub attributes: BTreeMap<String, String>,
    /// Exact decimal price without conversion through floating point.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub price: Option<Decimal>,
    /// Server price-mode discriminator, preserved as text.
    pub price_type: Option<String>,
    /// Identifier assigned by the connected shop, distinct from the MailerLite ID.
    pub resource_id: Option<String>,
    /// URL of the variant-specific image.
    pub image_url: Option<String>,
}

/// Discount returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Discount {
    /// MailerLite identifier, preserved as text.
    #[serde(deserialize_with = "wire::id")]
    pub id: String,
    /// Display name assigned to this resource.
    pub name: Option<String>,
    /// Discount amount or percentage, according to `is_percentage`.
    #[serde(default, deserialize_with = "wire::optional_decimal")]
    pub value: Option<Decimal>,
    /// Whether `value` is a percentage rather than a fixed amount.
    pub is_percentage: Option<bool>,
    /// Whether this resource is enabled on the server.
    pub enabled: Option<bool>,
    /// Beginning of the discount validity period, when supplied.
    pub start_date: Option<String>,
    /// End of the discount validity period, when supplied.
    pub end_date: Option<String>,
    /// Creation timestamp as returned by the server; no timezone is inferred.
    pub created_at: Option<String>,
    /// Last modification timestamp as returned by the server.
    pub updated_at: Option<String>,
    /// Resulting product price after this discount, when supplied.
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
