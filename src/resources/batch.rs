//! Submit bounded batches; write batches are never automatically replayed.
use crate::{
    Error, MailerLite, Response, Result, http::OperationPolicy, models::batch::BatchResult,
};
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;
use std::fmt;

/// An allowed HTTP method within a batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BatchMethod {
    /// Read a resource.
    Get,
    /// Create or invoke an operation.
    Post,
    /// Update a resource.
    Put,
    /// Partially update a resource.
    Patch,
    /// Delete a resource.
    Delete,
}
/// A validated API-relative batch operation.
#[derive(Clone, Serialize)]
pub struct BatchRequest {
    method: BatchMethod,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Value>,
}
impl BatchRequest {
    /// Create an operation with a path beginning `api/`.
    ///
    /// Absolute URLs, traversal, encoded separators, fragments, and webhooks
    /// are rejected. Percent-encoding in the query is allowed.
    ///
    /// This validates an API-relative path without sending a request. Endpoint-specific
    /// permissions and payload validation are still the server's responsibility.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::batch::{BatchMethod, BatchRequest};
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let request = BatchRequest::new(BatchMethod::Get, "api/groups?limit=25")?;
    /// # let _ = request;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] for paths without the `api/` prefix, empty or dot
    /// segments, route percent encoding/backslashes/colons, fragments, control
    /// characters, or webhook-management routes. Percent encoding in queries is allowed.
    pub fn new(method: BatchMethod, path: impl Into<String>) -> Result<Self> {
        let path = path.into();
        let route = path.split('?').next().unwrap_or_default();
        if !route.starts_with("api/")
            || route.contains(['%', '\\', '#', ':'])
            || path.contains('#')
            || path.chars().any(char::is_control)
            || route
                .split('/')
                .any(|segment| segment.is_empty() || segment == "." || segment == "..")
            || route == "api/webhooks"
            || route.starts_with("api/webhooks/")
        {
            return Err(Error::Config(
                "batch path must be a safe api/ route; webhooks are unsupported".into(),
            ));
        }
        Ok(Self {
            method,
            path,
            body: None,
        })
    }
    /// Add an object body; arrays and scalar bodies are not valid batch bodies.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::batch::{BatchMethod, BatchRequest};
    /// use serde_json::json;
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let request = BatchRequest::new(BatchMethod::Post, "api/groups")?
    ///     .body(json!({"name": "Newsletter readers"}))?;
    /// # let _ = request;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Serialize`] when the supplied value cannot be serialized, or
    /// [`Error::Config`] when it serializes to an array, scalar, or null rather than an
    /// object. This validates only the outer shape, not endpoint-specific fields.
    pub fn body(mut self, value: impl Serialize) -> Result<Self> {
        let value = serde_json::to_value(&value).map_err(Error::Serialize)?;
        if !value.is_object() {
            return Err(Error::Config("batch body must be a JSON object".into()));
        }
        self.body = Some(value);
        Ok(self)
    }
}
impl fmt::Debug for BatchRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BatchRequest")
            .field("method", &self.method)
            .finish_non_exhaustive()
    }
}
/// Batch endpoint operations.
#[derive(Debug, Clone, Copy)]
pub struct Batch<'a> {
    client: &'a MailerLite,
}
impl<'a> Batch<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// Submit one through fifty operations and retain every individual result.
    ///
    /// Batches containing only subscriber upserts share the subscriber-import
    /// pacing budget with group imports. Other batches use the global budget.
    /// No batch is automatically replayed, including a batch of read operations.
    ///
    /// Outer HTTP success does not imply that every operation succeeded. Inspect each
    /// item's status before decoding its body. Do not assume that failed items roll
    /// back successful ones.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::{
    ///     models::groups::Group,
    ///     pagination::NumberedPage,
    ///     resources::batch::{BatchMethod, BatchRequest},
    /// };
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.batch().send(vec![
    ///     BatchRequest::new(BatchMethod::Get, "api/groups?limit=25")?,
    /// ]).await?;
    ///
    /// for item in response.data.responses {
    ///     if item.is_success() {
    ///         let page: NumberedPage<Group> = item.decode()?;
    ///
    ///         println!("{} groups", page.data.len());
    ///     } else {
    ///         eprintln!("Batch operation failed: HTTP {}", item.code);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] unless there are 1..=50 operations, and
    /// [`Error::Serialize`] if the batch cannot be serialized. Server rejection returns
    /// [`crate::Error::Api`]. HTTP execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    /// Individual failed items remain in an otherwise successful [`BatchResult`];
    /// they are not automatically converted into a top-level `Error::Api`. A timeout
    /// can leave some operations applied, so inspect outcomes before resubmitting.
    pub async fn send(&self, requests: Vec<BatchRequest>) -> Result<Response<BatchResult>> {
        if requests.is_empty() || requests.len() > 50 {
            return Err(Error::Config(
                "batch must contain 1 through 50 operations".into(),
            ));
        }
        let policy = batch_policy(&requests);
        #[derive(Serialize)]
        struct Payload {
            requests: Vec<BatchRequest>,
        }
        self.client
            .request(
                Method::POST,
                &["batch"],
                &[],
                Some(&Payload { requests }),
                policy.with_success_statuses(&[200]),
            )
            .await
    }
}

fn batch_policy(requests: &[BatchRequest]) -> OperationPolicy {
    if requests.iter().all(|request| {
        request.method == BatchMethod::Post
            && request.path.split('?').next() == Some("api/subscribers")
    }) {
        OperationPolicy::IMPORT
    } else {
        OperationPolicy::WRITE
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use super::*;
    use crate::http::rate_limit::RateLimiter;
    use serde_json::json;
    use std::time::Duration;
    use tokio::time::Instant;

    fn request(method: BatchMethod, path: &str) -> BatchRequest {
        BatchRequest::new(method, path).unwrap()
    }

    #[test]
    fn subscriber_upsert_batch_with_query_uses_import_policy_without_retries() {
        let policy = batch_policy(&[
            request(BatchMethod::Post, "api/subscribers"),
            request(BatchMethod::Post, "api/subscribers?"),
        ]);
        assert!(policy.import);
        assert!(!policy.read_only);
    }

    #[test]
    fn subscriber_update_batch_uses_global_policy_without_retries() {
        let policy = batch_policy(&[request(BatchMethod::Put, "api/subscribers/42")]);
        assert!(!policy.import);
        assert!(!policy.read_only);
    }

    #[test]
    fn subscriber_subresource_is_not_classified_as_an_upsert() {
        let policy = batch_policy(&[request(BatchMethod::Post, "api/subscribers/42/forget")]);
        assert!(!policy.import);
        assert!(!policy.read_only);
    }

    #[test]
    fn read_only_batch_still_has_non_retrying_global_policy() {
        let policy = batch_policy(&[request(BatchMethod::Get, "api/subscribers?limit=1")]);
        assert!(!policy.import);
        assert!(!policy.read_only);
    }

    #[tokio::test(start_paused = true)]
    async fn mixed_batch_does_not_consume_the_subscriber_import_bucket() {
        let mixed = batch_policy(&[
            request(BatchMethod::Post, "api/subscribers"),
            request(BatchMethod::Get, "api/groups"),
        ]);
        let limiter = RateLimiter::new(Duration::ZERO, Duration::from_secs(12));
        let start = Instant::now();
        limiter.wait(mixed.import).await;
        limiter.wait(OperationPolicy::IMPORT.import).await;
        assert_eq!(start.elapsed(), Duration::ZERO);
        assert!(!mixed.read_only);
    }

    #[tokio::test(start_paused = true)]
    async fn subscriber_batches_observe_the_same_import_interval() {
        let first = batch_policy(&[request(BatchMethod::Post, "api/subscribers")]);
        let second = batch_policy(&[request(BatchMethod::Post, "api/subscribers?")]);
        let limiter = RateLimiter::new(Duration::ZERO, Duration::from_secs(12));
        let start = Instant::now();
        limiter.wait(first.import).await;
        limiter.wait(second.import).await;
        assert_eq!(start.elapsed(), Duration::from_secs(12));
    }

    #[test]
    fn request_rejects_absolute_urls() {
        assert!(BatchRequest::new(BatchMethod::Get, "https://evil.test/api/groups").is_err());
    }

    #[test]
    fn request_rejects_parent_segments() {
        assert!(BatchRequest::new(BatchMethod::Get, "api/../webhooks").is_err());
    }

    #[test]
    fn request_rejects_encoded_parent_segments() {
        assert!(BatchRequest::new(BatchMethod::Get, "api/%2e%2e/groups").is_err());
    }

    #[test]
    fn request_rejects_webhook_collection() {
        assert!(BatchRequest::new(BatchMethod::Get, "api/webhooks").is_err());
    }

    #[test]
    fn request_rejects_webhook_items() {
        assert!(BatchRequest::new(BatchMethod::Get, "api/webhooks/42").is_err());
    }

    #[test]
    fn request_rejects_empty_path_segments() {
        assert!(BatchRequest::new(BatchMethod::Get, "api//groups").is_err());
    }

    #[test]
    fn request_rejects_fragments() {
        assert!(BatchRequest::new(BatchMethod::Get, "api/groups#fragment").is_err());
    }

    #[test]
    fn request_rejects_array_body() {
        assert!(
            BatchRequest::new(BatchMethod::Post, "api/groups")
                .unwrap()
                .body(json!([]))
                .is_err()
        );
    }

    #[test]
    fn request_rejects_null_body() {
        assert!(
            BatchRequest::new(BatchMethod::Post, "api/groups")
                .unwrap()
                .body(json!(null))
                .is_err()
        );
    }

    #[test]
    fn request_rejects_boolean_body() {
        assert!(
            BatchRequest::new(BatchMethod::Post, "api/groups")
                .unwrap()
                .body(json!(true))
                .is_err()
        );
    }

    #[test]
    fn request_rejects_number_body() {
        assert!(
            BatchRequest::new(BatchMethod::Post, "api/groups")
                .unwrap()
                .body(json!(1))
                .is_err()
        );
    }

    #[test]
    fn request_rejects_string_body() {
        assert!(
            BatchRequest::new(BatchMethod::Post, "api/groups")
                .unwrap()
                .body(json!("text"))
                .is_err()
        );
    }
}
