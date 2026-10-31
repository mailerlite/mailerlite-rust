//! Group subscriber imports and explicit bounded progress polling.
use crate::{
    Error, MailerLite, Response, Result,
    http::OperationPolicy,
    models::{
        ImportId,
        imports::{ImportReceipt, ImportReport},
    },
};
use reqwest::{Method, Url};
use serde::Serialize;
use std::{collections::BTreeMap, fmt, time::Duration};

/// Scalar custom-field value for a subscriber import.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ImportFieldValue {
    /// Text field value.
    Text(String),
    /// Numeric field value, retained without conversion through floating point.
    Number(serde_json::Number),
}
impl From<String> for ImportFieldValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl From<&str> for ImportFieldValue {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}
impl From<i64> for ImportFieldValue {
    fn from(value: i64) -> Self {
        Self::Number(value.into())
    }
}
/// A subscriber to import into a group.
#[derive(Clone, Serialize)]
pub struct ImportSubscriber {
    email: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    fields: BTreeMap<String, ImportFieldValue>,
}
impl ImportSubscriber {
    /// Set the required email address.
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            fields: BTreeMap::new(),
        }
    }
    /// Add a named default or custom field. Omitted fields remain unchanged.
    pub fn field(mut self, name: impl Into<String>, value: impl Into<ImportFieldValue>) -> Self {
        self.fields.insert(name.into(), value.into());
        self
    }
}
impl fmt::Debug for ImportSubscriber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImportSubscriber").finish_non_exhaustive()
    }
}
/// Import creation and reporting operations.
#[derive(Debug, Clone, Copy)]
pub struct Imports<'a> {
    client: &'a MailerLite,
}
impl<'a> Imports<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// Start a group import; this write is never automatically retried.
    ///
    /// Success returns an acceptance receipt, not a completed import. Use
    /// [`Self::progress`] for one status check or [`Self::wait`] for explicit
    /// polling. Inspect the final report's outcome counts as well as `done`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{MailerLite, resources::imports::ImportSubscriber};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let receipt = mailerlite.imports().import_to_group(
    ///     "123456789",
    ///     vec![ImportSubscriber::new("jane@example.com").field("name", "Jane")],
    /// ).await?;
    ///
    /// let report = mailerlite.imports().progress(&receipt.data).await?;
    ///
    /// println!("Completed: {}", report.data.done);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] before dispatch for an empty subscriber list or
    /// an empty/dot group path segment. Serialization failures return
    /// [`Error::Serialize`]; API rejection, transport failure, malformed responses,
    /// and execution deadlines return [`Error::Api`], [`Error::Transport`],
    /// [`Error::Decode`], or [`Error::Timeout`].
    ///
    /// A returned receipt with an invalid progress URL also produces
    /// [`Error::Config`], but the server may already have accepted the import.
    /// Likewise, a timeout does not cancel server work; check the outcome before
    /// submitting the import again.
    pub async fn import_to_group(
        &self,
        group: impl AsRef<str>,
        subscribers: Vec<ImportSubscriber>,
    ) -> Result<Response<ImportReceipt>> {
        if subscribers.is_empty() {
            return Err(Error::Config(
                "an import requires at least one subscriber".into(),
            ));
        }
        #[derive(Serialize)]
        struct Payload {
            subscribers: Vec<ImportSubscriber>,
        }
        let response: Response<ImportReceipt> = self
            .client
            .request(
                Method::POST,
                &["groups", group.as_ref(), "import-subscribers"],
                &[],
                Some(&Payload { subscribers }),
                OperationPolicy::IMPORT.with_success_statuses(&[200]),
            )
            .await?;
        self.receipt_id(&response.data)?;
        Ok(response)
    }
    /// Fetch a report by import identifier.
    ///
    /// This performs one status lookup rather than polling. Opt-in read retries
    /// may make multiple HTTP attempts. It does not wait for `done` to become true.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let report = mailerlite.imports().get("123456789").await?;
    ///
    /// println!("Processed {} of {}", report.data.processed, report.data.total);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] for an empty or dot path segment. Rejected
    /// requests return [`Error::Api`]; HTTP execution may also return
    /// [`Error::Transport`], [`Error::Decode`], or [`Error::Timeout`]. An unfinished
    /// report is a successful result, not an error.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<ImportReport>> {
        self.client
            .data(
                Method::GET,
                &["subscribers", "import", id.as_ref()],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Validate a receipt's origin and exact endpoint, then fetch its report.
    ///
    /// The URL must identify the subscriber-import endpoint under the configured
    /// base URL. The SDK extracts its ID and builds its own request instead of
    /// following the URL directly.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{MailerLite, models::imports::ImportReceipt};
    ///
    /// # async fn example(mailerlite: &MailerLite, receipt: &ImportReceipt)
    /// #     -> mailerlite_rs::Result<()> {
    /// let report = mailerlite.imports().progress(receipt).await?;
    ///
    /// println!("Completed: {}", report.data.done);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] without a request if the URL is malformed,
    /// references another origin or endpoint, contains userinfo, a query, or a
    /// fragment, or lacks a nonempty digit-only import ID. After validation,
    /// returns the same HTTP errors as [`Self::get`].
    pub async fn progress(&self, receipt: &ImportReceipt) -> Result<Response<ImportReport>> {
        self.get(self.receipt_id(receipt)?).await
    }
    /// Poll until done or the explicit overall timeout expires.
    ///
    /// Fetches sequentially and observes the shared client rate limiter. Dropping
    /// this future cancels further polling. A stopped import without `done=true`
    /// continues until the supplied deadline; inspect reports manually if needed.
    /// Canceling polling does not cancel the server-side import. Completion means
    /// `done` is true, not that every subscriber was imported successfully.
    /// A failed status request ends polling; this method does not restart it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{MailerLite, models::imports::ImportReceipt};
    /// use std::time::Duration;
    ///
    /// # async fn example(mailerlite: &MailerLite, receipt: &ImportReceipt)
    /// #     -> mailerlite_rs::Result<()> {
    /// let report = mailerlite.imports().wait(
    ///     receipt,
    ///     Duration::from_secs(5),
    ///     Duration::from_secs(300),
    /// ).await?;
    ///
    /// println!("Imported: {}; rejected: {}", report.data.imported, report.data.errored);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] for a zero interval or timeout, a duration beyond
    /// the supported clock range, or an invalid receipt as described by
    /// [`Self::progress`]. Returns [`Error::Timeout`] if polling reaches its
    /// deadline, including when a fetch finishes at or after it. Synchronous
    /// response decoding cannot be interrupted; its deadline is checked after
    /// it finishes. Errors from individual [`Self::get`] calls propagate when
    /// they arrive before the polling deadline, including their own execution
    /// timeouts. Retain the receipt to check the import later after any failure.
    pub async fn wait(
        &self,
        receipt: &ImportReceipt,
        interval: Duration,
        timeout: Duration,
    ) -> Result<Response<ImportReport>> {
        if interval.is_zero() || timeout.is_zero() {
            return Err(Error::Config(
                "poll interval and timeout must be positive".into(),
            ));
        }
        let now = tokio::time::Instant::now();
        let deadline = now.checked_add(timeout).ok_or_else(|| {
            Error::Config("polling timeout exceeds the supported clock range".into())
        })?;
        if now.checked_add(interval).is_none() {
            return Err(Error::Config(
                "polling interval exceeds the supported clock range".into(),
            ));
        }
        let id = self.receipt_id(receipt)?;
        poll_until_complete(|| self.get(&id), interval, deadline).await
    }
    fn receipt_id(&self, receipt: &ImportReceipt) -> Result<ImportId> {
        let invalid = || {
            Error::Config("import receipt must reference the configured origin and subscriber import endpoint".into())
        };
        let url = Url::parse(&receipt.import_progress_url).map_err(|_| invalid())?;
        let base = &self.client.inner.base_url;
        if url.origin() != base.origin()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(invalid());
        }
        let prefix = format!("{}/subscribers/import/", base.path().trim_end_matches('/'));
        let id = url.path().strip_prefix(&prefix).ok_or_else(invalid)?;
        if id.is_empty() || !id.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(invalid());
        }
        Ok(ImportId::new(id))
    }
}

