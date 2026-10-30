//! Products: <https://developers.mailerlite.com/ecommerce/products>

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::{Category, Scope, Variant};
use crate::{
    Page, Result,
    types::{Id, Sort, Timestamp, de, string_enum},
};

/// A product.
///
/// Products of the MailerLite shop (sold through MailerLite checkout) also fill
/// [`product_type`](Self::product_type), [`currency`](Self::currency), [`discounts`](Self::discounts),
/// [`settings`](Self::settings) and the other checkout fields; for other shops they stay
/// empty.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Product {
    /// Product id.
    pub id: Id,
    /// Your own id for the product, if any.
    #[serde(default)]
    pub resource_id: Option<String>,
    /// Product name.
    pub name: String,
    /// Price in the shop's currency.
    #[serde(default, deserialize_with = "de::number")]
    pub price: f64,
    /// Product page URL.
    #[serde(default)]
    pub url: Option<String>,
    /// Main image URL.
    #[serde(default)]
    pub image: Option<String>,
    /// If `true`, the product never triggers automations.
    #[serde(default, deserialize_with = "de::null_default")]
    pub exclude_from_automations: bool,
    /// Whether the product is enabled (`false` if the response omits it).
    #[serde(default, deserialize_with = "de::null_default")]
    pub enabled: bool,
    /// Short description (HTML stripped).
    #[serde(default)]
    pub short_description: Option<String>,
    /// Description (HTML stripped).
    #[serde(default)]
    pub description: Option<String>,
    /// Categories the product belongs to.
    #[serde(default, deserialize_with = "de::null_default")]
    pub categories: Vec<Category>,
    /// Product variants.
    #[serde(default, deserialize_with = "de::null_default")]
    pub variants: Vec<Variant>,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Product type. Every product reports one (`one-time` unless set); other types only
    /// exist in the MailerLite shop.
    #[serde(default, rename = "type")]
    pub product_type: Option<ProductType>,
    /// MailerLite shop: ISO 4217 currency code of the product.
    #[serde(default)]
    pub currency: Option<String>,
    /// MailerLite shop: symbol of [`currency`](Self::currency).
    #[serde(default)]
    pub currency_symbol: Option<String>,
    /// MailerLite shop: discounts on the product.
    #[serde(default, deserialize_with = "de::null_default")]
    pub discounts: Vec<ProductDiscount>,
    /// MailerLite shop: files and links delivered after purchase.
    #[serde(default, deserialize_with = "de::null_default")]
    pub assets: Vec<Value>,
    /// MailerLite shop: post-purchase action and booking settings (`action`, `group_id`,
    /// `booking_type`, ...).
    #[serde(default, deserialize_with = "de::object")]
    pub settings: Map<String, Value>,
    /// MailerLite shop: `true` if the price is below Stripe's minimum charge.
    #[serde(default)]
    pub stripe_minimum: Option<bool>,
    /// MailerLite shop: whether the product has a checkout page.
    #[serde(default)]
    pub has_checkout: Option<bool>,
    /// MailerLite shop: checkout page URL (only when `has_checkout` is `true`).
    #[serde(default)]
    pub checkout_url: Option<String>,
    /// MailerLite shop: orders count and total amount (only in listings).
    #[serde(default)]
    pub order_stats: Option<Value>,
}

string_enum! {
    /// Type of a MailerLite shop product.
    pub enum ProductType {
        /// Paid once.
        OneTime => "one-time",
        /// Recurring subscription.
        Subscription => "subscription",
        /// Bookable appointment.
        Booking => "booking",
    }
}

/// A discount on a MailerLite shop [`Product`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProductDiscount {
    /// Discount id.
    pub id: Id,
    /// Discount name.
    #[serde(default)]
    pub name: Option<String>,
    /// Amount, or percentage if [`is_percentage`](Self::is_percentage).
    #[serde(default, deserialize_with = "de::number")]
    pub value: f64,
    /// Whether `value` is a percentage.
    #[serde(default, deserialize_with = "de::null_default")]
    pub is_percentage: bool,
    /// Whether the discount is active.
    #[serde(default, deserialize_with = "de::null_default")]
    pub enabled: bool,
    /// Start (`Y-m-d H:i:s`).
    #[serde(default)]
    pub start_date: Option<Timestamp>,
    /// End (`Y-m-d H:i:s`).
    #[serde(default)]
    pub end_date: Option<Timestamp>,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
    /// Product price with the discount applied.
    #[serde(default, deserialize_with = "de::optional_number")]
    pub price_after_discount: Option<f64>,
}

