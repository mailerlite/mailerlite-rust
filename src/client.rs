//! Reusable HTTP client, shared state, and transport entry points.
use crate::{
    ApiVersion, Error, Response, Result,
    config::MailerLiteBuilder,
    http::{OperationPolicy, RequestBody, rate_limit::RateLimiter, retry::RetryPolicy},
    response::DataEnvelope,
};
use reqwest::{Method, Url, header::HeaderValue};
use serde::de::DeserializeOwned;
use std::{fmt, sync::Arc, time::Duration};

/// Reusable, cheaply cloneable asynchronous MailerLite API client.
///
/// The client is `Send + Sync`; clones share the connection pool and rate-limit
/// state. Request futures using SDK parameter models and thread-safe arguments
/// can move between threads. Borrowed futures are not necessarily `'static`:
/// move a client clone into an `async move` block when using `tokio::spawn`.
///
/// # Examples
///
/// ```
/// use mailerlite_rs::MailerLite;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = MailerLite::new("api-token")?;
/// let task_client = client.clone();
/// let task = tokio::spawn(async move {
///     task_client.subscribers().get("123").await
/// });
///
/// let response = task.await??;
///
/// println!("{}", response.data.id);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct MailerLite {
    pub(crate) inner: Arc<Inner>,
}
pub(crate) struct Inner {
    pub http: reqwest::Client,
    pub base_url: Url,
    pub authorization: HeaderValue,
    pub version: ApiVersion,
    pub request_timeout: Duration,
    pub operation_timeout: Duration,
    pub retry: RetryPolicy,
    pub limiter: RateLimiter,
    pub max_response_bytes: usize,
}
impl fmt::Debug for MailerLite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MailerLite")
            .field("authorization", &"[REDACTED]")
            .field("api_version", &self.inner.version)
            .finish_non_exhaustive()
    }
}
impl MailerLite {
    /// Create a client using the latest API version and default timeouts and pacing.
    /// Automatic retries are disabled by default.
    ///
    /// Construction creates a reusable HTTP client but makes no API request.
    /// Token validity is checked by the server when a request is sent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    /// # let _ = mailerlite;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] for an empty token or one that cannot be encoded
    /// as an HTTP header, or [`Error::Transport`] if the HTTP client cannot be
    /// initialized. This does not authenticate the token with MailerLite.
    pub fn new(token: impl Into<String>) -> Result<Self> {
        Self::builder(token).build()
    }
    /// Configure a client with an explicit bearer token.
    ///
    /// Set optional values on the returned builder, then call
    /// [`MailerLiteBuilder::build`] to validate them. No request is sent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{MailerLite, RetryPolicy};
    /// use std::time::Duration;
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let mailerlite = MailerLite::builder("your-api-token")
    ///     .request_timeout(Duration::from_secs(15))
    ///     .retry_policy(RetryPolicy::safe_reads())
    ///     .build()?;
    /// # let _ = mailerlite;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder(token: impl Into<String>) -> MailerLiteBuilder {
        MailerLiteBuilder::new(token)
    }
    /// Create an HTTP-client builder with redirects and automatic retries disabled.
    ///
    /// Use this helper when injecting an HTTP client; changing those policies can
    /// undermine the SDK's credential boundaries and write-retry guarantees.
    pub fn http_client_builder() -> reqwest::ClientBuilder {
        reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .connect_timeout(Duration::from_secs(5))
    }
    pub(crate) fn from_builder(
        builder: MailerLiteBuilder,
        base_url: Url,
        authorization: HeaderValue,
    ) -> Result<Self> {
        let http = match builder.http_client {
            Some(client) => client,
            None => Self::http_client_builder()
                .build()
                .map_err(|e| Error::Transport(e.without_url()))?,
        };
        Ok(Self {
            inner: Arc::new(Inner {
                http,
                base_url,
                authorization,
                version: builder.api_version,
                request_timeout: builder.request_timeout,
                operation_timeout: builder.operation_timeout,
                retry: builder.retry_policy,
                limiter: RateLimiter::new(builder.global_interval, builder.import_interval),
                max_response_bytes: builder.max_response_bytes,
            }),
        })
    }
    pub(crate) async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &[&str],
        query: &[(String, String)],
        body: Option<&dyn RequestBody>,
        policy: OperationPolicy,
    ) -> Result<Response<T>> {
        crate::http::request(self, method, path, query, body, policy).await
    }
    pub(crate) async fn data<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &[&str],
        query: &[(String, String)],
        body: Option<&dyn RequestBody>,
        policy: OperationPolicy,
    ) -> Result<Response<T>> {
        self.request::<DataEnvelope<T>>(method, path, query, body, policy)
            .await
            .map(|response| response.map(|envelope| envelope.data))
    }
    pub(crate) async fn empty(
        &self,
        method: Method,
        path: &[&str],
        query: &[(String, String)],
        body: Option<&dyn RequestBody>,
        policy: OperationPolicy,
    ) -> Result<Response<()>> {
        crate::http::empty(self, method, path, query, body, policy).await
    }
}