// Keep the polling lifetime explicit: the deadline covers both fetches and
// intervals, and dropping this future drops the active fetch or sleep with it.
async fn poll_until_complete<F, Fut>(
    mut fetch: F,
    interval: Duration,
    deadline: tokio::time::Instant,
) -> Result<Response<ImportReport>>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<Response<ImportReport>>>,
{
    tokio::time::timeout_at(deadline, async {
        loop {
            if tokio::time::Instant::now() >= deadline {
                return Err(Error::Timeout);
            }
            let result = fetch().await;
            // A ready fetch can finish without yielding to the timeout timer.
            // Check before returning either a late report or a late error.
            if tokio::time::Instant::now() >= deadline {
                return Err(Error::Timeout);
            }
            let report = result?;
            if report.data.done {
                return Ok(report);
            }
            tokio::time::sleep(interval).await;
        }
    })
    .await
    .map_err(|_| Error::Timeout)?
}

#[cfg(test)]
mod polling_tests {
    use super::*;
    use crate::{ApiError, ResponseMetadata};
    use std::{
        future::{Future, pending, poll_fn},
        pin::Pin,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
        task::Poll,
    };
    use tokio::time::{Instant, advance};

    fn metadata(status: reqwest::StatusCode) -> ResponseMetadata {
        ResponseMetadata {
            status,
            rate_limit: None,
            rate_remaining: None,
            retry_after: None,
            request_id: None,
        }
    }

