//! Shop-scoped products, categories, and product variants.
use super::{List, Scope, params::*};
use crate::{Response, Result, http::body, models::ecommerce::*, response::Count};
use reqwest::Method;
/// Shop-scoped products operations.
#[derive(Clone, Debug)]
pub struct Products<'a> {
    pub(super) scope: Scope<'a>,
}
impl<'a> Products<'a> {
    /// List using numbered pages.
    pub fn list(&self) -> List<'a, Product> {
        List::new(self.scope.clone(), true)
    }
    /// Fetch a resource by ID.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Product>> {
        self.scope.child(id.as_ref()).data(Method::GET, None).await
    }
    /// Create a resource; this write is never replayed automatically.
    pub async fn create(&self, params: SaveProduct) -> Result<Response<Product>> {
        self.scope.data(Method::POST, Some(body(&params)?)).await
    }
    /// Update a resource using its documented write fields.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: SaveProduct,
    ) -> Result<Response<Product>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(body(&params)?))
            .await
    }
    /// Delete a resource and return the server's deleted-resource representation.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<Product>> {
        self.scope
            .child(id.as_ref())
            .data(Method::DELETE, None)
            .await
    }
    /// Count resources without fetching a list.
    pub async fn count(&self) -> Result<Response<Count>> {
        self.scope.count().await
    }
    /// Enable or disable a product.
    pub async fn set_enabled(
        &self,
        id: impl AsRef<str>,
        enabled: bool,
    ) -> Result<Response<Product>> {
        self.scope
            .child(id.as_ref())
            .child("change-status")
            .data(Method::PUT, Some(serde_json::json!({"status":enabled})))
            .await
    }
    /// Assign multiple categories; replace removes memberships absent from the list.
    pub async fn assign_categories(
        &self,
        id: impl AsRef<str>,
        categories: Vec<String>,
        replace: bool,
    ) -> Result<Response<Product>> {
        if categories.len() > 100 {
            return Err(crate::Error::Config(
                "at most 100 category IDs are allowed".into(),
            ));
        }
        self.scope
            .child(id.as_ref())
            .child("categories")
            .child("multiple")
            .data(
                Method::PUT,
                Some(serde_json::json!({"categories":categories,"replace":replace})),
            )
            .await
    }
    /// Scope product-variant operations to a product.
    pub fn variants(&self, id: impl AsRef<str>) -> Variants<'a> {
        Variants {
            scope: self.scope.child(id.as_ref()).child("variants"),
        }
    }
    /// Bulk import products and their categories.
    pub async fn import(&self, products: Vec<ImportProduct>) -> Result<Response<Vec<Product>>> {
        self.scope
            .child("import")
            .data(Method::POST, Some(body(&products)?))
            .await
    }
}
/// Shop-scoped categories operations.
#[derive(Clone, Debug)]
pub struct Categories<'a> {
    pub(super) scope: Scope<'a>,
}
impl<'a> Categories<'a> {
    /// List using numbered pages.
    pub fn list(&self) -> List<'a, Category> {
        List::new(self.scope.clone(), true)
    }
    /// Fetch a resource by ID.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Category>> {
        self.scope.child(id.as_ref()).data(Method::GET, None).await
    }
    /// Create a resource; this write is never replayed automatically.
    pub async fn create(&self, params: SaveCategory) -> Result<Response<Category>> {
        self.scope.data(Method::POST, Some(body(&params)?)).await
    }
    /// Update a resource using its documented write fields.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: SaveCategory,
    ) -> Result<Response<Category>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(body(&params)?))
            .await
    }
    /// Delete a resource and return the server's deleted-resource representation.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<Category>> {
        self.scope
            .child(id.as_ref())
            .data(Method::DELETE, None)
            .await
    }
    /// Count resources without fetching a list.
    pub async fn count(&self) -> Result<Response<Count>> {
        self.scope.count().await
    }
    /// List products assigned to a category.
    pub fn products(&self, id: impl AsRef<str>) -> List<'a, Product> {
        List::new(self.scope.child(id.as_ref()).child("products"), false)
    }
    /// Assign a product to a category and return the product.
    pub async fn assign(
        &self,
        category: impl AsRef<str>,
        product: impl AsRef<str>,
    ) -> Result<Response<Product>> {
        self.scope
            .child(category.as_ref())
            .child("products")
            .child(product.as_ref())
            .data(Method::PUT, None)
            .await
    }
    /// Remove a product from a category and return the product.
    pub async fn unassign(
        &self,
        category: impl AsRef<str>,
        product: impl AsRef<str>,
    ) -> Result<Response<Product>> {
        self.scope
            .child(category.as_ref())
            .child("products")
            .child(product.as_ref())
            .data(Method::DELETE, None)
            .await
    }
    /// Bulk import categories using an array request body.
    pub async fn import(&self, categories: Vec<SaveCategory>) -> Result<Response<Vec<Category>>> {
        self.scope
            .child("import")
            .data(Method::POST, Some(body(&categories)?))
            .await
    }
}
/// Product variants. Writes return the containing product.
#[derive(Debug, Clone)]
pub struct Variants<'a> {
    scope: Scope<'a>,
}
impl Variants<'_> {
    /// List variants without undocumented pagination parameters.
    pub async fn list(&self) -> Result<Response<Vec<ProductVariant>>> {
        self.scope.data(Method::GET, None).await
    }
    /// Create variants. If any item has a resource_id, the server deletes existing variants whose resource_id is absent from this request.
    pub async fn create(&self, variants: Vec<SaveVariant>) -> Result<Response<Product>> {
        self.scope.data(Method::POST, Some(body(&variants)?)).await
    }
    /// Update a variant and return its containing product.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        variant: SaveVariant,
    ) -> Result<Response<Product>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(body(&variant)?))
            .await
    }
    /// Delete a variant and return the product with its remaining variants.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<Product>> {
        self.scope
            .child(id.as_ref())
            .data(Method::DELETE, None)
            .await
    }
}
