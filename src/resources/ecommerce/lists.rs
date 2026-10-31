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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[must_use = "request builders do nothing until sent; call .send().await"]
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
    ///
    /// The returned page retains continuation metadata. This method does not
    /// follow response links or fetch additional pages. Shop listings may
    /// initialize the internal shop, and their requests are never retried even
    /// when safe-read retries are enabled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{MailerLite, resources::ecommerce::Sort};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop = mailerlite.ecommerce().shop("123456789");
    /// let page = shop.products().list()
    ///     .sort(Sort::Name)
    ///     .page(1)
    ///     .limit(25)
    ///     .send()
    ///     .await?;
    ///
    /// println!("Next page: {:?}", page.data.next_page());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] before dispatch for a zero page or limit, a sort
    /// on an endpoint that does not support sorting, or name sorting outside
    /// product/category paths. Empty or dot path identifiers are also rejected.
    /// Server validation of filters and resource access returns [`Error::Api`].
    /// HTTP failures, malformed responses, and execution deadlines return
    /// [`Error::Transport`], [`Error::Decode`], and [`Error::Timeout`]. Missing
    /// continuation metadata is valid and means no known next page.
    pub async fn send(self) -> Result<Response<NumberedPage<T>>> {
        self.validate()?;
        let path: Vec<&str> = self.scope.path.iter().map(String::as_str).collect();
        self.scope
            .client
            .request(Method::GET, &path, &self.scope.query, None, self.policy)
            .await
    }
    /// Fetch pages on demand, retaining at most one page. Dropping stops further requests.
    ///
    /// Requires the `stream` feature. Requests begin when the stream is polled;
    /// filters and external-resource-ID mode are retained across pages. Response
    /// links are never followed. Each page has its own execution deadline, rather
    /// than one deadline for consuming the entire stream. Shop-list requests
    /// remain non-retryable because they may initialize the internal shop.
    ///
    /// # Examples
    ///
    /// This example also uses `futures-util` to consume the stream.
    ///
    /// ```no_run
    /// use futures_util::TryStreamExt;
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let shop = mailerlite.ecommerce().shop("123456789");
    /// let stream = shop.products().list().limit(25).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(product) = stream.try_next().await? {
    ///     println!("{}", product.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors are yielded as stream items. The first poll reports invalid
    /// parameters without dispatching a request; see [`Self::send`] for validation
    /// and HTTP errors. A reported page number that differs from the requested
    /// page yields [`Error::Pagination`]. Missing continuation metadata ends the
    /// stream normally. An error terminates the stream, but items already yielded
    /// remain available to the caller.
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
