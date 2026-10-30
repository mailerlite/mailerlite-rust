//! Typed e-commerce list builders, filters, sorting, and pagination.
use super::{
    Scope,
    params::{OrderStatus, ProductType},
};
use crate::{
    Error, Response, Result,
    http::OperationPolicy,
    models::ecommerce::{Order, Product},
    pagination::NumberedPage,
};
use reqwest::Method;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
/// Sort fields supported by e-commerce collection endpoints.
#[derive(Debug, Clone, Copy)]
pub enum Sort {
    /// ID ascending.
    Id,
    /// ID descending.
    IdDescending,
    /// Name ascending, supported by products and categories.
    Name,
    /// Name descending, supported by products and categories.
    NameDescending,
    /// Creation time ascending.
    CreatedAt,
    /// Creation time descending.
    CreatedAtDescending,
}
impl Sort {
    fn as_str(self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::IdDescending => "-id",
            Self::Name => "name",
            Self::NameDescending => "-name",
            Self::CreatedAt => "created_at",
            Self::CreatedAtDescending => "-created_at",
        }
    }
}
/// A numbered e-commerce list request. Fetch one page or lazily stream items.
#[derive(Debug, Clone)]
pub struct List<'a, T> {
    scope: Scope<'a>,
    sortable: bool,
    policy: OperationPolicy,
    marker: PhantomData<T>,
}
impl<'a, T: DeserializeOwned + 'a> List<'a, T> {
    pub(super) fn new(scope: Scope<'a>, sortable: bool) -> Self {
        Self {
            scope,
            sortable,
            policy: OperationPolicy::READ,
            marker: PhantomData,
        }
    }
    pub(super) fn without_retries(mut self) -> Self {
        self.policy = OperationPolicy::WRITE.with_success_statuses(&[200]);
        self
    }
    fn set(mut self, key: &str, value: impl Into<String>) -> Self {
        self.scope.query.retain(|(k, _)| k != key);
        self.scope.query.push((key.into(), value.into()));
        self
    }
    /// Select a page, starting at one.
    pub fn page(self, page: u64) -> Self {
        self.set("page", page.to_string())
    }
    /// Set page size; use the resource's count method for `limit=0`.
    pub fn limit(self, limit: u64) -> Self {
        self.set("limit", limit.to_string())
    }
    /// Select a documented sort order; unsupported endpoint/field combinations fail locally.
    pub fn sort(self, sort: Sort) -> Self {
        self.set("sort", sort.as_str())
    }
    fn validate(&self) -> Result<()> {
        for (key, value) in &self.scope.query {
            if (key == "page" || key == "limit") && value == "0" {
                return Err(Error::Config("page and limit must be positive".into()));
            }
            if key == "sort" {
                if !self.sortable {
                    return Err(Error::Config(
                        "sorting is not documented for this endpoint".into(),
                    ));
                }
                if value.contains("name")
                    && !self
                        .scope
                        .path
                        .iter()
                        .any(|s| s == "products" || s == "categories")
                {
                    return Err(Error::Config(
                        "name sorting is not supported by this endpoint".into(),
                    ));
                }
            }
        }
        Ok(())
    }
    /// Fetch exactly one page.
    pub async fn send(self) -> Result<Response<NumberedPage<T>>> {
        self.validate()?;
        let path: Vec<&str> = self.scope.path.iter().map(String::as_str).collect();
        self.scope
            .client
            .request(Method::GET, &path, &self.scope.query, None, self.policy)
            .await
    }
    /// Fetch pages on demand, retaining at most one page. Dropping stops further requests.
    #[cfg(feature = "stream")]
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<T>> + 'a {
        let validation = self.validate();
        crate::pagination::validated_stream(
            validation,
            crate::pagination::numbered_stream_with_policy(
                self.scope.client,
                self.scope.path,
                self.scope.query,
                self.policy,
            ),
        )
    }
}
impl List<'_, Product> {
    /// Match products whose name contains this text.
    pub fn name(self, name: impl Into<String>) -> Self {
        self.set("filter[name]", name)
    }
    /// Filter MailerLite-shop product type.
    pub fn kind(self, kind: ProductType) -> Self {
        self.set("filter[type]", kind.as_str())
    }
    /// Filter enabled products.
    pub fn enabled(self, enabled: bool) -> Self {
        self.set("filter[enabled]", enabled.to_string())
    }
    /// Filter by assigned category.
    pub fn category(self, id: impl Into<String>) -> Self {
        self.set("filter[category]", id)
    }
    /// Filter automation exclusion state.
    pub fn excluded_from_automations(self, excluded: bool) -> Self {
        self.set("filter[exclude_from_automations]", excluded.to_string())
    }
}
impl List<'_, Order> {
    /// Filter order status.
    pub fn status(self, status: OrderStatus) -> Self {
        self.set("filter[status]", status.as_str())
    }
    /// Filter orders placed by this customer.
    pub fn customer(self, id: impl Into<String>) -> Self {
        self.set("filter[customer]", id)
    }
    /// Filter by whether an email earned the order.
    pub fn attributed(self, attributed: bool) -> Self {
        self.set("filter[attributed]", attributed.to_string())
    }
    /// Filter by attributed email ID.
    pub fn email(self, id: impl Into<String>) -> Self {
        self.set("filter[email_id]", id)
    }
}
