//! Carts: <https://developers.mailerlite.com/ecommerce/cart> and
//! <https://developers.mailerlite.com/ecommerce/cart-items>

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::{Customer, Product, Scope};
use crate::{
    Page, Result,
    types::{Id, Sort, de, string_enum},
};

/// A cart. Every order has one; carts are created through
/// [`Orders::create`](super::Orders::create) and deleted with their order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Cart {
    /// Cart id.
    pub id: Id,
    /// Your own id for the cart, if any.
    #[serde(default)]
    pub resource_id: Option<String>,
    /// The order the cart belongs to.
    #[serde(default)]
    pub ecommerce_order_id: Option<Id>,
    /// Checkout URL.
    #[serde(default)]
    pub checkout_url: Option<String>,
    /// Cart total in the shop's currency.
    #[serde(default, deserialize_with = "de::number")]
    pub cart_total: f64,
    /// The customer the cart belongs to (not included when the cart is embedded in an
    /// order).
    #[serde(default)]
    pub customer: Option<Customer>,
    /// Items in the cart.
    #[serde(default, deserialize_with = "de::null_default")]
    pub items: Vec<CartItem>,
}

/// An item in a [`Cart`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CartItem {
    /// Cart item id.
    pub id: Id,
    /// Your own id for the item, if any.
    #[serde(default)]
    pub resource_id: Option<String>,
    /// The product.
    pub product: Product,
    /// The [`Variant`](super::Variant) id, if one was set.
    #[serde(default)]
    pub ecommerce_product_variant_id: Option<Id>,
    /// Free-form variant description, e.g. `"Color black, size medium"`.
    #[serde(default)]
    pub variant: Option<String>,
    /// Quantity.
    #[serde(default)]
    pub quantity: u32,
    /// Item price in the shop's currency.
    #[serde(default, deserialize_with = "de::number")]
    pub price: f64,
}

string_enum! {
    /// Sort field for [`ListCarts`] and [`ListCartItems`].
    pub enum CartSort {
        /// Id (API default).
        Id => "id",
        /// Creation time.
        CreatedAt => "created_at",
    }
}

/// Query for [`Carts::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListCarts {
    /// Page size (API default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort<CartSort>>,
}

impl ListCarts {
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

    /// Sort order, e.g. `Sort::desc(CartSort::CreatedAt)`.
    #[must_use]
    pub fn sort(mut self, sort: Sort<CartSort>) -> Self {
        self.sort = Some(sort);
        self
    }
}

/// Query for [`CartItems::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListCartItems {
    /// Page size (API default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort<CartSort>>,
}

impl ListCartItems {
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

    /// Sort order, e.g. `Sort::desc(CartSort::CreatedAt)`.
    #[must_use]
    pub fn sort(mut self, sort: Sort<CartSort>) -> Self {
        self.sort = Some(sort);
        self
    }
}

/// Body for [`Carts::update`].
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UpdateCart {
    /// Checkout URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
    /// Cart total in the shop's currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_total: Option<f64>,
    /// The customer (of the same shop) the cart belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_customer_id: Option<Id>,
    /// Your own id for the cart, unique within the shop. If another cart already uses it,
    /// nothing is saved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl UpdateCart {
    /// An empty update.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checkout URL.
    #[must_use]
    pub fn checkout_url(mut self, checkout_url: impl Into<String>) -> Self {
        self.checkout_url = Some(checkout_url.into());
        self
    }

    /// Cart total.
    #[must_use]
    pub fn cart_total(mut self, cart_total: f64) -> Self {
        self.cart_total = Some(cart_total);
        self
    }

    /// The customer the cart belongs to.
    #[must_use]
    pub fn ecommerce_customer_id(mut self, customer_id: impl Into<Id>) -> Self {
        self.ecommerce_customer_id = Some(customer_id.into());
        self
    }

    /// Your own id for the cart.
    #[must_use]
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }
}