    fn report(done: bool) -> Response<ImportReport> {
        Response {
            data: serde_json::from_value(serde_json::json!({
                "id":"001", "total":1, "processed":u8::from(done),
                "imported":u8::from(done), "updated":0, "errored":0,
                "percent":if done { 100 } else { 0 }, "done":done
            }))
            .unwrap(),
            metadata: metadata(reqwest::StatusCode::OK),
        }
    }

    async fn poll_once<F: Future>(mut future: Pin<&mut F>) -> Poll<F::Output> {
        poll_fn(|context| Poll::Ready(future.as_mut().poll(context))).await
    }

    #[tokio::test(start_paused = true)]
    async fn expired_polling_deadline_prevents_the_first_fetch() {
        let calls = AtomicUsize::new(0);
        let deadline = Instant::now();
        let result = poll_until_complete(
            || {
                calls.fetch_add(1, Ordering::SeqCst);
                std::future::ready(Ok(report(true)))
            },
            Duration::from_secs(1),
            deadline,
        )
        .await;

        assert!(matches!(result, Err(Error::Timeout)));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    async fn assert_fetch_deadline(outcome: Result<Response<ImportReport>>, elapsed: Duration) {
        let calls = AtomicUsize::new(0);
        let start = Instant::now();
        let mut outcome = Some(outcome);
        let result = poll_until_complete(
            || {
                calls.fetch_add(1, Ordering::SeqCst);
                let outcome = outcome.take().expect("only one fetch should start");
                async move {
                    // Model a fetch that is ready when execution resumes at or
                    // after the deadline, without relying on wall-clock sleeps.
                    advance(elapsed).await;
                    outcome
                }
            },
            Duration::from_secs(1),
            start + Duration::from_secs(1),
        )
        .await;

        assert!(matches!(result, Err(Error::Timeout)), "got {result:?}");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(start.elapsed(), elapsed);
    }

    #[tokio::test(start_paused = true)]
    async fn completed_fetch_after_polling_deadline_returns_timeout() {
        assert_fetch_deadline(Ok(report(true)), Duration::from_secs(2)).await;
    }

    #[tokio::test(start_paused = true)]
    async fn completed_fetch_at_polling_deadline_returns_timeout() {
        assert_fetch_deadline(Ok(report(true)), Duration::from_secs(1)).await;
    }

    #[tokio::test(start_paused = true)]
    async fn failed_fetch_after_polling_deadline_returns_timeout() {
        let error = Error::Decode(crate::DecodeError::new(
            metadata(reqwest::StatusCode::OK),
            b"invalid response",
            "invalid test response".into(),
        ));
        assert_fetch_deadline(Err(error), Duration::from_secs(2)).await;
    }

    #[tokio::test(start_paused = true)]
    async fn unfinished_fetch_after_polling_deadline_does_not_poll_again() {
        assert_fetch_deadline(Ok(report(false)), Duration::from_secs(2)).await;
    }

    #[tokio::test(start_paused = true)]
    async fn overall_deadline_expires_during_interval_without_fetching_again() {
        let calls = AtomicUsize::new(0);
        let start = Instant::now();
        let result = poll_until_complete(
            || {
                calls.fetch_add(1, Ordering::SeqCst);
                std::future::ready(Ok(report(false)))
            },
            Duration::from_secs(30),
            start + Duration::from_secs(10),
        )
        .await;
        assert!(matches!(result, Err(Error::Timeout)));
        assert_eq!(start.elapsed(), Duration::from_secs(10));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn overall_deadline_expires_during_an_in_flight_fetch() {
        let calls = AtomicUsize::new(0);
        let dropped = Arc::new(AtomicUsize::new(0));
        let start = Instant::now();
        let result = poll_until_complete(
            || {
                calls.fetch_add(1, Ordering::SeqCst);
                let dropped = Arc::clone(&dropped);
                async move {
                    let _guard = DropProbe(dropped);
                    pending().await
                }
            },
            Duration::from_secs(1),
            start + Duration::from_secs(10),
        )
        .await;
        assert!(matches!(result, Err(Error::Timeout)));
        assert_eq!(start.elapsed(), Duration::from_secs(10));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(dropped.load(Ordering::SeqCst), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn completion_stops_polling_after_the_exact_interval() {
        let calls = AtomicUsize::new(0);
        let start = Instant::now();
        let result = poll_until_complete(
            || {
                let call = calls.fetch_add(1, Ordering::SeqCst);
                std::future::ready(Ok(report(call > 0)))
            },
            Duration::from_secs(5),
            start + Duration::from_secs(60),
        )
        .await
        .unwrap();
        assert!(result.data.done);
        assert_eq!(start.elapsed(), Duration::from_secs(5));
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        advance(Duration::from_secs(120)).await;
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn fetch_error_returns_immediately_without_another_poll() {
        let calls = AtomicUsize::new(0);
        let start = Instant::now();
        let result = poll_until_complete(
            || {
                calls.fetch_add(1, Ordering::SeqCst);
                std::future::ready(Err(Error::Api(ApiError::new(
                    metadata(reqwest::StatusCode::NOT_FOUND),
                    br#"{"message":"Import not found"}"#,
                ))))
            },
            Duration::from_secs(5),
            start + Duration::from_secs(60),
        )
        .await;
        let Error::Api(error) = result.unwrap_err() else {
            panic!("expected original API error");
        };
        assert_eq!(error.status(), reqwest::StatusCode::NOT_FOUND);
        assert_eq!(error.message(), Some("Import not found"));
        assert_eq!(start.elapsed(), Duration::ZERO);
        advance(Duration::from_secs(120)).await;
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn dropping_during_interval_prevents_future_polls() {
        let calls = AtomicUsize::new(0);
        let mut polling = Box::pin(poll_until_complete(
            || {
                calls.fetch_add(1, Ordering::SeqCst);
                std::future::ready(Ok(report(false)))
            },
            Duration::from_secs(5),
            Instant::now() + Duration::from_secs(60),
        ));
        assert!(poll_once(polling.as_mut()).await.is_pending());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        drop(polling);
        advance(Duration::from_secs(120)).await;
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    struct DropProbe(Arc<AtomicUsize>);
    impl Drop for DropProbe {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn dropping_during_fetch_cancels_that_fetch_and_prevents_future_polls() {
        let calls = AtomicUsize::new(0);
        let dropped = Arc::new(AtomicUsize::new(0));
        let mut polling = Box::pin(poll_until_complete(
            || {
                calls.fetch_add(1, Ordering::SeqCst);
                let dropped = Arc::clone(&dropped);
                async move {
                    let _guard = DropProbe(dropped);
                    pending().await
                }
            },
            Duration::from_secs(5),
            Instant::now() + Duration::from_secs(60),
        ));
        assert!(poll_once(polling.as_mut()).await.is_pending());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(dropped.load(Ordering::SeqCst), 0);
        drop(polling);
        assert_eq!(dropped.load(Ordering::SeqCst), 1);
        advance(Duration::from_secs(120)).await;
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
