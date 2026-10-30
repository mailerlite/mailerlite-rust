//! Product variants: <https://developers.mailerlite.com/ecommerce/product-variants>

use std::collections::BTreeMap;

use reqwest::Method;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::{Product, Scope};
use crate::{
    Result,
    types::{Id, de, string_enum},
};

/// A product variant, e.g. "Black / Medium".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Variant {
    /// Variant id.
    pub id: Id,
    /// Variant name.
    pub name: String,
    /// Attributes, e.g. `{"color": "Black", "size": "Medium"}`.
    #[serde(default, deserialize_with = "attributes")]
    pub attributes: BTreeMap<String, String>,
    /// Price (absolute or added to the product price, see [`price_type`](Self::price_type)).
    #[serde(default, deserialize_with = "de::optional_number")]
    pub price: Option<f64>,
    /// How [`price`](Self::price) applies.
    #[serde(default)]
    pub price_type: Option<PriceType>,
    /// Your own id for the variant, if any.
    #[serde(default)]
    pub resource_id: Option<String>,
    /// Variant image URL.
    #[serde(default)]
    pub image_url: Option<String>,
}

string_enum! {
    /// How a variant's price applies.
    pub enum PriceType {
        /// The variant's price replaces the product price.
        Fixed => "fixed",
        /// The variant's price is added to the product price.
        Increment => "increment",
    }
}

/// Attribute map that tolerates PHP's `[]` for an empty object and non-string scalars.
fn attributes<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<BTreeMap<String, String>, D::Error> {
    Ok(de::object(deserializer)?
        .into_iter()
        .filter_map(|(key, value)| match value {
            Value::Null => None,
            Value::String(text) => Some((key, text)),
            other => Some((key, other.to_string())),
        })
        .collect())
}

/// One variant for [`Variants::create`] and the body of [`Variants::update`].
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CreateVariant {
    /// Variant name.
    pub name: String,
    /// Attributes, e.g. `{"color": "Black"}` (required by the API).
    pub attributes: BTreeMap<String, String>,
    /// How [`price`](Self::price) applies. Requires `price`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type: Option<PriceType>,
    /// Variant price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Your own id for the variant; identifies an existing variant to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Variant image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

impl CreateVariant {
    /// A variant called `name`. Add at least one [`attribute`](Self::attribute).
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: BTreeMap::new(),
            price_type: None,
            price: None,
            resource_id: None,
            image_url: None,
        }
    }

    /// Sets an attribute, e.g. `.attribute("color", "Black")`.
    #[must_use]
    pub fn attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Price and how it applies (the API requires a price whenever a price type is sent).
    #[must_use]
    pub fn price(mut self, price_type: PriceType, price: f64) -> Self {
        self.price_type = Some(price_type);
        self.price = Some(price);
        self
    }

    /// Your own id for the variant.
    #[must_use]
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    /// Variant image URL.
    #[must_use]
    pub fn image_url(mut self, image_url: impl Into<String>) -> Self {
        self.image_url = Some(image_url.into());
        self
    }
}

/// Variant endpoints of one product. Obtained with
/// [`ShopScope::variants`](super::ShopScope::variants).
#[derive(Clone, Copy, Debug)]
pub struct Variants<'a> {
    pub(super) scope: Scope<'a>,
    pub(super) product_id: &'a str,
}