/// Body for [`CartItems::create`] / [`CartItems::update`], and one item of an
/// [`OrderCart`](super::OrderCart).
///
/// Reference the product either by id ([`new`](Self::new), sent as `ecommerce_product_id`)
/// or by your own product id ([`product_resource_id`](Self::product_resource_id), sent as
/// `product_resource_id`, as in MailerLite's `with_resource_id` guide).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CreateCartItem {
    /// Product id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_product_id: Option<Id>,
    /// Your own product id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_resource_id: Option<String>,
    /// [`Variant`](super::Variant) id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_product_variant_id: Option<Id>,
    /// Free-form variant description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// Quantity. API default `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    /// Item price. API default `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Your own id for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl CreateCartItem {
    /// An item for the product `product_id` (`ecommerce_product_id`).
    pub fn new(product_id: impl Into<Id>) -> Self {
        Self {
            ecommerce_product_id: Some(product_id.into()),
            ..Self::empty()
        }
    }

    /// An item for the product with your own id `resource_id` (`product_resource_id`).
    pub fn product_resource_id(resource_id: impl Into<String>) -> Self {
        Self {
            product_resource_id: Some(resource_id.into()),
            ..Self::empty()
        }
    }

    fn empty() -> Self {
        Self {
            ecommerce_product_id: None,
            product_resource_id: None,
            ecommerce_product_variant_id: None,
            variant: None,
            quantity: None,
            price: None,
            resource_id: None,
        }
    }

    /// [`Variant`](super::Variant) id.
    #[must_use]
    pub fn variant_id(mut self, variant_id: impl Into<Id>) -> Self {
        self.ecommerce_product_variant_id = Some(variant_id.into());
        self
    }

    /// Free-form variant description, e.g. `"Color black, size medium"`.
    #[must_use]
    pub fn variant(mut self, variant: impl Into<String>) -> Self {
        self.variant = Some(variant.into());
        self
    }

    /// Quantity.
    #[must_use]
    pub fn quantity(mut self, quantity: u32) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /// Item price.
    #[must_use]
    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// Your own id for the item.
    #[must_use]
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }
}

/// Cart endpoints of a shop. Obtained with [`ShopScope::carts`](super::ShopScope::carts).
#[derive(Clone, Copy, Debug)]
pub struct Carts<'a> {
    pub(super) scope: Scope<'a>,
}

impl<'a> Carts<'a> {
    /// `GET /ecommerce/shops/{shop_id}/carts`: one page of carts.
    pub async fn list(&self, query: &ListCarts) -> Result<Page<Cart>> {
        self.scope
            .call(Method::GET, &["carts"])
            .query(query)
            .send()
            .await
    }

    /// Every cart matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        &self,
        query: ListCarts,
    ) -> impl futures_util::Stream<Item = Result<Cart>> + 'a {
        let carts = *self;
        crate::pagination::stream::pages(query.page.unwrap_or(1), move |page| {
            let query = ListCarts {
                page: Some(page),
                ..query.clone()
            };
            async move { carts.list(&query).await }
        })
    }

    /// `GET /ecommerce/shops/{shop_id}/carts?limit=0`: number of carts.
    pub async fn count(&self) -> Result<u64> {
        self.scope.count(&["carts"]).await
    }

    /// `GET /ecommerce/shops/{shop_id}/carts/{id}`: one cart.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Cart> {
        self.scope
            .call(Method::GET, &["carts", id.as_ref()])
            .data()
            .await
    }

    /// `PUT /ecommerce/shops/{shop_id}/carts/{id}`: updates a cart. Change its items with
    /// [`ShopScope::cart_items`](super::ShopScope::cart_items).
    pub async fn update(&self, id: impl AsRef<str>, cart: &UpdateCart) -> Result<Cart> {
        self.scope
            .call(Method::PUT, &["carts", id.as_ref()])
            .json(cart)
            .data()
            .await
    }
}

