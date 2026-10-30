//! Customers: <https://developers.mailerlite.com/ecommerce/customers>

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::Scope;
use crate::{
    Page, Result,
    subscribers::Subscriber,
    types::{Id, Sort, Timestamp, de, string_enum},
};

/// A shop customer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Customer {
    /// Customer id.
    pub id: Id,
    /// Your own id for the customer, if any.
    #[serde(default)]
    pub resource_id: Option<String>,
    /// Email address.
    pub email: String,
    /// Whether the customer accepts marketing emails.
    #[serde(default, deserialize_with = "de::null_default")]
    pub accepts_marketing: bool,
    /// Total spent in the shop.
    #[serde(default, deserialize_with = "de::optional_number")]
    pub total_spent: Option<f64>,
    /// Number of orders.
    #[serde(default)]
    pub orders_count: Option<u64>,
    /// The linked subscriber, or `None` if the customer has none.
    #[serde(default)]
    pub subscriber: Option<Subscriber>,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
}

string_enum! {
    /// Sort field for [`ListCustomers`].
    pub enum CustomerSort {
        /// Id (API default).
        Id => "id",
        /// Email address.
        Email => "email",
        /// Creation time.
        CreatedAt => "created_at",
    }
}

/// Query for [`Customers::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListCustomers {
    /// Page size (API default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort<CustomerSort>>,
    /// Only the customer with this email (exact, case-insensitive match).
    #[serde(rename = "filter[email]", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl ListCustomers {
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

    /// Sort order, e.g. `Sort::desc(CustomerSort::CreatedAt)`.
    #[must_use]
    pub fn sort(mut self, sort: Sort<CustomerSort>) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Only the customer with `email` (exact, case-insensitive).
    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
}

/// Body for [`Customers::create`] and [`Customers::update`], the `customer` of a
/// [`CreateOrder`](super::CreateOrder) and one row of
/// [`Imports::customers`](super::Imports::customers).
///
/// `name` / `last_name` are saved to the linked subscriber's fields, so they only take
/// effect when the customer has (or [`create_subscriber`](Self::create_subscriber) creates)
/// a subscriber.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CreateCustomer {
    /// Email address.
    pub email: String,
    /// Your own id for the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Whether the customer accepts marketing emails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts_marketing: Option<bool>,
    /// Total spent in the shop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spent: Option<f64>,
    /// Number of orders (documented for imports).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders_count: Option<u64>,
    /// Create a subscriber for the customer if none exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_subscriber: Option<bool>,
    /// First name (at most 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Last name (at most 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl CreateCustomer {
    /// A customer with `email`.
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            resource_id: None,
            accepts_marketing: None,
            total_spent: None,
            orders_count: None,
            create_subscriber: None,
            name: None,
            last_name: None,
        }
    }

    /// Your own id for the customer.
    #[must_use]
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    /// Whether the customer accepts marketing emails.
    #[must_use]
    pub fn accepts_marketing(mut self, accepts: bool) -> Self {
        self.accepts_marketing = Some(accepts);
        self
    }

    /// Total spent in the shop.
    #[must_use]
    pub fn total_spent(mut self, total_spent: f64) -> Self {
        self.total_spent = Some(total_spent);
        self
    }

    /// Number of orders (imports).
    #[must_use]
    pub fn orders_count(mut self, orders_count: u64) -> Self {
        self.orders_count = Some(orders_count);
        self
    }

    /// Create a subscriber for the customer if none exists. `false` on update never removes
    /// an existing subscriber.
    #[must_use]
    pub fn create_subscriber(mut self, create: bool) -> Self {
        self.create_subscriber = Some(create);
        self
    }

    /// First name; a non-empty value overwrites the subscriber's Name field.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Last name; a non-empty value overwrites the subscriber's Last name field.
    #[must_use]
    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }
}

/// Customer endpoints of a shop. Obtained with
/// [`ShopScope::customers`](super::ShopScope::customers).
#[derive(Clone, Copy, Debug)]
pub struct Customers<'a> {
    pub(super) scope: Scope<'a>,
}

impl<'a> Customers<'a> {
    /// `GET /ecommerce/shops/{shop_id}/customers`: one page of customers.
    pub async fn list(&self, query: &ListCustomers) -> Result<Page<Customer>> {
        self.scope
            .call(Method::GET, &["customers"])
            .query(query)
            .send()
            .await
    }

