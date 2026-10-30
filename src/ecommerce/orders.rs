//! Orders: <https://developers.mailerlite.com/ecommerce/orders>

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::{Cart, CreateCartItem, CreateCustomer, Customer, Scope, Shop};
use crate::{
    Page, Result,
    types::{Id, Sort, Timestamp, de, string_enum},
};

/// An order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Order {
    /// Order id.
    pub id: Id,
    /// Your own id for the order, if any.
    #[serde(default)]
    pub resource_id: Option<String>,
    /// The shop (included by most endpoints).
    #[serde(default)]
    pub shop: Option<Shop>,
    /// The customer.
    pub customer: Customer,
    /// The cart with the ordered items.
    pub cart: Cart,
    /// Order total in the shop's currency.
    #[serde(default, deserialize_with = "de::number")]
    pub total: f64,
    /// Order status. `None` for an order created without an explicit status, which the API
    /// returns as `null` (such orders are treated as `pending`).
    #[serde(default)]
    pub status: Option<OrderStatus>,
    /// The email that earned the order, or `None` if it is not attributed.
    #[serde(default)]
    pub attribution: Option<Attribution>,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
    /// When the order was first marked complete.
    #[serde(default)]
    pub completed_at: Option<Timestamp>,
}

string_enum! {
    /// Status of an [`Order`].
    pub enum OrderStatus {
        /// Not paid yet; triggers abandoned-cart automations.
        Pending => "pending",
        /// Paid; triggers purchase automations and cancels triggered abandoned-cart ones.
        Complete => "complete",
    }
}

/// The campaign or automation email an [`Order`] is attributed to (read-only).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Attribution {
    /// Id of the email the customer clicked before ordering.
    pub email_id: Id,
    /// Whether the email belongs to a campaign or an automation.
    #[serde(rename = "type")]
    pub attribution_type: AttributionType,
}

string_enum! {
    /// Source of an [`Attribution`].
    pub enum AttributionType {
        /// A campaign email.
        Campaign => "campaign",
        /// An automation email.
        Automation => "automation",
    }
}

string_enum! {
    /// Sort field for [`ListOrders`].
    pub enum OrderSort {
        /// Id (API default).
        Id => "id",
        /// Creation time.
        CreatedAt => "created_at",
    }
}

/// Query for [`Orders::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListOrders {
    /// Page size (API default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort<OrderSort>>,
    /// Only orders with this status.
    #[serde(rename = "filter[status]", skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    /// Only orders placed by this customer.
    #[serde(rename = "filter[customer]", skip_serializing_if = "Option::is_none")]
    pub customer: Option<Id>,
    /// Only orders with (`true`) or without (`false`) an attribution.
    #[serde(rename = "filter[attributed]", skip_serializing_if = "Option::is_none")]
    pub attributed: Option<bool>,
    /// Only orders attributed to this email.
    #[serde(rename = "filter[email_id]", skip_serializing_if = "Option::is_none")]
    pub email_id: Option<Id>,
}

impl ListOrders {
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

    /// Sort order, e.g. `Sort::desc(OrderSort::CreatedAt)`.
    #[must_use]
    pub fn sort(mut self, sort: Sort<OrderSort>) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Only orders with `status`.
    #[must_use]
    pub fn status(mut self, status: OrderStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Only orders placed by `customer_id`.
    #[must_use]
    pub fn customer(mut self, customer_id: impl Into<Id>) -> Self {
        self.customer = Some(customer_id.into());
        self
    }

    /// Only attributed (`true`) or unattributed (`false`) orders.
    #[must_use]
    pub fn attributed(mut self, attributed: bool) -> Self {
        self.attributed = Some(attributed);
        self
    }

    /// Only orders attributed to the campaign or automation email `email_id`.
    #[must_use]
    pub fn email_id(mut self, email_id: impl Into<Id>) -> Self {
        self.email_id = Some(email_id.into());
        self
    }
}

/// Body for [`Orders::create`] and one row of [`Imports::orders`](super::Imports::orders).
///
/// The customer is created unless one with the same email exists. A `pending` order
/// triggers abandoned-cart automations, so send it as soon as you know the customer's email;
/// a `complete` order triggers purchase automations.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CreateOrder {
    /// The customer.
    pub customer: CreateCustomer,
    /// The cart.
    pub cart: OrderCart,
    /// Order status. API default `pending`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    /// Order total. API default `0`; MailerLite does not sum the items for you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<f64>,
    /// Your own id for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Record the order without triggering automations (it is then never attributed). For
    /// imports this per-order value is ignored; use
    /// [`Imports::orders_without_automations`](super::Imports::orders_without_automations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_automations: Option<bool>,
}