/// Item endpoints of one cart. Obtained with
/// [`ShopScope::cart_items`](super::ShopScope::cart_items).
#[derive(Clone, Copy, Debug)]
pub struct CartItems<'a> {
    pub(super) scope: Scope<'a>,
    pub(super) cart_id: &'a str,
}

impl<'a> CartItems<'a> {
    /// `GET /ecommerce/shops/{shop_id}/carts/{cart_id}/items`: one page of items.
    pub async fn list(&self, query: &ListCartItems) -> Result<Page<CartItem>> {
        self.scope
            .call(Method::GET, &["carts", self.cart_id, "items"])
            .query(query)
            .send()
            .await
    }

    /// Every item of the cart, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        &self,
        query: ListCartItems,
    ) -> impl futures_util::Stream<Item = Result<CartItem>> + 'a {
        let items = *self;
        crate::pagination::stream::pages(query.page.unwrap_or(1), move |page| {
            let query = ListCartItems {
                page: Some(page),
                ..query.clone()
            };
            async move { items.list(&query).await }
        })
    }

    /// `GET /ecommerce/shops/{shop_id}/carts/{cart_id}/items?limit=0`: number of items.
    pub async fn count(&self) -> Result<u64> {
        self.scope.count(&["carts", self.cart_id, "items"]).await
    }

    /// `GET /ecommerce/shops/{shop_id}/carts/{cart_id}/items/{id}`: one item.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<CartItem> {
        self.scope
            .call(Method::GET, &["carts", self.cart_id, "items", id.as_ref()])
            .data()
            .await
    }

    /// `POST /ecommerce/shops/{shop_id}/carts/{cart_id}/items`: adds an item to the cart.
    pub async fn create(&self, item: &CreateCartItem) -> Result<CartItem> {
        self.scope
            .call(Method::POST, &["carts", self.cart_id, "items"])
            .json(item)
            .data()
            .await
    }

    /// `PUT /ecommerce/shops/{shop_id}/carts/{cart_id}/items/{id}`: updates an item.
    pub async fn update(&self, id: impl AsRef<str>, item: &CreateCartItem) -> Result<CartItem> {
        self.scope
            .call(Method::PUT, &["carts", self.cart_id, "items", id.as_ref()])
            .json(item)
            .data()
            .await
    }

    /// `DELETE /ecommerce/shops/{shop_id}/carts/{cart_id}/items/{id}`: removes an item and
    /// returns it.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<CartItem> {
        self.scope
            .call(
                Method::DELETE,
                &["carts", self.cart_id, "items", id.as_ref()],
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
    async fn carts_list_count_get_update() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_carts_list"))
            .respond(200, json!({"total": 123}))
            .respond(200, fixture("ecommerce_cart"))
            .respond(200, fixture("ecommerce_cart"));
        let shop = client.ecommerce().shop("9");
        let carts = shop.carts();

        let page = carts
            .list(
                &ListCarts::new()
                    .limit(2)
                    .page(1)
                    .sort(Sort::desc(CartSort::CreatedAt)),
            )
            .await
            .unwrap();
        let total = carts.count().await.unwrap();
        let fetched = carts.get("12564435675647").await.unwrap();
        carts
            .update(
                "12564435675647",
                &UpdateCart::new()
                    .checkout_url("https://url-to-cart-checkout.com")
                    .cart_total(1234.99)
                    .ecommerce_customer_id("319868430643456756")
                    .resource_id("cart-1"),
            )
            .await
            .unwrap();

        assert_eq!(page.data[0], fetched);
        assert_eq!(fetched.resource_id.as_deref(), Some("cart-1"));
        assert_eq!(fetched.ecommerce_order_id.as_deref(), Some("3456678978907"));
        assert!((fetched.cart_total - 1234.99).abs() < f64::EPSILON);
        assert_eq!(
            fetched.customer.as_ref().unwrap().email,
            "customer@email.com"
        );
        assert_eq!(fetched.items[0].variant.as_deref(), Some("product variant"));
        assert_eq!(total, 123);
        let requests = mock.requests();
        requests[0].assert(
            Method::GET,
            "ecommerce/shops/9/carts?limit=2&page=1&sort=-created_at",
        );
        requests[1].assert(Method::GET, "ecommerce/shops/9/carts?limit=0");
        requests[2]
            .assert(Method::GET, "ecommerce/shops/9/carts/12564435675647")
            .no_body();
        requests[3]
            .assert(Method::PUT, "ecommerce/shops/9/carts/12564435675647")
            .body(&json!({
                "checkout_url": "https://url-to-cart-checkout.com",
                "cart_total": 1234.99,
                "ecommerce_customer_id": "319868430643456756",
                "resource_id": "cart-1"
            }));
    }

    #[tokio::test]
    async fn cart_items_crud() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_cart_items_list"))
            .respond(200, json!({"total": 123}))
            .respond(200, fixture("ecommerce_cart_item"))
            .respond(201, fixture("ecommerce_cart_item"))
            .respond(200, fixture("ecommerce_cart_item"))
            .respond(200, fixture("ecommerce_cart_item"));
        let shop = client.ecommerce().shop("9");
        let items = shop.cart_items("7");

        let page = items
            .list(&ListCartItems::new().limit(10).sort(Sort::asc(CartSort::Id)))
            .await
            .unwrap();
        let total = items.count().await.unwrap();
        let fetched = items.get("8").await.unwrap();
        items
            .create(
                &CreateCartItem::new("64375689754")
                    .price(123.4)
                    .variant_id("31986843543567900")
                    .variant("Color black, size medium")
                    .quantity(2),
            )
            .await
            .unwrap();
        items
            .update("8", &CreateCartItem::new("64375689754").quantity(3))
            .await
            .unwrap();
        let deleted = items.delete("8").await.unwrap();

        assert_eq!(page.data[0].ecommerce_product_variant_id, None);
        assert_eq!(page.data[0].product.categories.len(), 2);
        assert_eq!(total, 123);
        assert_eq!(fetched.quantity, 2);
        assert_eq!(fetched, deleted);
        let requests = mock.requests();
        requests[0].assert(
            Method::GET,
            "ecommerce/shops/9/carts/7/items?limit=10&sort=id",
        );
        requests[1].assert(Method::GET, "ecommerce/shops/9/carts/7/items?limit=0");
        requests[2]
            .assert(Method::GET, "ecommerce/shops/9/carts/7/items/8")
            .no_body();
        requests[3]
            .assert(Method::POST, "ecommerce/shops/9/carts/7/items")
            .body(&json!({
                "ecommerce_product_id": "64375689754",
                "price": 123.4,
                "ecommerce_product_variant_id": "31986843543567900",
                "variant": "Color black, size medium",
                "quantity": 2
            }));
        requests[4]
            .assert(Method::PUT, "ecommerce/shops/9/carts/7/items/8")
            .body(&json!({"ecommerce_product_id": "64375689754", "quantity": 3}));
        requests[5]
            .assert(Method::DELETE, "ecommerce/shops/9/carts/7/items/8")
            .no_body();
    }

    #[tokio::test]
    async fn cart_item_with_resource_ids() {
        let (client, mock) = client();
        mock.respond(201, fixture("ecommerce_cart_item"));
        let shop = client.ecommerce().shop("134").with_resource_id();

        shop.cart_items("cart1")
            .create(
                &CreateCartItem::product_resource_id("prod-tshirt2")
                    .resource_id("cart-item2")
                    .quantity(1)
                    .price(20.0),
            )
            .await
            .unwrap();

        mock.request()
            .assert(
                Method::POST,
                "ecommerce/shops/134/carts/cart1/items?with_resource_id=",
            )
            .body(&json!({
                "resource_id": "cart-item2",
                "product_resource_id": "prod-tshirt2",
                "quantity": 1,
                "price": 20.0
            }));
    }
}