    /// Every customer matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        &self,
        query: ListCustomers,
    ) -> impl futures_util::Stream<Item = Result<Customer>> + 'a {
        let customers = *self;
        crate::pagination::stream::paged(
            query,
            move |query| async move { customers.list(&query).await },
        )
    }

    /// `GET /ecommerce/shops/{shop_id}/customers?limit=0`: number of customers.
    pub async fn count(&self) -> Result<u64> {
        self.scope.count(&["customers"]).await
    }

    /// `GET /ecommerce/shops/{shop_id}/customers/{id}`: one customer.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Customer> {
        self.scope
            .call(Method::GET, &["customers", id.as_ref()])
            .data()
            .await
    }

    /// `POST /ecommerce/shops/{shop_id}/customers`: creates a customer, linking it to the
    /// subscriber with the same email if one exists.
    pub async fn create(&self, customer: &CreateCustomer) -> Result<Customer> {
        self.scope
            .call(Method::POST, &["customers"])
            .json(customer)
            .data()
            .await
    }

    /// `PUT /ecommerce/shops/{shop_id}/customers/{id}`: updates a customer.
    pub async fn update(&self, id: impl AsRef<str>, customer: &CreateCustomer) -> Result<Customer> {
        self.scope
            .call(Method::PUT, &["customers", id.as_ref()])
            .json(customer)
            .data()
            .await
    }

    /// `DELETE /ecommerce/shops/{shop_id}/customers/{id}`: deletes a customer and returns
    /// it. The linked subscriber is not affected.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Customer> {
        self.scope
            .call(Method::DELETE, &["customers", id.as_ref()])
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
        http::mock::{client, fixture},
        subscribers::SubscriberStatus,
    };

    #[tokio::test]
    async fn list_and_count() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_customers_list"))
            .respond(200, json!({"total": 123}));
        let shop = client.ecommerce().shop("9");

        let page = shop
            .customers()
            .list(
                &ListCustomers::new()
                    .limit(1)
                    .page(1)
                    .sort(Sort::asc(CustomerSort::Email))
                    .email("customer@email.com"),
            )
            .await
            .unwrap();
        let total = shop.customers().count().await.unwrap();

        let customer = &page.data[0];
        assert_eq!(customer.email, "customer@email.com");
        assert_eq!(customer.total_spent, Some(1234.99));
        assert_eq!(customer.orders_count, Some(3));
        let subscriber = customer.subscriber.as_ref().unwrap();
        assert_eq!(subscriber.status, SubscriberStatus::Active);
        assert_eq!(total, 123);
        let requests = mock.requests();
        requests[0].assert(
            Method::GET,
            "ecommerce/shops/9/customers?limit=1&page=1&sort=email\
             &filter%5Bemail%5D=customer%40email.com",
        );
        requests[1].assert(Method::GET, "ecommerce/shops/9/customers?limit=0");
    }

    #[tokio::test]
    async fn get_create_update_delete() {
        let (client, mock) = client();
        for _ in 0..4 {
            mock.respond(200, fixture("ecommerce_customer"));
        }
        let shop = client.ecommerce().shop("9");
        let customers = shop.customers();

        let fetched = customers.get("5").await.unwrap();
        customers
            .create(
                &CreateCustomer::new("customer@email.com")
                    .accepts_marketing(true)
                    .total_spent(1234.99)
                    .create_subscriber(true)
                    .name("Dummy")
                    .last_name("Testerson"),
            )
            .await
            .unwrap();
        customers
            .update(
                "5",
                &CreateCustomer::new("customer@email.com").resource_id("customer1"),
            )
            .await
            .unwrap();
        let deleted = customers.delete("5").await.unwrap();

        assert_eq!(fetched, deleted);
        assert_eq!(
            fetched.created_at.as_ref().map(Timestamp::as_str),
            Some("2021-09-01 14:03:50")
        );
        let requests = mock.requests();
        requests[0]
            .assert(Method::GET, "ecommerce/shops/9/customers/5")
            .no_body();
        requests[1]
            .assert(Method::POST, "ecommerce/shops/9/customers")
            .body(&json!({
                "email": "customer@email.com",
                "accepts_marketing": true,
                "total_spent": 1234.99,
                "create_subscriber": true,
                "name": "Dummy",
                "last_name": "Testerson"
            }));
        requests[2]
            .assert(Method::PUT, "ecommerce/shops/9/customers/5")
            .body(&json!({"email": "customer@email.com", "resource_id": "customer1"}));
        requests[3]
            .assert(Method::DELETE, "ecommerce/shops/9/customers/5")
            .no_body();
    }
}
