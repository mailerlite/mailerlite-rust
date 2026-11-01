//! Successful responses and HTTP metadata.
use serde::Deserialize;
use std::time::Duration;
/// A decoded API result with its HTTP metadata.
///
/// Metadata defaults to [`ResponseMetadata`]. New endpoint families can provide
/// additional typed metadata while `Response<T>` continues to name the standard metadata type.
///
/// When constructing a response with a conversion such as `metadata.into()`, give
/// the response an explicit type (`let response: Response<T> = ...`). The metadata
/// type cannot always be inferred from an unannotated struct literal; the default
/// type parameter does not resolve an otherwise ambiguous conversion.
#[derive(Debug, Clone)]
pub struct Response<T, M = ResponseMetadata> {
    /// Endpoint-specific response data.
    pub data: T,
    /// HTTP status and rate-limit information.
    pub metadata: M,
}
impl<T, M> Response<T, M> {
    /// Transform response data, retaining metadata.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Response<U, M> {
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
