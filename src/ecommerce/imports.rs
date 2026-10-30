//! Bulk imports: <https://developers.mailerlite.com/ecommerce/import>

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::{
    Category, CreateCategory, CreateCustomer, CreateOrder, CreateProduct, Order, Product, Scope,
};
use crate::{Result, types::de};

/// A customer accepted by [`Imports::customers`]. Customers are saved in the background, so
/// the response only echoes the accepted data.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportedCustomer {
    /// Your own id for the customer, if sent.
    #[serde(default)]
    pub resource_id: Option<String>,
    /// Email address.
    pub email: String,
    /// Whether the customer accepts marketing emails.
    #[serde(default, deserialize_with = "de::null_default")]
    pub accepts_marketing: bool,
    /// Whether a subscriber is created for the customer.
    #[serde(default, deserialize_with = "de::null_default")]
    pub create_subscriber: bool,
}

/// Bulk import endpoints of a shop. Obtained with
/// [`ShopScope::imports`](super::ShopScope::imports).
///
/// With [`ShopScope::with_resource_id`](super::ShopScope::with_resource_id), references
/// inside the rows (e.g. a product's categories) are your own `resource_id`s.
#[derive(Clone, Copy, Debug)]
pub struct Imports<'a> {
    pub(super) scope: Scope<'a>,
}