impl CreateOrder {
    /// An order by `customer` for the items in `cart`.
    #[must_use]
    pub fn new(customer: CreateCustomer, cart: OrderCart) -> Self {
        Self {
            customer,
            cart,
            status: None,
            total_price: None,
            resource_id: None,
            without_automations: None,
        }
    }

    /// Order status.
    #[must_use]
    pub fn status(mut self, status: OrderStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Order total.
    #[must_use]
    pub fn total_price(mut self, total_price: f64) -> Self {
        self.total_price = Some(total_price);
        self
    }

    /// Your own id for the order.
    #[must_use]
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    /// Don't trigger automations for this order.
    #[must_use]
    pub fn without_automations(mut self, without: bool) -> Self {
        self.without_automations = Some(without);
        self
    }
}

/// The `cart` of a [`CreateOrder`].
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct OrderCart {
    /// Your own id for the cart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Items.
    pub items: Vec<CreateCartItem>,
}

impl OrderCart {
    /// A cart with `items`.
    pub fn new(items: impl IntoIterator<Item = CreateCartItem>) -> Self {
        Self {
            resource_id: None,
            items: items.into_iter().collect(),
        }
    }

    /// Your own id for the cart.
    #[must_use]
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }
}

/// Body for [`Orders::update`]. The cart and customer of an order cannot be changed here;
/// use [`ShopScope::cart_items`](super::ShopScope::cart_items) and
/// [`ShopScope::customers`](super::ShopScope::customers).
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UpdateOrder {
    /// Order status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    /// Order total.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<f64>,
}

impl UpdateOrder {
    /// An empty update.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Order status. Setting `complete` triggers purchase automations and cancels triggered
    /// abandoned-cart ones.
    #[must_use]
    pub fn status(mut self, status: OrderStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Order total.
    #[must_use]
    pub fn total_price(mut self, total_price: f64) -> Self {
        self.total_price = Some(total_price);
        self
    }
}

/// Order endpoints of a shop. Obtained with [`ShopScope::orders`](super::ShopScope::orders).
#[derive(Clone, Copy, Debug)]
pub struct Orders<'a> {
    pub(super) scope: Scope<'a>,
}

impl<'a> Orders<'a> {
    /// `GET /ecommerce/shops/{shop_id}/orders`: one page of orders.
    pub async fn list(self, query: &ListOrders) -> Result<Page<Order>> {
        self.scope
            .call(Method::GET, &["orders"])
            .query(query)
            .send()
            .await
    }