string_enum! {
    /// Sort field for [`ListProducts`].
    pub enum ProductSort {
        /// Id (API default).
        Id => "id",
        /// Name.
        Name => "name",
        /// Creation time.
        CreatedAt => "created_at",
    }
}

/// Query for [`Products::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListProducts {
    /// Page size (API default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort<ProductSort>>,
    /// Only products whose name contains this value.
    #[serde(rename = "filter[name]", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Only products of this type.
    #[serde(rename = "filter[type]", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
    /// Only enabled (`true`) or disabled (`false`) products.
    #[serde(rename = "filter[enabled]", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Only products assigned to this category.
    #[serde(rename = "filter[category]", skip_serializing_if = "Option::is_none")]
    pub category: Option<Id>,
    /// Only products with this `exclude_from_automations` value.
    #[serde(
        rename = "filter[exclude_from_automations]",
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_from_automations: Option<bool>,
}

impl ListProducts {
    /// Default query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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

    /// Sort order, e.g. `Sort::desc(ProductSort::CreatedAt)`.
    #[must_use]
    pub fn sort(mut self, sort: Sort<ProductSort>) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Only products whose name contains `name`.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Only products of `product_type`.
    #[must_use]
    pub fn product_type(mut self, product_type: ProductType) -> Self {
        self.product_type = Some(product_type);
        self
    }

    /// Only enabled (`true`) or disabled (`false`) products.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Only products in `category`.
    #[must_use]
    pub fn category(mut self, category: impl Into<Id>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Only products with this `exclude_from_automations` value.
    #[must_use]
    pub fn exclude_from_automations(mut self, exclude: bool) -> Self {
        self.exclude_from_automations = Some(exclude);
        self
    }
}

/// Body for [`Products::create`], [`Products::update`] and one row of
/// [`Imports::products`](super::Imports::products).
///
/// Products of the **MailerLite shop** additionally require [`price`](Self::price),
/// [`currency`](Self::currency) and [`settings`](Self::settings) (with at least `action`) on
/// both create and update. They only allow the `one-time` and `booking` types, can't have
/// categories or variants, and `one-time` products also need downloadable `assets`, which this
/// type can't express yet (use [`MailerLite::request`](crate::MailerLite::request)). See
/// <https://developers.mailerlite.com/ecommerce/products#mailerlite-shop-products> for the
/// settings and booking fields.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CreateProduct {
    /// Product name.
    pub name: String,
    /// Price in the shop's currency. API default `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Keep the product from triggering automations. API default `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_from_automations: Option<bool>,
    /// Whether the product is enabled. API default `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Product page URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Main image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Short description, at most 255 characters (HTML is stripped).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    /// Description (HTML is stripped).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Your own id for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Category ids. They are read as category `resource_id`s whenever the product itself has
    /// a [`resource_id`](Self::resource_id) (even without `with_resource_id`), otherwise as
    /// MailerLite category ids. Not allowed for MailerLite shop products.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<Id>,
    /// MailerLite shop: ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// MailerLite shop: product type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
    /// MailerLite shop: post-purchase action and booking settings, e.g.
    /// `{"action": "move_to_group", "group_id": "1234567890"}`. Merged into the existing
    /// settings on update, but `action` must always be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Value>,
    /// MailerLite shop: discounts. On update, `Some(vec![])` removes every discount and
    /// `None` keeps them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<Discount>>,
}