impl Variants<'_> {
    /// `GET /ecommerce/shops/{shop_id}/products/{product_id}/variants`: every variant of the
    /// product (not paginated).
    pub async fn list(&self) -> Result<Vec<Variant>> {
        self.scope
            .call(Method::GET, &["products", self.product_id, "variants"])
            .data()
            .await
    }

    /// `POST /ecommerce/shops/{shop_id}/products/{product_id}/variants`: creates or updates
    /// several variants and returns the product with its variants.
    ///
    /// # Warning: this is a sync
    ///
    /// If **any** variant in `variants` has a [`resource_id`](CreateVariant::resource_id),
    /// every existing variant of the product whose `resource_id` is not in the request is
    /// **deleted**. Always send the complete list of variants when you use resource ids.
    pub async fn create(&self, variants: &[CreateVariant]) -> Result<Product> {
        self.scope
            .call(Method::POST, &["products", self.product_id, "variants"])
            .json(variants)
            .data()
            .await
    }

    /// `PUT /ecommerce/shops/{shop_id}/products/{product_id}/variants/{id}`: updates one
    /// variant and returns the product with its variants.
    pub async fn update(&self, id: impl AsRef<str>, variant: &CreateVariant) -> Result<Product> {
        self.scope
            .call(
                Method::PUT,
                &["products", self.product_id, "variants", id.as_ref()],
            )
            .json(variant)
            .data()
            .await
    }

    /// `DELETE /ecommerce/shops/{shop_id}/products/{product_id}/variants/{id}`: deletes one
    /// variant and returns the product with its remaining variants.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Product> {
        self.scope
            .call(
                Method::DELETE,
                &["products", self.product_id, "variants", id.as_ref()],
            )
            .data()
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::json;

    use super::*;
    use crate::http::mock::{client, fixture};

    #[tokio::test]
    async fn list() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_variants_list"));
        let shop = client.ecommerce().shop("9");

        let variants = shop.variants("4").list().await.unwrap();

        mock.request()
            .assert(Method::GET, "ecommerce/shops/9/products/4/variants")
            .no_body();
        let variant = &variants[0];
        assert_eq!(variant.id, "31986843543567900");
        assert_eq!(variant.attributes["size"], "Medium");
        assert_eq!(variant.price, Some(123.4));
        assert_eq!(variant.price_type, Some(PriceType::Fixed));
        assert_eq!(variant.resource_id.as_deref(), Some("variant-123"));
    }

    #[tokio::test]
    async fn create_update_delete() {
        let (client, mock) = client();
        for _ in 0..3 {
            mock.respond(200, fixture("ecommerce_product_with_variants"));
        }
        let shop = client.ecommerce().shop("9");
        let variants = shop.variants("4");

        let product = variants
            .create(&[CreateVariant::new("Black / Medium")
                .attribute("color", "Black")
                .attribute("size", "Medium")
                .price(PriceType::Fixed, 123.4)
                .resource_id("variant-123")
                .image_url("https://shop-url.com/product-black.jpg")])
            .await
            .unwrap();
        variants
            .update(
                "5",
                &CreateVariant::new("Black / Large")
                    .attribute("color", "Black")
                    .attribute("size", "Large"),
            )
            .await
            .unwrap();
        variants.delete("5").await.unwrap();

        assert_eq!(product.variants.len(), 1);
        assert_eq!(product.variants[0].name, "Black / Medium");
        let requests = mock.requests();
        requests[0]
            .assert(Method::POST, "ecommerce/shops/9/products/4/variants")
            .body(&json!([{
                "name": "Black / Medium",
                "attributes": {"color": "Black", "size": "Medium"},
                "price": 123.4,
                "price_type": "fixed",
                "resource_id": "variant-123",
                "image_url": "https://shop-url.com/product-black.jpg"
            }]));
        requests[1]
            .assert(Method::PUT, "ecommerce/shops/9/products/4/variants/5")
            .body(&json!({
                "name": "Black / Large",
                "attributes": {"color": "Black", "size": "Large"}
            }));
        requests[2]
            .assert(Method::DELETE, "ecommerce/shops/9/products/4/variants/5")
            .no_body();
    }

    #[test]
    fn attributes_tolerate_php_empty_array() {
        let variant: Variant = serde_json::from_value(json!({
            "id": 1, "name": "One size", "attributes": [], "price": null, "price_type": null
        }))
        .unwrap();
        assert!(variant.attributes.is_empty());
        assert_eq!(variant.price, None);
    }
}
