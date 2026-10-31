//! Shop-scoped customers, orders, carts, and cart items.
use super::{List, Scope, params::*};
use crate::{Response, Result, models::ecommerce::*, response::Count};
use reqwest::Method;
/// Shop-scoped customers operations.
#[derive(Clone, Debug)]
pub struct Customers<'a> {
    pub(super) scope: Scope<'a>,
}
impl<'a> Customers<'a> {
    /// List using numbered pages.
    ///
    /// Constructing this builder makes no request. Fetch a page with [`List::send`], or
    /// enable `stream` for lazy consumption.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .customers()
    ///     .list()
    ///     .limit(25)
    ///     .send()
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Builder construction is infallible. Parameter validation and request failures
    /// are reported when fetching pages; see [`List::send`].
    pub fn list(&self) -> List<'a, Customer> {
        List::new(self.scope.clone(), false)
    }
    /// Fetch a resource by ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let customer_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .customers()
    ///     .get(customer_id)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Customer>> {
        self.scope.child(id.as_ref()).data(Method::GET, None).await
    }
    /// Create a resource; this write is never replayed automatically.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::ecommerce::params::SaveCustomer;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let customer_email = "jane@example.com";
    ///
    /// let params = SaveCustomer::new(customer_email);
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .customers()
    ///     .create(params)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Typed request serialization can return [`crate::Error::Serialize`]. Field
    /// values, resource relationships, and account restrictions are validated by
    /// MailerLite and rejected requests return [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn create(&self, params: SaveCustomer) -> Result<Response<Customer>> {
        self.scope.data(Method::POST, Some(&params)).await
    }
    /// Update a resource using its documented write fields.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::ecommerce::params::SaveCustomer;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let customer_id = "123456789";
    /// let customer_email = "jane@example.com";
    ///
    /// let params = SaveCustomer::new(customer_email);
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .customers()
    ///     .update(customer_id, params)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Typed request serialization can return [`crate::Error::Serialize`]. Field
    /// values, resource relationships, and account restrictions are validated by
    /// MailerLite and rejected requests return [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: SaveCustomer,
    ) -> Result<Response<Customer>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(&params))
            .await
    }
    /// Delete a resource and return the server's deleted-resource representation.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let customer_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .customers()
    ///     .delete(customer_id)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<Customer>> {
        self.scope
            .child(id.as_ref())
            .data(Method::DELETE, None)
            .await
    }
    /// Count resources without fetching a list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .customers()
    ///     .count()
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn count(&self) -> Result<Response<Count>> {
        self.scope.count().await
    }
    /// Accept customers for background import. The response is accepted data, not completion.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// Accepted entries do not prove that background processing has finished or that
    /// every customer was successfully persisted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::ecommerce::params::{ImportCustomer, SaveCustomer};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let customer_email = "jane@example.com";
    ///
    /// let entry = ImportCustomer::new(SaveCustomer::new(customer_email));
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .customers()
    ///     .import(vec![entry])
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Typed request serialization can return [`crate::Error::Serialize`]. Field
    /// values, resource relationships, and account restrictions are validated by
    /// MailerLite and rejected requests return [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn import(
        &self,
        customers: Vec<ImportCustomer>,
    ) -> Result<Response<Vec<AcceptedCustomer>>> {
        self.scope
            .child("import")
            .data(Method::POST, Some(&customers))
            .await
    }
}
/// Shop-scoped orders operations.
#[derive(Clone, Debug)]
pub struct Orders<'a> {
    pub(super) scope: Scope<'a>,
}
impl<'a> Orders<'a> {
    /// List using numbered pages.
    ///
    /// Constructing this builder makes no request. Fetch a page with [`List::send`], or
    /// enable `stream` for lazy consumption.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .orders()
    ///     .list()
    ///     .limit(25)
    ///     .send()
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Builder construction is infallible. Parameter validation and request failures
    /// are reported when fetching pages; see [`List::send`].
    pub fn list(&self) -> List<'a, Order> {
        List::new(self.scope.clone(), true)
    }
    /// Fetch a resource by ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let order_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .orders()
    ///     .get(order_id)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Order>> {
        self.scope.child(id.as_ref()).data(Method::GET, None).await
    }
    /// Create a resource; this write is never replayed automatically.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{Decimal, MailerLite};
    /// use mailerlite_rs::resources::ecommerce::params::{
    ///     CreateOrder, OrderCart, OrderStatus, SaveCartItem, SaveCustomer,
    /// };
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let customer_email = "jane@example.com";
    /// let product_id = "123456789";
    ///
    /// let params = CreateOrder::new(
    ///     SaveCustomer::new(customer_email),
    ///     OrderCart::new(vec![
    ///         SaveCartItem::new(product_id)
    ///             .quantity(1)
    ///             .price(Decimal::new(1999, 2)),
    ///     ]),
    /// )
    /// .status(OrderStatus::Pending)
    /// .total_price(Decimal::new(1999, 2));
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .orders()
    ///     .create(params)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Typed request serialization can return [`crate::Error::Serialize`]. Field
    /// values, resource relationships, and account restrictions are validated by
    /// MailerLite and rejected requests return [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn create(&self, params: CreateOrder) -> Result<Response<Order>> {
        self.scope.data(Method::POST, Some(&params)).await
    }
    /// Update a resource using its documented write fields.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::ecommerce::params::{OrderStatus, UpdateOrder};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let order_id = "123456789";
    ///
    /// let params = UpdateOrder::new().status(OrderStatus::Complete);
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .orders()
    ///     .update(order_id, params)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Typed request serialization can return [`crate::Error::Serialize`]. Field
    /// values, resource relationships, and account restrictions are validated by
    /// MailerLite and rejected requests return [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: UpdateOrder,
    ) -> Result<Response<Order>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(&params))
            .await
    }
    /// Delete a resource and return the server's deleted-resource representation.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let order_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .orders()
    ///     .delete(order_id)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<Order>> {
        self.scope
            .child(id.as_ref())
            .data(Method::DELETE, None)
            .await
    }
    /// Count resources without fetching a list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .orders()
    ///     .count()
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn count(&self) -> Result<Response<Count>> {
        self.scope.count().await
    }
    /// Bulk import orders using the backend's `{"orders": [...]}` request body.
    ///
    /// The SDK wraps the supplied orders; callers do not need to add the envelope.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// This backend-compatible envelope deliberately differs from the bare-array
    /// example in the public API documentation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{Decimal, MailerLite};
    /// use mailerlite_rs::resources::ecommerce::params::{
    ///     CreateOrder, OrderCart, SaveCartItem, SaveCustomer,
    /// };
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let customer_email = "jane@example.com";
    /// let product_id = "123456789";
    ///
    /// let entry = CreateOrder::new(
    ///     SaveCustomer::new(customer_email),
    ///     OrderCart::new(vec![
    ///         SaveCartItem::new(product_id)
    ///             .quantity(1)
    ///             .price(Decimal::new(1999, 2)),
    ///     ]),
    /// )
    /// .total_price(Decimal::new(1999, 2));
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .orders()
    ///     .import(vec![entry])
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Typed request serialization can return [`crate::Error::Serialize`]. Field
    /// values, resource relationships, and account restrictions are validated by
    /// MailerLite and rejected requests return [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn import(&self, orders: Vec<CreateOrder>) -> Result<Response<Vec<Order>>> {
        #[derive(serde::Serialize)]
        struct Payload {
            orders: Vec<CreateOrder>,
        }
        self.scope
            .child("import")
            .data(Method::POST, Some(&Payload { orders }))
            .await
    }
}
/// Shop-scoped carts operations.
#[derive(Clone, Debug)]
pub struct Carts<'a> {
    pub(super) scope: Scope<'a>,
}
impl<'a> Carts<'a> {
    /// List using numbered pages.
    ///
    /// Constructing this builder makes no request. Fetch a page with [`List::send`], or
    /// enable `stream` for lazy consumption.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .carts()
    ///     .list()
    ///     .limit(25)
    ///     .send()
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Builder construction is infallible. Parameter validation and request failures
    /// are reported when fetching pages; see [`List::send`].
    pub fn list(&self) -> List<'a, Cart> {
        List::new(self.scope.clone(), true)
    }
    /// Fetch a resource by ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let cart_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .carts()
    ///     .get(cart_id)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Cart>> {
        self.scope.child(id.as_ref()).data(Method::GET, None).await
    }
    /// Update a resource using its documented write fields.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::ecommerce::params::UpdateCart;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let cart_id = "123456789";
    /// let checkout_url = "https://shop.example.com/checkout";
    ///
    /// let params = UpdateCart::new().checkout_url(checkout_url);
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .carts()
    ///     .update(cart_id, params)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Typed request serialization can return [`crate::Error::Serialize`]. Field
    /// values, resource relationships, and account restrictions are validated by
    /// MailerLite and rejected requests return [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn update(&self, id: impl AsRef<str>, params: UpdateCart) -> Result<Response<Cart>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(&params))
            .await
    }
    /// Count resources without fetching a list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    ///
    /// let response = mailerlite.ecommerce().shop(shop_id).carts().count().await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn count(&self) -> Result<Response<Count>> {
        self.scope.count().await
    }
    /// Scope line-item operations to an existing cart.
    ///
    /// Constructing this handle does not create or fetch a cart. The cart ID is
    /// interpreted using the enclosing shop scope's identifier mode.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop = mailerlite.ecommerce().shop("123456789");
    /// let page = shop.carts().items("987654321").list().limit(25).send().await?;
    ///
    /// println!("Cart items on this page: {}", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn items(&self, id: impl AsRef<str>) -> CartItems<'a> {
        CartItems {
            scope: self.scope.child(id.as_ref()).child("items"),
        }
    }
}
/// Shop-scoped cartitems operations.
#[derive(Clone, Debug)]
pub struct CartItems<'a> {
    pub(super) scope: Scope<'a>,
}
impl<'a> CartItems<'a> {
    /// List using numbered pages.
    ///
    /// Constructing this builder makes no request. Fetch a page with [`List::send`], or
    /// enable `stream` for lazy consumption.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let cart_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .carts()
    ///     .items(cart_id)
    ///     .list()
    ///     .limit(25)
    ///     .send()
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Builder construction is infallible. Parameter validation and request failures
    /// are reported when fetching pages; see [`List::send`].
    pub fn list(&self) -> List<'a, CartItem> {
        List::new(self.scope.clone(), true)
    }
    /// Fetch a resource by ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let cart_id = "123456789";
    /// let cart_item_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .carts()
    ///     .items(cart_id)
    ///     .get(cart_item_id)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<CartItem>> {
        self.scope.child(id.as_ref()).data(Method::GET, None).await
    }
    /// Create a resource; this write is never replayed automatically.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{Decimal, MailerLite};
    /// use mailerlite_rs::resources::ecommerce::params::SaveCartItem;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let cart_id = "123456789";
    /// let product_id = "123456789";
    ///
    /// let params = SaveCartItem::new(product_id)
    ///     .quantity(1)
    ///     .price(Decimal::new(1999, 2));
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .carts()
    ///     .items(cart_id)
    ///     .create(params)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Typed request serialization can return [`crate::Error::Serialize`]. Field
    /// values, resource relationships, and account restrictions are validated by
    /// MailerLite and rejected requests return [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn create(&self, params: SaveCartItem) -> Result<Response<CartItem>> {
        self.scope.data(Method::POST, Some(&params)).await
    }
    /// Update a resource using its documented write fields.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{Decimal, MailerLite};
    /// use mailerlite_rs::resources::ecommerce::params::SaveCartItem;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let cart_id = "123456789";
    /// let cart_item_id = "123456789";
    /// let product_id = "123456789";
    ///
    /// let params = SaveCartItem::new(product_id)
    ///     .quantity(1)
    ///     .price(Decimal::new(1999, 2));
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .carts()
    ///     .items(cart_id)
    ///     .update(cart_item_id, params)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Typed request serialization can return [`crate::Error::Serialize`]. Field
    /// values, resource relationships, and account restrictions are validated by
    /// MailerLite and rejected requests return [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: SaveCartItem,
    ) -> Result<Response<CartItem>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(&params))
            .await
    }
    /// Delete a resource and return the server's deleted-resource representation.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let cart_id = "123456789";
    /// let cart_item_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .carts()
    ///     .items(cart_id)
    ///     .delete(cart_item_id)
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<CartItem>> {
        self.scope
            .child(id.as_ref())
            .data(Method::DELETE, None)
            .await
    }
    /// Count resources without fetching a list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let cart_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .carts()
    ///     .items(cart_id)
    ///     .count()
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty or dot (`.` or `..`) path
    /// identifier, including the enclosing shop or resource scope where applicable.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn count(&self) -> Result<Response<Count>> {
        self.scope.count().await
    }
}
