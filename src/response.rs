//! Successful responses and HTTP metadata.
use serde::Deserialize;
use std::time::Duration;
/// A decoded API result with its HTTP metadata.
#[derive(Debug, Clone)]
pub struct Response<T> {
    /// Endpoint-specific response data.
    pub data: T,
    /// HTTP status and rate-limit information.
    pub metadata: ResponseMetadata,
}
impl<T> Response<T> {
    /// Transform response data, retaining metadata.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Response<U> {
        Response {
            data: f(self.data),
            metadata: self.metadata,
        }
    }
}
/// Selected response headers.
#[derive(Debug, Clone)]
pub struct ResponseMetadata {
    /// HTTP status.
    pub status: reqwest::StatusCode,
    /// Server-reported limit.
    pub rate_limit: Option<u64>,
    /// Remaining requests.
    pub rate_remaining: Option<u64>,
    /// Minimum retry delay.
    pub retry_after: Option<Duration>,
    /// Server correlation identifier.
    pub request_id: Option<String>,
}
impl ResponseMetadata {
    /// Server-directed retry delay.
    pub fn retry_after(&self) -> Option<Duration> {
        self.retry_after
    }
}
#[derive(Deserialize)]
pub(crate) struct DataEnvelope<T> {
    pub data: T,
}
/// Response when querying an endpoint with `limit=0`.
#[derive(Debug, Clone, Deserialize)]
pub struct Count {
    /// Total matching resources.
    pub total: u64,
}