    /// Every order matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        self,
        query: ListOrders,
    ) -> impl futures_util::Stream<Item = Result<Order>> + 'a {
        let orders = self;
        crate::pagination::stream::paged(
            query,
            move |query| async move { orders.list(&query).await },
        )
    }

    /// `GET /ecommerce/shops/{shop_id}/orders?limit=0`: number of orders.
    pub async fn count(self) -> Result<u64> {
        self.scope.count(&["orders"]).await
    }

    /// `GET /ecommerce/shops/{shop_id}/orders/{id}`: one order.
    pub async fn get(self, id: impl AsRef<str>) -> Result<Order> {
        self.scope
            .call(Method::GET, &["orders", id.as_ref()])
            .data()
            .await
    }

    /// `POST /ecommerce/shops/{shop_id}/orders`: creates an order with its customer and
    /// cart. `pending` orders trigger abandoned-cart automations, `complete` ones purchase
    /// automations.
    pub async fn create(self, order: &CreateOrder) -> Result<Order> {
        self.scope
            .call(Method::POST, &["orders"])
            .json(order)
            .data()
            .await
    }

    /// `PUT /ecommerce/shops/{shop_id}/orders/{id}`: updates an order's status or total.
    pub async fn update(self, id: impl AsRef<str>, order: &UpdateOrder) -> Result<Order> {
        self.scope
            .call(Method::PUT, &["orders", id.as_ref()])
            .json(order)
            .data()
            .await
    }

    /// `DELETE /ecommerce/shops/{shop_id}/orders/{id}`: deletes an order together with its
    /// cart and cart items, and returns it.
    pub async fn delete(self, id: impl AsRef<str>) -> Result<Order> {
        self.scope
            .call(Method::DELETE, &["orders", id.as_ref()])
            .data()
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::{Value, json};

    use super::*;
    use crate::http::mock::{client, fixture};

    #[tokio::test]
    async fn list_with_filters_and_count() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_orders_list"))
            .respond(200, json!({"total": 123}));
        let shop = client.ecommerce().shop("9");

        let page = shop
            .orders()
            .list(
                &ListOrders::new()
                    .limit(10)
                    .page(2)
                    .sort(Sort::desc(OrderSort::CreatedAt))
                    .status(OrderStatus::Complete)
                    .customer("319868430643456756")
                    .attributed(true)
                    .email_id("31986843064993540"),
            )
            .await
            .unwrap();
        let total = shop.orders().count().await.unwrap();

        let order = &page.data[0];
        assert_eq!(order.status, Some(OrderStatus::Complete));
        assert!((order.total - 123.4).abs() < f64::EPSILON);
        let attribution = order.attribution.as_ref().unwrap();
        assert_eq!(attribution.email_id, "31986843064993540");
        assert_eq!(attribution.attribution_type, AttributionType::Campaign);
        let shop_model = order.shop.as_ref().unwrap();
        assert_eq!(shop_model.currency_sign, None);
        assert_eq!(order.cart.items[0].product.name, "Product name");
        assert_eq!(
            order.completed_at.as_ref().map(Timestamp::as_str),
            Some("2021-09-01 14:03:50")
        );
        assert_eq!(total, 123);
        let requests = mock.requests();
        requests[0].assert(
            Method::GET,
            "ecommerce/shops/9/orders?limit=10&page=2&sort=-created_at\
             &filter%5Bstatus%5D=complete&filter%5Bcustomer%5D=319868430643456756\
             &filter%5Battributed%5D=true&filter%5Bemail_id%5D=31986843064993540",
        );
        requests[1].assert(Method::GET, "ecommerce/shops/9/orders?limit=0");
    }

    #[tokio::test]
    async fn get_create_update_delete() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_order"))
            .respond(201, fixture("ecommerce_order"))
            .respond(200, fixture("ecommerce_order"))
            .respond(200, fixture("ecommerce_order"));
        let shop = client.ecommerce().shop("9");
        let orders = shop.orders();

        let fetched = orders.get("3").await.unwrap();
        orders
            .create(
                &CreateOrder::new(
                    CreateCustomer::new("customer@email.com")
                        .create_subscriber(true)
                        .accepts_marketing(true)
                        .name("Dummy")
                        .last_name("Testerson"),
                    OrderCart::new([CreateCartItem::new("1236545467567")
                        .variant("product variant")
                        .quantity(1)
                        .price(123.4)]),
                )
                .status(OrderStatus::Pending)
                .total_price(123.4)
                .without_automations(true),
            )
            .await
            .unwrap();
        orders
            .update(
                "3",
                &UpdateOrder::new()
                    .status(OrderStatus::Complete)
                    .total_price(40.0),
            )
            .await
            .unwrap();
        let deleted = orders.delete("3").await.unwrap();

        assert_eq!(fetched, deleted);
        let requests = mock.requests();
        requests[0]
            .assert(Method::GET, "ecommerce/shops/9/orders/3")
            .no_body();
        requests[1]
            .assert(Method::POST, "ecommerce/shops/9/orders")
            .body(&json!({
                "customer": {
                    "email": "customer@email.com",
                    "create_subscriber": true,
                    "accepts_marketing": true,
                    "name": "Dummy",
                    "last_name": "Testerson"
                },
                "cart": {
                    "items": [{
                        "ecommerce_product_id": "1236545467567",
                        "variant": "product variant",
                        "quantity": 1,
                        "price": 123.4
                    }]
                },
                "status": "pending",
                "total_price": 123.4,
                "without_automations": true
            }));
        requests[2]
            .assert(Method::PUT, "ecommerce/shops/9/orders/3")
            .body(&json!({"status": "complete", "total_price": 40.0}));
        requests[3]
            .assert(Method::DELETE, "ecommerce/shops/9/orders/3")
            .no_body();
    }

    #[tokio::test]
    async fn guide_flow_with_resource_ids() {
        let (client, mock) = client();
        mock.respond(201, fixture("ecommerce_guide_order"))
            .respond(200, fixture("ecommerce_guide_order_complete"));
        let shop = client.ecommerce().shop("134").with_resource_id();

        let created = shop
            .orders()
            .create(
                &CreateOrder::new(
                    CreateCustomer::new("customer@email.com")
                        .resource_id("customer1")
                        .create_subscriber(true)
                        .accepts_marketing(true)
                        .name("Dummy")
                        .last_name("Testerson"),
                    OrderCart::new([CreateCartItem::product_resource_id("prod-tshirt1")
                        .resource_id("cart-item1")
                        .quantity(1)
                        .price(20.0)])
                    .resource_id("cart1"),
                )
                .resource_id("new-order1")
                .status(OrderStatus::Pending)
                .total_price(20.0),
            )
            .await
            .unwrap();
        let completed = shop
            .orders()
            .update(
                "new-order1",
                &UpdateOrder::new().status(OrderStatus::Complete),
            )
            .await
            .unwrap();

        assert_eq!(created.resource_id.as_deref(), Some("new-order1"));
        assert!((created.total - 20.0).abs() < f64::EPSILON);
        assert!(created.cart.cart_total.abs() < f64::EPSILON);
        assert!((created.cart.items[0].product.price - 20.0).abs() < f64::EPSILON);
        assert_eq!(created.cart.items[0].variant, None);
        assert_eq!(created.attribution, None);
        let subscriber = created.customer.subscriber.as_ref().unwrap();
        assert_eq!(subscriber.source.as_deref(), Some("ecommerce"));
        assert_eq!(completed.status, Some(OrderStatus::Complete));
        assert!((completed.total - 20.0).abs() < f64::EPSILON);
        assert_eq!(completed.customer.total_spent, Some(20.0));
        assert_eq!(completed.cart.items.len(), 2);
        assert_eq!(completed.shop.as_ref().unwrap().id, "134");
        let requests = mock.requests();
        requests[0]
            .assert(Method::POST, "ecommerce/shops/134/orders?with_resource_id=")
            .body(&json!({
                "resource_id": "new-order1",
                "customer": {
                    "resource_id": "customer1",
                    "email": "customer@email.com",
                    "create_subscriber": true,
                    "accepts_marketing": true,
                    "name": "Dummy",
                    "last_name": "Testerson"
                },
                "cart": {
                    "resource_id": "cart1",
                    "items": [{
                        "resource_id": "cart-item1",
                        "product_resource_id": "prod-tshirt1",
                        "quantity": 1,
                        "price": 20.0
                    }]
                },
                "status": "pending",
                "total_price": 20.0
            }));
        requests[1]
            .assert(
                Method::PUT,
                "ecommerce/shops/134/orders/new-order1?with_resource_id=",
            )
            .body(&json!({"status": "complete"}));
    }

    /// Live API (2026-10-30): creating an order without a status returned `"status": null`.
    #[test]
    fn null_status_decodes_as_none() {
        let mut order = fixture("ecommerce_order")["data"].clone();
        order["status"] = Value::Null;

        let order: Order = serde_json::from_value(order).unwrap();
        assert_eq!(order.status, None);
    }
}
