//! The [`MailerLite`] client and its builder.

use std::{fmt, sync::Arc, time::Duration};

use reqwest::{
    Url,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderName, HeaderValue, USER_AGENT},
};

use crate::{BASE_URL, Error, Result};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_USER_AGENT: &str = concat!("mailerlite-rs/", env!("CARGO_PKG_VERSION"));

/// Async client for the MailerLite API.
///
/// Cheap to clone (an `Arc` internally) and safe to share between tasks; reuse one instance
/// so connections are pooled. Resource handles such as [`MailerLite::subscribers`] borrow the
/// client and cost nothing to create.
///
/// ```no_run
/// # async fn run() -> mailerlite_rs::Result<()> {
/// use mailerlite_rs::MailerLite;
///
/// let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
/// let groups = client.groups().list(&Default::default()).await?;
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct MailerLite {
    pub(crate) inner: Arc<Inner>,
}

pub(crate) struct Inner {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: Url,
    /// Authorization (marked sensitive), Accept, User-Agent and the optional X-Version.
    pub(crate) headers: HeaderMap,
    /// Per-request timeout, applied on top of the `reqwest::Client`'s own settings.
    pub(crate) timeout: Option<Duration>,
    #[cfg(test)]
    pub(crate) mock: Option<Arc<crate::http::mock::Mock>>,
}

impl MailerLite {
    /// Creates a client with default settings.
    ///
    /// Fails only if the key contains characters that are invalid in an HTTP header.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder(api_key).build()
    }

    /// Starts configuring a client.
    pub fn builder(api_key: impl Into<String>) -> MailerLiteBuilder {
        MailerLiteBuilder {
            api_key: api_key.into(),
            timeout: TimeoutSetting::Default,
            api_version: None,
            user_agent: None,
            http: None,
            #[cfg(test)]
            mock: None,
        }
    }
}

impl fmt::Debug for MailerLite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MailerLite")
            .field("base_url", &self.inner.base_url.as_str())
            .field("timeout", &self.inner.timeout)
            .finish_non_exhaustive()
    }
}

/// Builder for [`MailerLite`].
#[must_use]
pub struct MailerLiteBuilder {
    api_key: String,
    timeout: TimeoutSetting,
    api_version: Option<String>,
    user_agent: Option<String>,
    http: Option<reqwest::Client>,
    #[cfg(test)]
    mock: Option<Arc<crate::http::mock::Mock>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TimeoutSetting {
    /// 30 seconds, unless a caller-supplied client brings its own settings.
    Default,
    Custom(Duration),
    Disabled,
}

impl MailerLiteBuilder {
    /// Total time allowed per request, including reading the body.
    ///
    /// Defaults to 30 seconds. With [`http_client`](Self::http_client), the default is not
    /// applied and your client's own timeout is used, unless you call this explicitly.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = TimeoutSetting::Custom(timeout);
        self
    }

    /// Disables the SDK's per-request timeout. A client passed to
    /// [`http_client`](Self::http_client) keeps whatever timeout it was built with.
    pub fn no_timeout(mut self) -> Self {
        self.timeout = TimeoutSetting::Disabled;
        self
    }

