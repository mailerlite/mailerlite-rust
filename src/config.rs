//! MailerLite construction and API version selection.
use crate::{Error, MailerLite, Result, http::retry::RetryPolicy, models::SecretString};
use reqwest::{Url, header::HeaderValue};
use std::{fmt, time::Duration};

/// Release reference date available for explicit API version pinning.
///
/// Despite its historical name, this date is not applied by default:
/// [`ApiVersion::Latest`] is the default version policy.
pub const DEFAULT_API_VERSION: &str = "2026-10-30";

/// Whether requests use a pinned date or the server's latest version.
#[derive(Clone, Debug, Default)]
pub enum ApiVersion {
    /// Use a validated `YYYY-MM-DD` version date.
    Pinned(String),
    /// Omit `X-Version` and use the latest server behavior (the default).
    #[default]
    Latest,
}
impl ApiVersion {
    /// Validate a Gregorian date and select it as the API version.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::ApiVersion;
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let version = ApiVersion::pinned("2026-10-30")?;
    /// # let _ = version;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] unless the input is a valid Gregorian date in
    /// `YYYY-MM-DD` format with a nonzero year. Whether the server supports that
    /// version is not checked locally.
    pub fn pinned(date: impl Into<String>) -> Result<Self> {
        let date = date.into();
        validate_date(&date)?;
        Ok(Self::Pinned(date))
    }
}
fn validate_date(date: &str) -> Result<()> {
    let bytes = date.as_bytes();
    let invalid = || Error::Config("API version must be a valid YYYY-MM-DD date".into());
    if bytes.len() != 10
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes
            .iter()
            .enumerate()
            .any(|(i, b)| i != 4 && i != 7 && !b.is_ascii_digit())
    {
        return Err(invalid());
    }
    let year: u32 = date[..4].parse().map_err(|_| invalid())?;
    let month: u32 = date[5..7].parse().map_err(|_| invalid())?;
    let day: u32 = date[8..].parse().map_err(|_| invalid())?;
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => 0,
    };
    if year == 0 || day == 0 || day > days {
        return Err(invalid());
    }
    Ok(())
}

