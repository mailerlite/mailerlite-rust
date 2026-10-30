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
    pub fn list(&self) -> List<'a, Shop> {
        List::new(
            Scope::new(self.client, vec!["ecommerce".into(), "shops".into()]),
            false,
        )
        .without_retries()
    }
    /// Fetch a connected shop.
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
    pub fn shop(&self, id: impl AsRef<str>) -> ShopScope<'a> {
        ShopScope {
            scope: Scope::new(
                self.client,
                vec!["ecommerce".into(), "shops".into(), id.as_ref().into()],
            ),
        }
    }
    /// Read e-commerce attribution settings.
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