    /// Pins the API version by sending `X-Version: <date>` (e.g. `"2038-01-19"`).
    /// Without it MailerLite serves the latest version.
    pub fn api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = Some(version.into());
        self
    }

    /// Appends `product/version` to the default `mailerlite-rs/x.y.z` user agent so MailerLite
    /// can attribute traffic to your integration.
    pub fn user_agent_suffix(mut self, suffix: impl AsRef<str>) -> Self {
        self.user_agent = Some(format!("{DEFAULT_USER_AGENT} {}", suffix.as_ref()));
        self
    }

    /// Uses your own `reqwest::Client` (e.g. to share a connection pool or set a proxy).
    /// Authentication and other headers are still added to every request.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http = Some(client);
        self
    }

    #[cfg(test)]
    pub(crate) fn mock(mut self, mock: Arc<crate::http::mock::Mock>) -> Self {
        self.mock = Some(mock);
        self
    }

    /// Builds the client.
    pub fn build(self) -> Result<MailerLite> {
        let mut authorization = HeaderValue::try_from(format!("Bearer {}", self.api_key.trim()))
            .map_err(|_| {
                Error::Config("API key contains characters not allowed in an HTTP header".into())
            })?;
        authorization.set_sensitive(true);

        let user_agent = match self.user_agent {
            Some(agent) => HeaderValue::try_from(agent).map_err(|_| {
                Error::Config("user agent suffix is not a valid header value".into())
            })?,
            None => HeaderValue::from_static(DEFAULT_USER_AGENT),
        };

        let mut headers = HeaderMap::with_capacity(4);
        headers.insert(AUTHORIZATION, authorization);
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, user_agent);
        if let Some(version) = self.api_version {
            let version = HeaderValue::try_from(version)
                .map_err(|_| Error::Config("API version is not a valid header value".into()))?;
            headers.insert(HeaderName::from_static("x-version"), version);
        }

        let base_url = Url::parse(BASE_URL)
            .map_err(|err| Error::Config(format!("invalid base URL: {err}")))?;
        let timeout = match (self.timeout, self.http.is_some()) {
            (TimeoutSetting::Custom(timeout), _) => Some(timeout),
            (TimeoutSetting::Default, false) => Some(DEFAULT_TIMEOUT),
            // Respect the caller's client configuration.
            (TimeoutSetting::Default, true) | (TimeoutSetting::Disabled, _) => None,
        };
        let http = match self.http {
            Some(http) => http,
            None => reqwest::Client::builder().build()?,
        };

        Ok(MailerLite {
            inner: Arc::new(Inner {
                http,
                base_url,
                headers,
                timeout,
                #[cfg(test)]
                mock: self.mock,
            }),
        })
    }
}

impl fmt::Debug for MailerLiteBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MailerLiteBuilder")
            .field("api_key", &"<redacted>")
            .field("timeout", &self.timeout)
            .field("api_version", &self.api_version)
            .field("user_agent", &self.user_agent)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_output_hides_api_key() {
        let builder = MailerLite::builder("secret_api_key");
        assert!(!format!("{builder:?}").contains("secret_api_key"));

        let client = builder.build().unwrap();
        assert!(!format!("{client:?}").contains("secret_api_key"));
        assert!(!format!("{:?}", client.inner.headers).contains("secret_api_key"));
    }

    #[test]
    fn timeout_respects_caller_supplied_client() {
        let timeout = |builder: MailerLiteBuilder| builder.build().unwrap().inner.timeout;
        let custom = Duration::from_secs(120);

        assert_eq!(timeout(MailerLite::builder("k")), Some(DEFAULT_TIMEOUT));
        assert_eq!(timeout(MailerLite::builder("k").no_timeout()), None);
        assert_eq!(
            timeout(MailerLite::builder("k").timeout(custom)),
            Some(custom)
        );

        let own = || reqwest::Client::builder().timeout(custom).build().unwrap();
        assert_eq!(timeout(MailerLite::builder("k").http_client(own())), None);
        assert_eq!(
            timeout(
                MailerLite::builder("k")
                    .http_client(own())
                    .timeout(Duration::from_secs(5))
            ),
            Some(Duration::from_secs(5))
        );
    }

    #[test]
    fn rejects_invalid_api_key() {
        let err = MailerLite::new("bad\nkey").unwrap_err();
        assert!(matches!(err, Error::Config(_)));
    }

    #[test]
    fn user_agent_suffix_is_appended() {
        let client = MailerLite::builder("k")
            .user_agent_suffix("my-app/1.0")
            .build()
            .unwrap();
        let agent = client.inner.headers[USER_AGENT].to_str().unwrap();
        assert!(agent.starts_with("mailerlite-rs/") && agent.ends_with(" my-app/1.0"));
    }
}