/// Configures a reusable MailerLite client.
///
/// The builder's debug output excludes credentials and injected HTTP-client internals.
#[derive(Clone)]
#[must_use = "client builders do nothing until built; call .build()"]
pub struct MailerLiteBuilder {
    pub(crate) token: SecretString,
    pub(crate) http_client: Option<reqwest::Client>,
    pub(crate) base_url: String,
    pub(crate) api_version: ApiVersion,
    pub(crate) request_timeout: Duration,
    pub(crate) operation_timeout: Duration,
    pub(crate) retry_policy: RetryPolicy,
    pub(crate) global_interval: Duration,
    pub(crate) import_interval: Duration,
    pub(crate) max_response_bytes: usize,
}
impl fmt::Debug for MailerLiteBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MailerLiteBuilder")
            .field("token", &"[REDACTED]")
            .field("api_version", &self.api_version)
            .field("request_timeout", &self.request_timeout)
            .field("operation_timeout", &self.operation_timeout)
            .finish_non_exhaustive()
    }
}
impl MailerLiteBuilder {
    pub(crate) fn new(token: impl Into<String>) -> Self {
        Self {
            token: SecretString::new(token),
            http_client: None,
            base_url: "https://connect.mailerlite.com/api".into(),
            api_version: ApiVersion::default(),
            request_timeout: Duration::from_secs(30),
            operation_timeout: Duration::from_secs(60),
            retry_policy: RetryPolicy::default(),
            global_interval: Duration::from_millis(500),
            import_interval: Duration::from_secs(12),
            max_response_bytes: 16 * 1024 * 1024,
        }
    }
    /// Inject an HTTP client to reuse its connection pool and transport settings.
    ///
    /// The SDK cannot inspect or override its redirect or automatic retry policies.
    /// Build it using [`MailerLite::http_client_builder`] to preserve SDK guarantees.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }
    /// Set a custom API base URL, including its API path prefix.
    ///
    /// HTTP is allowed for local mock servers and explicitly configured proxies.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
    /// Select a pinned API version or the default latest server behavior.
    pub fn api_version(mut self, version: ApiVersion) -> Self {
        self.api_version = version;
        self
    }
    /// Set the timeout for each HTTP attempt, including reading its response.
    /// Defaults to 30 seconds.
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }
    /// Set the deadline from request serialization through response decoding,
    /// including rate-limit waits, HTTP attempts, and retry delays.
    /// Defaults to 60 seconds.
    ///
    /// Synchronous serialization and decoding cannot be interrupted; if either
    /// exceeds the budget, the operation returns [`Error::Timeout`] after that
    /// work finishes. An expired serialization budget prevents HTTP dispatch.
    /// Parameter construction and local validation before dispatch are outside
    /// this deadline. Each page or import-progress request has its own deadline.
    /// A timeout after dispatch does not mean the server rolled back a write.
    pub fn operation_timeout(mut self, timeout: Duration) -> Self {
        self.operation_timeout = timeout;
        self
    }
    /// Configure bounded retries of explicitly classified read operations.
    ///
    /// Defaults to [`RetryPolicy::none`]. Use [`RetryPolicy::safe_reads`] to opt in.
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }
    /// Set local pacing intervals. Zero disables that local pacing bucket.
    ///
    /// Server-directed cooldowns remain enforced. Independent processes do not share this state.
    pub fn rate_limit_intervals(mut self, global: Duration, imports: Duration) -> Self {
        self.global_interval = global;
        self.import_interval = imports;
        self
    }
    /// Bound response body allocation. Defaults to 16 MiB.
    pub fn max_response_bytes(mut self, limit: usize) -> Self {
        self.max_response_bytes = limit;
        self
    }
    /// Validate configuration and build the reusable client.
    ///
    /// No authentication or other API request is performed during construction.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{ApiVersion, MailerLite};
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let mailerlite = MailerLite::builder("your-api-token")
    ///     .api_version(ApiVersion::pinned("2026-10-30")?)
    ///     .build()?;
    /// # let _ = mailerlite;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] for an empty or invalid token header, invalid
    /// version date, zero timeout or response-size limit, or a duration beyond
    /// the supported clock range. The base URL must be HTTP(S), have a host, and
    /// contain no userinfo, query, or fragment; invalid URLs also return
    /// `Error::Config`. A zero pacing interval is allowed.
    ///
    /// Returns [`Error::Transport`] if creating the default HTTP client fails.
    /// An injected client's redirect, retry, and proxy policies are not validated;
    /// see [`MailerLite::http_client_builder`].
    pub fn build(self) -> Result<MailerLite> {
        if self.token.expose_secret().is_empty() {
            return Err(Error::Config("API token must not be empty".into()));
        }
        if self.request_timeout.is_zero()
            || self.operation_timeout.is_zero()
            || self.max_response_bytes == 0
        {
            return Err(Error::Config(
                "timeouts and response size limit must be nonzero".into(),
            ));
        }
        let now = std::time::Instant::now();
        if [
            self.request_timeout,
            self.operation_timeout,
            self.global_interval,
            self.import_interval,
        ]
        .into_iter()
        .any(|duration| now.checked_add(duration).is_none())
        {
            return Err(Error::Config(
                "configured duration exceeds the supported clock range".into(),
            ));
        }
        if let ApiVersion::Pinned(ref date) = self.api_version {
            validate_date(date)?;
        }
        let mut url =
            Url::parse(&self.base_url).map_err(|_| Error::Config("invalid API base URL".into()))?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(Error::Config(
                "base URL must be HTTP(S) without userinfo, query, or fragment".into(),
            ));
        }
        let path = format!("{}/", url.path().trim_end_matches('/'));
        url.set_path(&path);
        let mut authorization =
            HeaderValue::from_str(&format!("Bearer {}", self.token.expose_secret())).map_err(
                |_| Error::Config("API token cannot be represented as an HTTP header".into()),
            )?;
        authorization.set_sensitive(true);
        MailerLite::from_builder(self, url, authorization)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_constructor_defaults_to_latest_version() {
        let client = MailerLite::new("test-token").unwrap();
        assert!(matches!(client.inner.version, ApiVersion::Latest));
    }

    #[test]
    fn configuration_is_fallible_and_redacted() {
        assert!(
            MailerLite::builder("secret")
                .request_timeout(Duration::MAX)
                .build()
                .is_err()
        );
        assert!(MailerLite::new("bad\r\ntoken").is_err());
        assert!(
            MailerLite::builder("secret")
                .base_url("https://user:pass@example.test/api")
                .build()
                .is_err()
        );
        assert!(ApiVersion::pinned("2025-02-29").is_err());
        assert!(ApiVersion::pinned("2024-02-29").is_ok());
        assert!(!format!("{:?}", MailerLite::builder("private-token")).contains("private-token"));
    }
}
