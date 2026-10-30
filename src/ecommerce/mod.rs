//! E-commerce: <https://developers.mailerlite.com/ecommerce/shops>
//!
//! Connect an external shop so MailerLite can run e-commerce automations (abandoned cart,
//! post-purchase) and attribute revenue to campaigns.
//!
//! * Account-level endpoints hang off [`MailerLite::ecommerce`]:
//!   [`shops`](Ecommerce::shops) and [`settings`](Ecommerce::settings).
//! * Everything that lives inside a shop goes through a [`ShopScope`] from
//!   [`Ecommerce::shop`], which stores the shop id once:
//!   [`products`](ShopScope::products), [`variants`](ShopScope::variants),
//!   [`categories`](ShopScope::categories), [`customers`](ShopScope::customers),
//!   [`orders`](ShopScope::orders), [`carts`](ShopScope::carts),
//!   [`cart_items`](ShopScope::cart_items) and [`imports`](ShopScope::imports).
//!
//! # Your own ids (`with_resource_id`)
//!
//! Most resources accept a `resource_id`: an id you choose (e.g. your platform's product id).
//! Call [`ShopScope::with_resource_id`] and every request through that scope carries the
//! `with_resource_id` flag. In that mode every id *below* the shop is one of your
//! `resource_id`s: ids in the path and references in request bodies (product categories, cart
//! item products, ...). The shop itself is still addressed by its MailerLite id, and variant
//! endpoints ([`ShopScope::variants`]) and id filters such as `filter[category]` or
//! `filter[customer]` always take MailerLite ids.
//!
//! ```no_run
//! # async fn run(client: mailerlite_rs::MailerLite) -> mailerlite_rs::Result<()> {
//! use mailerlite_rs::ecommerce::{CreateCartItem, CreateCustomer, CreateOrder, OrderCart};
//!
//! let shop = client.ecommerce().shop("134").with_resource_id();
//! let order = shop
//!     .orders()
//!     .create(
//!         &CreateOrder::new(
//!             CreateCustomer::new("customer@email.com").resource_id("customer1"),
//!             OrderCart::new([CreateCartItem::product_resource_id("prod-tshirt1")
//!                 .resource_id("cart-item1")
//!                 .price(20.0)])
//!             .resource_id("cart1"),
//!         )
//!         .resource_id("new-order1"),
//!     )
//!     .await?;
//! let category = shop.categories().get("cat-superdeals").await?;
//! # Ok(()) }
//! ```

use reqwest::Method;
use serde::Serialize;

use crate::{
    MailerLite, RawRequest, Result,
    types::{COUNT_QUERY, Id, Total},
};

mod carts;
mod categories;
mod customers;
mod imports;
mod orders;
mod products;
mod settings;
mod shops;
mod variants;

pub use carts::{
    Cart, CartItem, CartItems, CartSort, Carts, CreateCartItem, ListCartItems, ListCarts,
    UpdateCart,
};
pub use categories::{
    Categories, Category, CategorySort, CreateCategory, ListCategories, ListCategoryProducts,
};
pub use customers::{CreateCustomer, Customer, CustomerSort, Customers, ListCustomers};
pub use imports::{ImportedCustomer, Imports};
pub use orders::{
    Attribution, AttributionType, CreateOrder, ListOrders, Order, OrderCart, OrderSort,
    OrderStatus, Orders, UpdateOrder,
};
pub use products::{
    AssignCategories, CreateProduct, Discount, ListProducts, Product, ProductDiscount, ProductSort,
    ProductType, Products,
};
pub use settings::{EcommerceSettings, Settings};
pub use shops::{CreateShop, ListShops, Shop, ShopGroup, ShopStats, ShopStatsDay, Shops};
pub use variants::{CreateVariant, PriceType, Variant, Variants};

/// E-commerce endpoints. Obtained with [`MailerLite::ecommerce`].
#[derive(Clone, Copy, Debug)]
pub struct Ecommerce<'a> {
    client: &'a MailerLite,
}

impl MailerLite {
    /// E-commerce endpoints: shops, settings and, through [`Ecommerce::shop`], everything
    /// inside a shop.
    #[must_use]
    pub fn ecommerce(&self) -> Ecommerce<'_> {
        Ecommerce { client: self }
    }
}

impl<'a> Ecommerce<'a> {
    /// Shop endpoints (`/ecommerce/shops`).
    #[must_use]
    pub fn shops(&self) -> Shops<'a> {
        Shops {
            client: self.client,
        }
    }

    /// Account-wide e-commerce tracking settings (`/ecommerce/settings`).
    #[must_use]
    pub fn settings(&self) -> Settings<'a> {
        Settings {
            client: self.client,
        }
    }

    /// Endpoints scoped to one shop (`/ecommerce/shops/{shop_id}/...`).
    #[must_use]
    pub fn shop(&self, shop_id: impl Into<Id>) -> ShopScope<'a> {
        ShopScope {
            client: self.client,
            shop_id: shop_id.into(),
            with_resource_id: false,
        }
    }
}

/// Endpoints scoped to one shop. Obtained with [`Ecommerce::shop`].
///
/// Child handles borrow the scope, so keep the scope in a variable:
///
/// ```no_run
/// # async fn run(client: mailerlite_rs::MailerLite) -> mailerlite_rs::Result<()> {
/// let shop = client.ecommerce().shop("31986843064993537");
/// let products = shop.products().count().await?;
/// let customers = shop.customers().count().await?;
/// # Ok(()) }
/// ```
#[derive(Clone, Debug)]
pub struct ShopScope<'a> {
    client: &'a MailerLite,
    shop_id: Id,
    with_resource_id: bool,
}

