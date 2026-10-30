//! Shop management, tracking settings, and access to shop-scoped resources.
use super::{Carts, Categories, Customers, List, Orders, Products, Scope, params::SaveShop};
use crate::{
    MailerLite, Response, Result,
    http::{OperationPolicy, body},
    models::ecommerce::{Settings, Shop},
    response::Count,
};
use reqwest::Method;

/// Entry point for shop management and tracking settings.
#[derive(Clone, Copy, Debug)]
pub struct Ecommerce<'a> {
    pub(super) client: &'a MailerLite,
}
impl<'a> Ecommerce<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List connected shops; the server may initialize its internal MailerLite shop.
    /// Consequently this operation, including lazy pages, is never automatically retried.
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
    /// let response = mailerlite.ecommerce().list().limit(25).send().await?;
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
    pub fn list(&self) -> List<'a, Shop> {
        List::new(
            Scope::new(self.client, vec!["ecommerce".into(), "shops".into()]),
            false,
        )
        .without_retries()
    }
    /// Fetch a connected shop.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    ///
    /// let response = mailerlite.ecommerce().get(shop_id).await?;
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
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Shop>> {
        self.client
            .data(
                Method::GET,
                &["ecommerce", "shops", id.as_ref()],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Create a shop with its required name, URL, and currency.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::ecommerce::params::SaveShop;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_url = "https://shop.example.com";
    ///
    /// let params = SaveShop::new("Example store", shop_url, "USD");
    ///
    /// let response = mailerlite.ecommerce().create(params).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Typed request serialization can return [`crate::Error::Serialize`]. Field
    /// values, resource relationships, and account restrictions are validated by
    /// MailerLite and rejected requests return [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn create(&self, params: SaveShop) -> Result<Response<Shop>> {
        self.client
            .data(
                Method::POST,
                &["ecommerce", "shops"],
                &[],
                Some(body(&params)?),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Update an external shop; MailerLite's internal shop cannot be updated.
    ///
    /// This mutation is never automatically retried. After a timeout or connection
    /// failure, check the server state before repeating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::ecommerce::params::SaveShop;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop_id = "123456789";
    /// let shop_url = "https://shop.example.com";
    ///
    /// let params = SaveShop::new("Example store", shop_url, "USD");
    ///
    /// let response = mailerlite.ecommerce().update(shop_id, params).await?;
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
    pub async fn update(&self, id: impl AsRef<str>, params: SaveShop) -> Result<Response<Shop>> {
        self.client
            .data(
                Method::PUT,
                &["ecommerce", "shops", id.as_ref()],
                &[],
                Some(body(&params)?),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Delete an external shop and return the deleted resource.
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
    ///
    /// let response = mailerlite.ecommerce().delete(shop_id).await?;
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
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<Shop>> {
        self.client
            .data(
                Method::DELETE,
                &["ecommerce", "shops", id.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Count connected shops. This uses the shop-list endpoint, which may
    /// initialize the internal shop, so automatic retries are disabled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.ecommerce().count().await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn count(&self) -> Result<Response<Count>> {
        self.client
            .request(
                Method::GET,
                &["ecommerce", "shops"],
                &[("limit".into(), "0".into())],
                None,
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Scope catalog, customer, and order operations to a shop.
    ///
    /// Creating a scope makes no request and does not verify that the shop exists.
    /// Path validation and server access checks occur when an operation is sent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop = mailerlite.ecommerce().shop("123456789");
    /// let products = shop.products().list().limit(25).send().await?;
    ///
    /// println!("Products on this page: {}", products.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn shop(&self, id: impl AsRef<str>) -> ShopScope<'a> {
        ShopScope {
            scope: Scope::new(
                self.client,
                vec!["ecommerce".into(), "shops".into(), id.as_ref().into()],
            ),
        }
    }
    /// Read e-commerce attribution settings.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.ecommerce().settings().await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Missing resources, access restrictions, and other server rejections return
    /// [`crate::Error::Api`].
    /// HTTP transport failures, unexpected or malformed responses, and execution
    /// deadlines return [`crate::Error::Transport`], [`crate::Error::Decode`], and
    /// [`crate::Error::Timeout`] respectively.
    pub async fn settings(&self) -> Result<Response<Settings>> {
        self.client
            .data(
                Method::GET,
                &["ecommerce", "settings"],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
}
/// Operations scoped to a connected shop.
#[derive(Clone, Debug)]
pub struct ShopScope<'a> {
    pub(super) scope: Scope<'a>,
}
impl<'a> ShopScope<'a> {
    /// Address supported nested resources using your external resource IDs.
    /// The shop itself is still identified by its MailerLite shop ID.
    ///
    /// This changes how the server interprets supported nested identifiers; it
    /// does not convert IDs locally or look up the shop by an external ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop = mailerlite.ecommerce().shop("123456789").with_resource_ids();
    /// let product = shop.products().get("store-product-42").await?;
    ///
    /// println!("MailerLite product ID: {}", product.data.id);
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_resource_ids(mut self) -> Self {
        self.scope
            .query
            .push(("with_resource_id".into(), "".into()));
        self
    }
    /// Manage products.
    pub fn products(&self) -> Products<'a> {
        Products {
            scope: self.scope.child("products"),
        }
    }
    /// Manage categories and category membership.
    pub fn categories(&self) -> Categories<'a> {
        Categories {
            scope: self.scope.child("categories"),
        }
    }
    /// Manage customers.
    pub fn customers(&self) -> Customers<'a> {
        Customers {
            scope: self.scope.child("customers"),
        }
    }
    /// Manage orders.
    pub fn orders(&self) -> Orders<'a> {
        Orders {
            scope: self.scope.child("orders"),
        }
    }
    /// Read and update carts.
    pub fn carts(&self) -> Carts<'a> {
        Carts {
            scope: self.scope.child("carts"),
        }
    }
}
