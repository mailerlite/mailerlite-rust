//! Reusable HTTP client, shared state, and transport entry points.
use crate::{
    ApiVersion, Error, Response, Result,
    config::MailerLiteBuilder,
    http::{OperationPolicy, rate_limit::RateLimiter, retry::RetryPolicy},
    response::DataEnvelope,
};
use reqwest::{Method, Url, header::HeaderValue};
use serde::{Serialize, de::DeserializeOwned};
use std::{fmt, sync::Arc, time::Duration};

/// Reusable, cheaply cloneable asynchronous MailerLite API client.
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
    pub fn new(token: impl Into<String>) -> Result<Self> {
        Self::builder(token).build()
    }
    /// Configure a client with an explicit bearer token.
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
        body: Option<serde_json::Value>,
        policy: OperationPolicy,
    ) -> Result<Response<T>> {
        crate::http::request(self, method, path, query, body, policy).await
    }
    pub(crate) async fn data<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &[&str],
        query: &[(String, String)],
        body: Option<serde_json::Value>,
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
        body: Option<serde_json::Value>,
        policy: OperationPolicy,
    ) -> Result<Response<()>> {
        crate::http::empty(self, method, path, query, body, policy).await
    }
    pub(crate) fn body<T: Serialize>(value: &T) -> Result<serde_json::Value> {
        serde_json::to_value(value).map_err(Error::Serialize)
    }
}
