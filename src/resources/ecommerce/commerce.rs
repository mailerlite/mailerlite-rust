//! Shop-scoped customers, orders, carts, and cart items.
use super::{List, Scope, params::*};
use crate::{Response, Result, http::body, models::ecommerce::*, response::Count};
use reqwest::Method;
/// Shop-scoped customers operations.
#[derive(Clone, Debug)]
pub struct Customers<'a> {
    pub(super) scope: Scope<'a>,
}
impl<'a> Customers<'a> {
    /// List using numbered pages.
    pub fn list(&self) -> List<'a, Customer> {
        List::new(self.scope.clone(), false)
    }
    /// Fetch a resource by ID.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Customer>> {
        self.scope.child(id.as_ref()).data(Method::GET, None).await
    }
    /// Create a resource; this write is never replayed automatically.
    pub async fn create(&self, params: SaveCustomer) -> Result<Response<Customer>> {
        self.scope.data(Method::POST, Some(body(&params)?)).await
    }
    /// Update a resource using its documented write fields.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: SaveCustomer,
    ) -> Result<Response<Customer>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(body(&params)?))
            .await
    }
    /// Delete a resource and return the server's deleted-resource representation.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<Customer>> {
        self.scope
            .child(id.as_ref())
            .data(Method::DELETE, None)
            .await
    }
    /// Count resources without fetching a list.
    pub async fn count(&self) -> Result<Response<Count>> {
        self.scope.count().await
    }
    /// Accept customers for background import. The response is accepted data, not completion.
    pub async fn import(
        &self,
        customers: Vec<ImportCustomer>,
    ) -> Result<Response<Vec<AcceptedCustomer>>> {
        self.scope
            .child("import")
            .data(Method::POST, Some(body(&customers)?))
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
    pub fn list(&self) -> List<'a, Order> {
        List::new(self.scope.clone(), true)
    }
    /// Fetch a resource by ID.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Order>> {
        self.scope.child(id.as_ref()).data(Method::GET, None).await
    }
    /// Create a resource; this write is never replayed automatically.
    pub async fn create(&self, params: CreateOrder) -> Result<Response<Order>> {
        self.scope.data(Method::POST, Some(body(&params)?)).await
    }
    /// Update a resource using its documented write fields.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: UpdateOrder,
    ) -> Result<Response<Order>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(body(&params)?))
            .await
    }
    /// Delete a resource and return the server's deleted-resource representation.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<Order>> {
        self.scope
            .child(id.as_ref())
            .data(Method::DELETE, None)
            .await
    }
    /// Count resources without fetching a list.
    pub async fn count(&self) -> Result<Response<Count>> {
        self.scope.count().await
    }
    /// Bulk import orders using the backend's `{"orders": [...]}` request body.
    ///
    /// The SDK wraps the supplied orders; callers do not need to add the envelope.
    pub async fn import(&self, orders: Vec<CreateOrder>) -> Result<Response<Vec<Order>>> {
        #[derive(serde::Serialize)]
        struct Payload {
            orders: Vec<CreateOrder>,
        }
        self.scope
            .child("import")
            .data(Method::POST, Some(body(&Payload { orders })?))
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
    pub fn list(&self) -> List<'a, Cart> {
        List::new(self.scope.clone(), true)
    }
    /// Fetch a resource by ID.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Cart>> {
        self.scope.child(id.as_ref()).data(Method::GET, None).await
    }
    /// Update a resource using its documented write fields.
    pub async fn update(&self, id: impl AsRef<str>, params: UpdateCart) -> Result<Response<Cart>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(body(&params)?))
            .await
    }
    /// Count resources without fetching a list.
    pub async fn count(&self) -> Result<Response<Count>> {
        self.scope.count().await
    }
    /// Scope line-item operations to an existing cart.
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
    pub fn list(&self) -> List<'a, CartItem> {
        List::new(self.scope.clone(), true)
    }
    /// Fetch a resource by ID.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<CartItem>> {
        self.scope.child(id.as_ref()).data(Method::GET, None).await
    }
    /// Create a resource; this write is never replayed automatically.
    pub async fn create(&self, params: SaveCartItem) -> Result<Response<CartItem>> {
        self.scope.data(Method::POST, Some(body(&params)?)).await
    }
    /// Update a resource using its documented write fields.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: SaveCartItem,
    ) -> Result<Response<CartItem>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(body(&params)?))
            .await
    }
    /// Delete a resource and return the server's deleted-resource representation.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<CartItem>> {
        self.scope
            .child(id.as_ref())
            .data(Method::DELETE, None)
            .await
    }
    /// Count resources without fetching a list.
    pub async fn count(&self) -> Result<Response<Count>> {
        self.scope.count().await
    }
}