impl CreateProduct {
    /// A product called `name`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            price: None,
            exclude_from_automations: None,
            enabled: None,
            url: None,
            image: None,
            short_description: None,
            description: None,
            resource_id: None,
            categories: Vec::new(),
            currency: None,
            product_type: None,
            settings: None,
            discounts: None,
        }
    }

    /// Price in the shop's currency.
    #[must_use]
    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// Keep the product from triggering automations.
    #[must_use]
    pub fn exclude_from_automations(mut self, exclude: bool) -> Self {
        self.exclude_from_automations = Some(exclude);
        self
    }

    /// Enable or disable the product.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Product page URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Main image URL.
    #[must_use]
    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }

    /// Short description.
    #[must_use]
    pub fn short_description(mut self, short_description: impl Into<String>) -> Self {
        self.short_description = Some(short_description.into());
        self
    }

    /// Description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Your own id for the product.
    #[must_use]
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    /// Categories of the product (used by imports).
    #[must_use]
    pub fn categories<I: Into<Id>>(mut self, categories: impl IntoIterator<Item = I>) -> Self {
        self.categories = categories.into_iter().map(Into::into).collect();
        self
    }

    /// MailerLite shop: ISO 4217 currency code.
    #[must_use]
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// MailerLite shop: product type.
    #[must_use]
    pub fn product_type(mut self, product_type: ProductType) -> Self {
        self.product_type = Some(product_type);
        self
    }

    /// MailerLite shop: post-purchase action and booking settings.
    #[must_use]
    pub fn settings(mut self, settings: Value) -> Self {
        self.settings = Some(settings);
        self
    }

    /// MailerLite shop: discounts (replaces all existing discounts on update).
    #[must_use]
    pub fn discounts(mut self, discounts: impl IntoIterator<Item = Discount>) -> Self {
        self.discounts = Some(discounts.into_iter().collect());
        self
    }
}

/// A discount to set on a MailerLite shop product, for [`CreateProduct::discounts`].
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[non_exhaustive]
pub struct Discount {
    /// Discount name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Amount, or percentage with [`is_percentage`](Self::is_percentage).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Whether `value` is a percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_percentage: Option<bool>,
    /// Whether the discount is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Start, `Y-m-d H:i:s`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// End, `Y-m-d H:i:s`, after `start_date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl Discount {
    /// An empty discount.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Discount name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// A fixed amount off.
    #[must_use]
    pub fn amount(mut self, amount: f64) -> Self {
        self.value = Some(amount);
        self.is_percentage = Some(false);
        self
    }

    /// A percentage off.
    #[must_use]
    pub fn percentage(mut self, percentage: f64) -> Self {
        self.value = Some(percentage);
        self.is_percentage = Some(true);
        self
    }

    /// Enable or disable the discount.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Start, `Y-m-d H:i:s`.
    #[must_use]
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    /// End, `Y-m-d H:i:s`.
    #[must_use]
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
}

/// Body for [`Products::assign_categories`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct AssignCategories {
    /// Category ids (at most 100).
    pub categories: Vec<Id>,
    /// If `true`, the product is removed from categories not listed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
}

impl AssignCategories {
    /// Adds the product to `categories` (at most 100), keeping its other categories.
    pub fn new<I: Into<Id>>(categories: impl IntoIterator<Item = I>) -> Self {
        Self {
            categories: categories.into_iter().map(Into::into).collect(),
            replace: None,
        }
    }

    /// If `true`, also removes the product from every category not listed.
    #[must_use]
    pub fn replace(mut self, replace: bool) -> Self {
        self.replace = Some(replace);
        self
    }
}

#[derive(Serialize)]
struct StatusBody {
    status: bool,
}

/// Product endpoints of a shop. Obtained with
/// [`ShopScope::products`](super::ShopScope::products).
#[derive(Clone, Copy, Debug)]
pub struct Products<'a> {
    pub(super) scope: Scope<'a>,
}

impl<'a> Products<'a> {
    /// `GET /ecommerce/shops/{shop_id}/products`: one page of products.
    pub async fn list(self, query: &ListProducts) -> Result<Page<Product>> {
        self.scope
            .call(Method::GET, &["products"])
            .query(query)
            .send()
            .await
    }