impl<'a> ShopScope<'a> {
    /// Switches to your own ids: every request through this scope adds the
    /// `with_resource_id` query flag, so every id in the path below the shop and every id
    /// reference in a request body is read as a `resource_id` you assigned. The shop id
    /// itself stays the MailerLite shop id, as do variant endpoints and list id filters.
    #[must_use]
    pub fn with_resource_id(mut self) -> Self {
        self.with_resource_id = true;
        self
    }

    /// The shop id this scope targets.
    #[must_use]
    pub fn shop_id(&self) -> &Id {
        &self.shop_id
    }

    /// `true` if [`with_resource_id`](Self::with_resource_id) is on.
    #[must_use]
    pub fn uses_resource_ids(&self) -> bool {
        self.with_resource_id
    }

    /// Product endpoints.
    #[must_use]
    pub fn products(&self) -> Products<'_> {
        Products {
            scope: self.scope(),
        }
    }

    /// Variant endpoints of the product `product_id`.
    ///
    /// Variant endpoints only take MailerLite ids (product and variant), even on a
    /// [`with_resource_id`](Self::with_resource_id) scope, and never send the flag: the API
    /// rejects variant syncs that carry it.
    #[must_use]
    pub fn variants<'s>(&'s self, product_id: &'s str) -> Variants<'s> {
        Variants {
            scope: Scope {
                with_resource_id: false,
                ..self.scope()
            },
            product_id,
        }
    }

    /// Category endpoints, including assigning products to categories.
    #[must_use]
    pub fn categories(&self) -> Categories<'_> {
        Categories {
            scope: self.scope(),
        }
    }

    /// Customer endpoints.
    #[must_use]
    pub fn customers(&self) -> Customers<'_> {
        Customers {
            scope: self.scope(),
        }
    }

    /// Order endpoints.
    #[must_use]
    pub fn orders(&self) -> Orders<'_> {
        Orders {
            scope: self.scope(),
        }
    }

    /// Cart endpoints. Carts are created through [`Orders::create`].
    #[must_use]
    pub fn carts(&self) -> Carts<'_> {
        Carts {
            scope: self.scope(),
        }
    }

    /// Item endpoints of the cart `cart_id`.
    #[must_use]
    pub fn cart_items<'s>(&'s self, cart_id: &'s str) -> CartItems<'s> {
        CartItems {
            scope: self.scope(),
            cart_id,
        }
    }

    /// Bulk import endpoints for categories, customers, products and orders.
    #[must_use]
    pub fn imports(&self) -> Imports<'_> {
        Imports {
            scope: self.scope(),
        }
    }

    fn scope(&self) -> Scope<'_> {
        Scope {
            client: self.client,
            shop_id: &self.shop_id,
            with_resource_id: self.with_resource_id,
        }
    }
}

/// The cheap, copyable core every shop-scoped handle carries.
#[derive(Clone, Copy, Debug)]
struct Scope<'a> {
    client: &'a MailerLite,
    shop_id: &'a str,
    with_resource_id: bool,
}

/// `?with_resource_id`. The docs pass the flag bare; `serde_urlencoded` can only emit
/// `key=value`, and PHP parses `with_resource_id=` exactly like a bare `with_resource_id`
/// (both yield an empty string), so the empty value is the equivalent wire form.
#[derive(Serialize)]
struct ResourceIdFlag {
    with_resource_id: &'static str,
}

const RESOURCE_ID_FLAG: ResourceIdFlag = ResourceIdFlag {
    with_resource_id: "",
};

impl<'a> Scope<'a> {
    /// `ecommerce/shops/{shop_id}/{segments...}`, plus the resource id flag when enabled.
    fn call(self, method: Method, segments: &[&str]) -> RawRequest<'a> {
        let mut path = Vec::with_capacity(segments.len() + 3);
        path.extend_from_slice(&["ecommerce", "shops", self.shop_id]);
        path.extend_from_slice(segments);
        let request = self.client.call(method, &path);
        if self.with_resource_id {
            request.query(&RESOURCE_ID_FLAG)
        } else {
            request
        }
    }

    /// `GET ecommerce/shops/{shop_id}/{segments...}?limit=0` → `{"total": n}`.
    async fn count(self, segments: &[&str]) -> Result<u64> {
        let total: Total = self
            .call(Method::GET, segments)
            .query(&COUNT_QUERY)
            .send()
            .await?;
        Ok(total.total)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::json;

    use crate::http::mock::{client, fixture};

    #[tokio::test]
    async fn scope_prefixes_shop_and_adds_resource_id_flag() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_guide_category"))
            .respond(200, json!({"total": 4}));

        let shop = client.ecommerce().shop("134").with_resource_id();
        assert!(shop.uses_resource_ids());
        assert_eq!(shop.shop_id(), "134");
        let category = shop.categories().get("cat-superdeals").await.unwrap();
        let total = shop.categories().count().await.unwrap();

        assert_eq!(category.id, "52014701898368045");
        assert_eq!(category.resource_id.as_deref(), Some("cat-superdeals"));
        assert_eq!(total, 4);
        let requests = mock.requests();
        requests[0]
            .assert(
                Method::GET,
                "ecommerce/shops/134/categories/cat-superdeals?with_resource_id=",
            )
            .no_body();
        requests[1].assert(
            Method::GET,
            "ecommerce/shops/134/categories?with_resource_id=&limit=0",
        );
    }

    #[tokio::test]
    async fn path_segments_are_encoded() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_category"));

        client
            .ecommerce()
            .shop("my shop")
            .categories()
            .get("a/b")
            .await
            .unwrap();

        mock.request()
            .assert(Method::GET, "ecommerce/shops/my%20shop/categories/a%2Fb");
    }
}