impl Imports<'_> {
    /// `POST /ecommerce/shops/{shop_id}/categories/import`: creates several categories.
    pub async fn categories(&self, categories: &[CreateCategory]) -> Result<Vec<Category>> {
        self.scope
            .call(Method::POST, &["categories", "import"])
            .json(categories)
            .data()
            .await
    }

    /// `POST /ecommerce/shops/{shop_id}/customers/import`: queues several customers for
    /// import in the background. Fails with `413` if they would exceed the account's
    /// subscriber limit.
    pub async fn customers(&self, customers: &[CreateCustomer]) -> Result<Vec<ImportedCustomer>> {
        self.scope
            .call(Method::POST, &["customers", "import"])
            .json(customers)
            .data()
            .await
    }

    /// `POST /ecommerce/shops/{shop_id}/products/import`: creates several products, with
    /// [`CreateProduct::categories`].
    pub async fn products(&self, products: &[CreateProduct]) -> Result<Vec<Product>> {
        self.scope
            .call(Method::POST, &["products", "import"])
            .json(products)
            .data()
            .await
    }

    /// `POST /ecommerce/shops/{shop_id}/orders/import`: creates several orders with their
    /// customers, carts and cart items.
    pub async fn orders(&self, orders: &[CreateOrder]) -> Result<Vec<Order>> {
        self.import_orders(orders, None).await
    }

    /// Like [`orders`](Self::orders), but records them without triggering automations, e.g.
    /// when backfilling historical orders.
    pub async fn orders_without_automations(&self, orders: &[CreateOrder]) -> Result<Vec<Order>> {
        self.import_orders(orders, Some(true)).await
    }

    async fn import_orders(
        &self,
        orders: &[CreateOrder],
        without_automations: Option<bool>,
    ) -> Result<Vec<Order>> {
        #[derive(Serialize)]
        struct Body<'a> {
            orders: &'a [CreateOrder],
            #[serde(skip_serializing_if = "Option::is_none")]
            without_automations: Option<bool>,
        }

        self.scope
            .call(Method::POST, &["orders", "import"])
            .json(&Body {
                orders,
                without_automations,
            })
            .data()
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::json;

    use super::*;
    use crate::{
        ecommerce::{CreateCartItem, OrderCart, OrderStatus},
        http::mock::{client, fixture},
    };

    #[tokio::test]
    async fn categories() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_import_categories"));
        let shop = client.ecommerce().shop("9");

        let categories = shop
            .imports()
            .categories(&[
                CreateCategory::new("Category 1").exclude_from_automations(false),
                CreateCategory::new("Category 2").exclude_from_automations(false),
            ])
            .await
            .unwrap();

        mock.request()
            .assert(Method::POST, "ecommerce/shops/9/categories/import")
            .body(&json!([
                {"name": "Category 1", "exclude_from_automations": false},
                {"name": "Category 2", "exclude_from_automations": false}
            ]));
        assert_eq!(categories[0].id, "46919150796276752");
        assert_eq!(categories[1].name, "Category 2");
    }

    #[tokio::test]
    async fn customers() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_import_customers"));
        let shop = client.ecommerce().shop("9");

        let customers = shop
            .imports()
            .customers(&[
                CreateCustomer::new("customer@email.com")
                    .resource_id("1001")
                    .accepts_marketing(true)
                    .create_subscriber(true)
                    .total_spent(149.99)
                    .orders_count(3),
                CreateCustomer::new("customer2@email.com")
                    .resource_id("1002")
                    .accepts_marketing(false)
                    .create_subscriber(false),
            ])
            .await
            .unwrap();

        mock.request()
            .assert(Method::POST, "ecommerce/shops/9/customers/import")
            .body(&json!([
                {
                    "email": "customer@email.com",
                    "resource_id": "1001",
                    "accepts_marketing": true,
                    "create_subscriber": true,
                    "total_spent": 149.99,
                    "orders_count": 3
                },
                {
                    "email": "customer2@email.com",
                    "resource_id": "1002",
                    "accepts_marketing": false,
                    "create_subscriber": false
                }
            ]));
        assert_eq!(customers[0].resource_id.as_deref(), Some("1001"));
        assert!(customers[0].create_subscriber);
        assert!(!customers[1].accepts_marketing);
    }

    #[tokio::test]
    async fn products() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_import_products"));
        let shop = client.ecommerce().shop("9");

        let products = shop
            .imports()
            .products(&[CreateProduct::new("Product name")
                .price(123.4)
                .url("https://shop-url.com/product-name")
                .image("https://shop-url.com/product-image.jpg")
                .exclude_from_automations(false)
                .categories([46_919_150_796_276_752_u64, 46_919_150_798_373_905])])
            .await
            .unwrap();

        mock.request()
            .assert(Method::POST, "ecommerce/shops/9/products/import")
            .body(&json!([{
                "name": "Product name",
                "price": 123.4,
                "url": "https://shop-url.com/product-name",
                "image": "https://shop-url.com/product-image.jpg",
                "exclude_from_automations": false,
                "categories": ["46919150796276752", "46919150798373905"]
            }]));
        assert_eq!(products[0].id, "46920297804203043");
        assert_eq!(products[1].categories[1].id, "46919150798373905");
    }

    #[tokio::test]
    async fn products_with_resource_ids() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_guide_import_products"));
        let shop = client.ecommerce().shop("134").with_resource_id();

        let products = shop
            .imports()
            .products(&[CreateProduct::new("T-Shirt 1")
                .resource_id("prod-tshirt1")
                .categories(["cat-superdeals", "cat-pattern"])
                .price(20.0)
                .exclude_from_automations(false)])
            .await
            .unwrap();

        mock.request()
            .assert(
                Method::POST,
                "ecommerce/shops/134/products/import?with_resource_id=",
            )
            .body(&json!([{
                "resource_id": "prod-tshirt1",
                "name": "T-Shirt 1",
                "categories": ["cat-superdeals", "cat-pattern"],
                "price": 20.0,
                "exclude_from_automations": false
            }]));
        assert_eq!(products[0].resource_id.as_deref(), Some("prod-tshirt1"));
        assert_eq!(
            products[0].categories[0].resource_id.as_deref(),
            Some("cat-superdeals")
        );
    }

    #[tokio::test]
    async fn orders() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_import_orders"));
        let shop = client.ecommerce().shop("9");

        let orders = shop
            .imports()
            .orders(&[CreateOrder::new(
                CreateCustomer::new("customer2@email.com")
                    .create_subscriber(true)
                    .accepts_marketing(true),
                OrderCart::new([CreateCartItem::new("46920297804203043")
                    .variant("product variant")
                    .quantity(1)
                    .price(123.4)]),
            )
            .status(OrderStatus::Pending)
            .total_price(123.4)])
            .await
            .unwrap();

        mock.request()
            .assert(Method::POST, "ecommerce/shops/9/orders/import")
            // Unlike the other imports, orders are wrapped in an `orders` key.
            .body(&json!({"orders": [{
                "customer": {
                    "email": "customer2@email.com",
                    "create_subscriber": true,
                    "accepts_marketing": true
                },
                "cart": {
                    "items": [{
                        "ecommerce_product_id": "46920297804203043",
                        "variant": "product variant",
                        "quantity": 1,
                        "price": 123.4
                    }]
                },
                "status": "pending",
                "total_price": 123.4
            }]}));
        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0].id, "46920756608631854");
        assert_eq!(orders[0].customer.id, "46920756260504617");
        assert_eq!(orders[0].customer.total_spent, None);
        assert!((orders[0].cart.items[0].price - 123.4).abs() < f64::EPSILON);
        assert!((orders[0].cart.items[0].product.price - 123.4).abs() < f64::EPSILON);
    }
}