    /// Every product matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        self,
        query: ListProducts,
    ) -> impl futures_util::Stream<Item = Result<Product>> + 'a {
        let products = self;
        crate::pagination::stream::paged(
            query,
            move |query| async move { products.list(&query).await },
        )
    }

    /// `GET /ecommerce/shops/{shop_id}/products?limit=0`: number of products.
    pub async fn count(self) -> Result<u64> {
        self.scope.count(&["products"]).await
    }

    /// `GET /ecommerce/shops/{shop_id}/products/{id}`: one product.
    pub async fn get(self, id: impl AsRef<str>) -> Result<Product> {
        self.scope
            .call(Method::GET, &["products", id.as_ref()])
            .data()
            .await
    }

    /// `POST /ecommerce/shops/{shop_id}/products`: creates a product.
    pub async fn create(self, product: &CreateProduct) -> Result<Product> {
        self.scope
            .call(Method::POST, &["products"])
            .json(product)
            .data()
            .await
    }

    /// `PUT /ecommerce/shops/{shop_id}/products/{id}`: updates a product.
    pub async fn update(self, id: impl AsRef<str>, product: &CreateProduct) -> Result<Product> {
        self.scope
            .call(Method::PUT, &["products", id.as_ref()])
            .json(product)
            .data()
            .await
    }

    /// `PUT /ecommerce/shops/{shop_id}/products/{id}/change-status`: enables (`true`) or
    /// disables (`false`) a product.
    pub async fn change_status(self, id: impl AsRef<str>, enabled: bool) -> Result<Product> {
        self.scope
            .call(Method::PUT, &["products", id.as_ref(), "change-status"])
            .json(&StatusBody { status: enabled })
            .data()
            .await
    }

    /// `DELETE /ecommerce/shops/{shop_id}/products/{id}`: deletes a product and returns it.
    pub async fn delete(self, id: impl AsRef<str>) -> Result<Product> {
        self.scope
            .call(Method::DELETE, &["products", id.as_ref()])
            .data()
            .await
    }

    /// `PUT /ecommerce/shops/{shop_id}/products/{id}/categories/multiple`: adds the product
    /// to several categories at once (see [`AssignCategories::replace`]).
    pub async fn assign_categories(
        self,
        id: impl AsRef<str>,
        categories: &AssignCategories,
    ) -> Result<Product> {
        self.scope
            .call(
                Method::PUT,
                &["products", id.as_ref(), "categories", "multiple"],
            )
            .json(categories)
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
    async fn list_with_filters_and_count() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_products_list"))
            .respond(200, json!({"total": 123}));
        let shop = client.ecommerce().shop("9");

        let page = shop
            .products()
            .list(
                &ListProducts::new()
                    .limit(20)
                    .page(3)
                    .sort(Sort::asc(ProductSort::Name))
                    .name("shirt")
                    .product_type(ProductType::OneTime)
                    .enabled(true)
                    .category("1234566457534276")
                    .exclude_from_automations(false),
            )
            .await
            .unwrap();
        let total = shop.products().count().await.unwrap();

        let product = &page.data[0];
        assert_eq!(product.id, "31986843543567834");
        assert!((product.price - 123.4).abs() < f64::EPSILON);
        assert_eq!(
            product.short_description.as_deref(),
            Some("Product's short description.")
        );
        assert_eq!(product.categories[1].name, "Category 2");
        assert_eq!(product.variants, Vec::new());
        assert_eq!(product.product_type, None);
        assert_eq!(total, 123);
        let requests = mock.requests();
        requests[0].assert(
            Method::GET,
            "ecommerce/shops/9/products?limit=20&page=3&sort=name&filter%5Bname%5D=shirt\
             &filter%5Btype%5D=one-time&filter%5Benabled%5D=true\
             &filter%5Bcategory%5D=1234566457534276&filter%5Bexclude_from_automations%5D=false",
        );
        requests[1].assert(Method::GET, "ecommerce/shops/9/products?limit=0");
    }

    #[tokio::test]
    async fn get_create_update_change_status_delete() {
        let (client, mock) = client();
        for _ in 0..5 {
            mock.respond(200, fixture("ecommerce_product"));
        }
        let shop = client.ecommerce().shop("9");
        let products = shop.products();

        let fetched = products.get("4").await.unwrap();
        products
            .create(
                &CreateProduct::new("Product name")
                    .price(123.4)
                    .url("https://shop-url.com/product-name")
                    .image("https://shop-url.com/product-image.jpg")
                    .exclude_from_automations(false)
                    .short_description("Product's short description.")
                    .description("Product's longer description."),
            )
            .await
            .unwrap();
        products
            .update(
                "4",
                &CreateProduct::new("Product name")
                    .enabled(false)
                    .resource_id("prod-1"),
            )
            .await
            .unwrap();
        products.change_status("4", false).await.unwrap();
        let deleted = products.delete("4").await.unwrap();

        assert_eq!(fetched, deleted);
        let requests = mock.requests();
        requests[0]
            .assert(Method::GET, "ecommerce/shops/9/products/4")
            .no_body();
        requests[1]
            .assert(Method::POST, "ecommerce/shops/9/products")
            .body(&json!({
                "name": "Product name",
                "price": 123.4,
                "url": "https://shop-url.com/product-name",
                "image": "https://shop-url.com/product-image.jpg",
                "exclude_from_automations": false,
                "short_description": "Product's short description.",
                "description": "Product's longer description."
            }));
        requests[2]
            .assert(Method::PUT, "ecommerce/shops/9/products/4")
            .body(&json!({"name": "Product name", "enabled": false, "resource_id": "prod-1"}));
        requests[3]
            .assert(Method::PUT, "ecommerce/shops/9/products/4/change-status")
            .body(&json!({"status": false}));
        requests[4]
            .assert(Method::DELETE, "ecommerce/shops/9/products/4")
            .no_body();
    }

    #[tokio::test]
    async fn mailerlite_shop_product() {
        let (client, mock) = client();
        mock.respond(201, fixture("ecommerce_product_mailerlite_shop"));

        let product = client
            .ecommerce()
            .shop("1")
            .products()
            .create(
                &CreateProduct::new("Product name")
                    .price(20.0)
                    .currency("USD")
                    .product_type(ProductType::OneTime)
                    .settings(json!({"action": "move_to_group", "group_id": "1234567890"}))
                    .discounts([Discount::new()
                        .name("Launch discount")
                        .percentage(10.0)
                        .enabled(true)
                        .start_date("2026-10-01 00:00:00")
                        .end_date("2026-10-31 23:59:59")]),
            )
            .await
            .unwrap();

        mock.request()
            .assert(Method::POST, "ecommerce/shops/1/products")
            .body(&json!({
                "name": "Product name",
                "price": 20.0,
                "currency": "USD",
                "type": "one-time",
                "settings": {"action": "move_to_group", "group_id": "1234567890"},
                "discounts": [{
                    "name": "Launch discount",
                    "value": 10.0,
                    "is_percentage": true,
                    "enabled": true,
                    "start_date": "2026-10-01 00:00:00",
                    "end_date": "2026-10-31 23:59:59"
                }]
            }));
        assert_eq!(product.product_type, Some(ProductType::OneTime));
        assert_eq!(product.currency_symbol.as_deref(), Some("$"));
        assert!((product.discounts[0].value - 10.0).abs() < f64::EPSILON);
        assert_eq!(product.discounts[0].price_after_discount, Some(18.0));
        assert_eq!(product.settings["action"], "move_to_group");
        assert_eq!(product.has_checkout, Some(true));
        assert_eq!(product.categories, Vec::new());
        assert_eq!(
            product.created_at.as_ref().map(Timestamp::as_str),
            Some("2026-10-01 10:00:00")
        );
    }

    #[tokio::test]
    async fn assign_categories() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_product_categories_assigned"));

        let product = client
            .ecommerce()
            .shop("9")
            .products()
            .assign_categories(
                "4",
                &AssignCategories::new(["1234566457534276", "1234566457534278"]).replace(false),
            )
            .await
            .unwrap();

        mock.request()
            .assert(
                Method::PUT,
                "ecommerce/shops/9/products/4/categories/multiple",
            )
            .body(&json!({
                "categories": ["1234566457534276", "1234566457534278"],
                "replace": false
            }));
        assert_eq!(product.categories.len(), 2);
    }
}
