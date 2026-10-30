//! Error types returned by every fallible SDK call.

use std::{collections::BTreeMap, fmt, time::Duration};

use reqwest::{StatusCode, header::HeaderMap};
use serde::Deserialize;

/// Result alias used throughout the crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Anything that can go wrong while talking to MailerLite.
///
/// The SDK never panics on network, HTTP or decoding failures; they all surface here.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// MailerLite answered with a non-2xx status code.
    #[error(transparent)]
    Api(Box<ApiError>),

    /// The request could not be sent or the response could not be read
    /// (DNS, TLS, connection reset, timeout, ...).
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// MailerLite answered 2xx but the body did not match the expected shape.
    #[error("failed to decode MailerLite response: {source}")]
    Decode {
        /// Underlying `serde_json` error.
        #[source]
        source: serde_json::Error,
        /// The start of the offending body (truncated to 1 KiB), for debugging.
        body: String,
    },

    /// The client could not be built, e.g. the API key contains characters that are not
    /// valid in an HTTP header.
    #[error("invalid client configuration: {0}")]
    Config(String),
}

impl Error {
    /// The API error, if MailerLite returned a non-2xx response.
    #[must_use]
    pub fn as_api(&self) -> Option<&ApiError> {
        match self {
            Self::Api(err) => Some(err),
            _ => None,
        }
    }

    /// HTTP status of the API error, if any.
    #[must_use]
    pub fn status(&self) -> Option<StatusCode> {
        self.as_api().map(|err| err.status)
    }

    /// `true` for `404 Not Found`.
    #[must_use]
    pub fn is_not_found(&self) -> bool {
        self.as_api().is_some_and(ApiError::is_not_found)
    }

    /// `true` for `422 Unprocessable Entity` (validation failed).
    #[must_use]
    pub fn is_validation(&self) -> bool {
        self.as_api().is_some_and(ApiError::is_validation)
    }

    /// `true` for `429 Too Many Requests`. See [`ApiError::retry_after`].
    #[must_use]
    pub fn is_rate_limited(&self) -> bool {
        self.as_api().is_some_and(ApiError::is_rate_limited)
    }

    pub(crate) fn decode(source: serde_json::Error, body: &[u8]) -> Self {
        Self::Decode {
            source,
            body: truncate(body, 1024),
        }
    }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Self {
        Self::Api(Box::new(err))
    }
}

/// A non-2xx response from MailerLite.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub struct ApiError {
    /// HTTP status code.
    pub status: StatusCode,
    /// Human readable message from the API (falls back to the status' canonical reason).
    pub message: String,
    /// Per-field validation messages on `422`, keyed by field path (e.g. `email`,
    /// `emails.0.subject`, `requests.3.path`).
    pub errors: BTreeMap<String, Vec<String>>,
    /// How long to wait before retrying, from the `Retry-After` header (set on `429`).
    pub retry_after: Option<Duration>,
    /// Requests left in the current window, from `X-RateLimit-Remaining`.
    pub rate_limit_remaining: Option<u32>,
}

impl ApiError {
    /// `true` for `401 Unauthorized` (missing or invalid API key).
    #[must_use]
    pub fn is_unauthorized(&self) -> bool {
        self.status == StatusCode::UNAUTHORIZED
    }

    /// `true` for `404 Not Found`.
    #[must_use]
    pub fn is_not_found(&self) -> bool {
        self.status == StatusCode::NOT_FOUND
    }

    /// `true` for `422 Unprocessable Entity`.
    #[must_use]
    pub fn is_validation(&self) -> bool {
        self.status == StatusCode::UNPROCESSABLE_ENTITY
    }

    /// `true` for `429 Too Many Requests`.
    #[must_use]
    pub fn is_rate_limited(&self) -> bool {
        self.status == StatusCode::TOO_MANY_REQUESTS
    }

    /// `true` for `5xx` and `408`, which MailerLite documents as errors on its side.
    #[must_use]
    pub fn is_server_error(&self) -> bool {
        self.status.is_server_error() || self.status == StatusCode::REQUEST_TIMEOUT
    }

