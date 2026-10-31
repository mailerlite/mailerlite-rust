//! Shop-scoped products, categories, and product variants.
use super::{
    List, Scope,
    params::{ImportProduct, SaveCategory, SaveProduct, SaveVariant},
};
use crate::{
    Response, Result,
    models::ecommerce::{Category, Product, ProductVariant},
    response::Count,
};
use reqwest::Method;
/// Shop-scoped products operations.
#[derive(Clone, Debug)]
pub struct Products<'a> {
    pub(super) scope: Scope<'a>,
}
impl<'a> Products<'a> {
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
    ///     .products()
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
    pub fn list(&self) -> List<'a, Product> {
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
    /// let product_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .products()
    ///     .get(product_id)
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
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Product>> {
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
    /// use mailerlite_rs::resources::ecommerce::params::SaveProduct;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    ///
    /// let params = SaveProduct::new("Example product").price(Decimal::new(1999, 2));
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .products()
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
    pub async fn create(&self, params: SaveProduct) -> Result<Response<Product>> {
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
    /// use mailerlite_rs::resources::ecommerce::params::SaveProduct;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let product_id = "123456789";
    ///
    /// let params = SaveProduct::new("Example product").price(Decimal::new(1999, 2));
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .products()
    ///     .update(product_id, params)
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
        params: SaveProduct,
    ) -> Result<Response<Product>> {
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
    /// let product_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .products()
    ///     .delete(product_id)
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
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<Product>> {
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
    ///     .products()
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
    /// Enable or disable a product.
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
    /// let product_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .products()
    ///     .set_enabled(product_id, true)
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
    pub async fn set_enabled(
        &self,
        id: impl AsRef<str>,
        enabled: bool,
    ) -> Result<Response<Product>> {
        #[derive(serde::Serialize)]
        struct Payload {
            status: bool,
        }
        self.scope
            .child(id.as_ref())
            .child("change-status")
            .data(Method::PUT, Some(&Payload { status: enabled }))
            .await
    }
    /// Assign multiple categories; replace removes memberships absent from the list.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// Use `replace = false` to add memberships while preserving existing ones. With
    /// `replace = true`, omitted category memberships are removed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let product_id = "123456789";
    /// let category_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .products()
    ///     .assign_categories(product_id, vec![category_id.to_owned()], false)
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
    /// More than 100 category IDs are also rejected locally with `Error::Config`;
    /// category existence and ownership are checked by the server.
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
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
        #[derive(serde::Serialize)]
        struct Payload {
            categories: Vec<String>,
            replace: bool,
        }
        self.scope
            .child(id.as_ref())
            .child("categories")
            .child("multiple")
            .data(
                Method::PUT,
                Some(&Payload {
                    categories,
                    replace,
                }),
            )
            .await
    }
    /// Scope product-variant operations to a product.
    ///
    /// No request is made until a variant operation is invoked. The product ID
    /// uses the enclosing shop scope's identifier mode.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop = mailerlite.ecommerce().shop("123456789");
    /// let variants = shop.products().variants("987654321").list().await?;
    ///
    /// println!("Variants: {}", variants.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn variants(&self, id: impl AsRef<str>) -> Variants<'a> {
        Variants {
            scope: self.scope.child(id.as_ref()).child("variants"),
        }
    }
    /// Bulk import products and their categories.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{Decimal, MailerLite};
    /// use mailerlite_rs::resources::ecommerce::params::{ImportProduct, SaveProduct};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    ///
    /// let entry =
    ///     ImportProduct::new(SaveProduct::new("Example product").price(Decimal::new(1999, 2)));
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .products()
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
    pub async fn import(&self, products: Vec<ImportProduct>) -> Result<Response<Vec<Product>>> {
        self.scope
            .child("import")
            .data(Method::POST, Some(&products))
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
    ///     .categories()
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
    pub fn list(&self) -> List<'a, Category> {
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
    /// let category_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .categories()
    ///     .get(category_id)
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
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Category>> {
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
    /// use mailerlite_rs::resources::ecommerce::params::SaveCategory;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    ///
    /// let params = SaveCategory::new("Example category");
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .categories()
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
    pub async fn create(&self, params: SaveCategory) -> Result<Response<Category>> {
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
    /// use mailerlite_rs::resources::ecommerce::params::SaveCategory;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let category_id = "123456789";
    ///
    /// let params = SaveCategory::new("Example category");
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .categories()
    ///     .update(category_id, params)
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
        params: SaveCategory,
    ) -> Result<Response<Category>> {
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
    /// let category_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .categories()
    ///     .delete(category_id)
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
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<Category>> {
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
    ///     .categories()
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
    /// List products assigned to a category.
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
    /// let category_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .categories()
    ///     .products(category_id)
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
    pub fn products(&self, id: impl AsRef<str>) -> List<'a, Product> {
        List::new(self.scope.child(id.as_ref()).child("products"), false)
    }
    /// Assign a product to a category and return the product.
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
    /// let category_id = "123456789";
    /// let product_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .categories()
    ///     .assign(category_id, product_id)
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
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// This removes category membership; it does not delete the product.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let category_id = "123456789";
    /// let product_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .categories()
    ///     .unassign(category_id, product_id)
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
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::ecommerce::params::SaveCategory;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    ///
    /// let entry = SaveCategory::new("Example category");
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .categories()
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
    pub async fn import(&self, categories: Vec<SaveCategory>) -> Result<Response<Vec<Category>>> {
        self.scope
            .child("import")
            .data(Method::POST, Some(&categories))
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let product_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .products()
    ///     .variants(product_id)
    ///     .list()
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
    pub async fn list(&self) -> Result<Response<Vec<ProductVariant>>> {
        self.scope.data(Method::GET, None).await
    }
    /// Create variants. If any item has a resource_id, the server deletes existing variants whose resource_id is absent from this request.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// Treat a request containing any `resource_id` as synchronization: include every
    /// variant resource ID that must remain. The response is the containing product,
    /// not a standalone variant.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{Decimal, MailerLite};
    /// use mailerlite_rs::resources::ecommerce::params::{PriceType, SaveVariant};
    /// use std::collections::BTreeMap;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let product_id = "123456789";
    ///
    /// let variant = SaveVariant::new(
    ///     "Black / Medium",
    ///     BTreeMap::from([
    ///         ("color".into(), "Black".into()),
    ///         ("size".into(), "Medium".into()),
    ///     ]),
    /// )
    /// .price_type(PriceType::Fixed)
    /// .price(Decimal::new(1999, 2));
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .products()
    ///     .variants(product_id)
    ///     .create(vec![variant])
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
    pub async fn create(&self, variants: Vec<SaveVariant>) -> Result<Response<Product>> {
        self.scope.data(Method::POST, Some(&variants)).await
    }
    /// Update a variant and return its containing product.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{Decimal, MailerLite};
    /// use mailerlite_rs::resources::ecommerce::params::{PriceType, SaveVariant};
    /// use std::collections::BTreeMap;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let product_id = "123456789";
    /// let variant_id = "123456789";
    ///
    /// let variant = SaveVariant::new(
    ///     "Black / Medium",
    ///     BTreeMap::from([
    ///         ("color".into(), "Black".into()),
    ///         ("size".into(), "Medium".into()),
    ///     ]),
    /// )
    /// .price_type(PriceType::Fixed)
    /// .price(Decimal::new(1999, 2));
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .products()
    ///     .variants(product_id)
    ///     .update(variant_id, variant)
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
        variant: SaveVariant,
    ) -> Result<Response<Product>> {
        self.scope
            .child(id.as_ref())
            .data(Method::PUT, Some(&variant))
            .await
    }
    /// Delete a variant and return the product with its remaining variants.
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
    /// let product_id = "123456789";
    /// let variant_id = "123456789";
    ///
    /// let response = mailerlite
    ///     .ecommerce()
    ///     .shop(shop_id)
    ///     .products()
    ///     .variants(product_id)
    ///     .delete(variant_id)
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
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<Product>> {
        self.scope
            .child(id.as_ref())
            .data(Method::DELETE, None)
            .await
    }
}