    pub(crate) fn from_parts(status: StatusCode, headers: &HeaderMap, body: &[u8]) -> Self {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Body {
            Object {
                #[serde(default)]
                message: Option<String>,
                #[serde(default)]
                errors: Option<BTreeMap<String, Vec<String>>>,
            },
            // Some campaign endpoints answer with a bare JSON string.
            Message(String),
        }

        let (message, errors) = match serde_json::from_slice::<Body>(body) {
            Ok(Body::Object { message, errors }) => (message, errors.unwrap_or_default()),
            Ok(Body::Message(message)) => (Some(message), BTreeMap::new()),
            Err(_) if body.is_empty() => (None, BTreeMap::new()),
            Err(_) => (Some(truncate(body, 512)), BTreeMap::new()),
        };

        let message = message
            .filter(|message| !message.is_empty())
            .or_else(|| status.canonical_reason().map(str::to_owned))
            .unwrap_or_default();

        Self {
            status,
            message,
            errors,
            retry_after: header_u64(headers, "retry-after").map(Duration::from_secs),
            rate_limit_remaining: header_u64(headers, "x-ratelimit-remaining")
                .and_then(|value| u32::try_from(value).ok()),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MailerLite API error {}: {}", self.status, self.message)?;
        for (field, messages) in &self.errors {
            write!(f, "; {field}: {}", messages.join(", "))?;
        }
        Ok(())
    }
}

fn header_u64(headers: &HeaderMap, name: &str) -> Option<u64> {
    headers.get(name)?.to_str().ok()?.trim().parse().ok()
}

fn truncate(body: &[u8], max: usize) -> String {
    let text = String::from_utf8_lossy(body);
    match text.char_indices().nth(max) {
        Some((idx, _)) => format!("{}…", &text[..idx]),
        None => text.into_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    #[test]
    fn parses_validation_errors() {
        let body = br#"{"message":"The given data was invalid.","errors":{"email":["The email must be a valid email address."]}}"#;
        let err = ApiError::from_parts(StatusCode::UNPROCESSABLE_ENTITY, &HeaderMap::new(), body);

        assert!(err.is_validation());
        assert_eq!(err.message, "The given data was invalid.");
        assert_eq!(
            err.errors["email"],
            ["The email must be a valid email address."]
        );
        assert!(err.to_string().contains("email: The email must be"));
    }

    #[test]
    fn parses_bare_string_body() {
        let body = br#""Campaign is not with status draft""#;
        let err = ApiError::from_parts(StatusCode::UNPROCESSABLE_ENTITY, &HeaderMap::new(), body);

        assert_eq!(err.message, "Campaign is not with status draft");
        assert!(err.errors.is_empty());
    }

    #[test]
    fn falls_back_to_canonical_reason() {
        let empty = ApiError::from_parts(StatusCode::BAD_GATEWAY, &HeaderMap::new(), b"");
        assert_eq!(empty.message, "Bad Gateway");
        assert!(empty.is_server_error());

        let blank = ApiError::from_parts(
            StatusCode::NOT_FOUND,
            &HeaderMap::new(),
            br#"{"message":""}"#,
        );
        assert_eq!(blank.message, "Not Found");

        let html = ApiError::from_parts(
            StatusCode::BAD_GATEWAY,
            &HeaderMap::new(),
            b"<html>oops</html>",
        );
        assert_eq!(html.message, "<html>oops</html>");
    }

    #[test]
    fn reads_rate_limit_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("retry-after", HeaderValue::from_static("119"));
        headers.insert("x-ratelimit-remaining", HeaderValue::from_static("0"));

        let err = ApiError::from_parts(
            StatusCode::TOO_MANY_REQUESTS,
            &headers,
            br#"{"message":"Too Many Attempts."}"#,
        );

        assert!(err.is_rate_limited());
        assert_eq!(err.retry_after, Some(Duration::from_secs(119)));
        assert_eq!(err.rate_limit_remaining, Some(0));
        assert!(Error::from(err).is_rate_limited());
    }

    #[test]
    fn truncates_on_char_boundary() {
        assert_eq!(truncate("ąčę".as_bytes(), 2), "ąč…");
        assert_eq!(truncate(b"short", 10), "short");
    }
}
