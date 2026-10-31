//! Errors retain HTTP diagnostics without printing response bodies or credentials.
use crate::ResponseMetadata;
use reqwest::StatusCode;
use std::{collections::BTreeMap, fmt};

const DIAGNOSTIC_LIMIT: usize = 8192;

/// The result of an SDK operation.
pub type Result<T> = std::result::Result<T, Error>;

/// A recoverable configuration, transport, API, or decoding failure.
#[derive(thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Invalid local configuration or request parameters.
    #[error("invalid SDK configuration or parameters: {0}")]
    Config(String),
    /// A booking weekday name could not be converted to a typed request value.
    #[error(transparent)]
    BookingWeekday(#[from] crate::resources::ecommerce::params::ParseBookingWeekdayError),
    /// A request could not be serialized.
    #[error("request serialization failed")]
    Serialize(#[from] serde_json::Error),
    /// HTTP transport failed. URLs are stripped before retaining the source.
    #[error("HTTP transport failed")]
    Transport(#[source] reqwest::Error),
    /// MailerLite returned a non-success HTTP status.
    #[error(transparent)]
    Api(ApiError),
    /// A response was incomplete or did not match its documented representation.
    #[error(transparent)]
    Decode(DecodeError),
    /// Pagination metadata was invalid or failed to make progress.
    #[error("invalid pagination: {0}")]
    Pagination(String),
    /// The complete operation exceeded its configured deadline.
    #[error("operation deadline exceeded")]
    Timeout,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Transport(error.without_url())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(_) => f.write_str("Config(<details omitted>)"),
            Self::BookingWeekday(error) => fmt::Debug::fmt(error, f),
            Self::Serialize(_) => f.write_str("Serialize(<details omitted>)"),
            Self::Transport(_) => f.write_str("Transport(<details omitted>)"),
            Self::Api(error) => fmt::Debug::fmt(error, f),
            Self::Decode(error) => fmt::Debug::fmt(error, f),
            Self::Pagination(_) => f.write_str("Pagination(<details omitted>)"),
            Self::Timeout => f.write_str("Timeout"),
        }
    }
}

/// A server error with structured validation details and bounded raw diagnostics.
///
/// Messages and bodies are available through explicit accessors, but omitted from
/// `Debug` and `Display` because servers can echo submitted sensitive data.
#[derive(Clone)]
pub struct ApiError {
    metadata: Box<ResponseMetadata>,
    message: Option<String>,
    validation_errors: BTreeMap<String, Vec<String>>,
    body: Vec<u8>,
    truncated: bool,
}

impl ApiError {
    pub(crate) fn new(metadata: ResponseMetadata, body: &[u8]) -> Self {
        let parsed = serde_json::from_slice::<serde_json::Value>(body).ok();
        let message = parsed
            .as_ref()
            .and_then(|value| {
                value
                    .as_str()
                    .or_else(|| value.get("message").and_then(|v| v.as_str()))
            })
            .map(str::to_owned);
        let validation_errors = parsed
            .as_ref()
            .and_then(|value| value.get("errors"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .unwrap_or_default();
        Self {
            metadata: Box::new(metadata),
            message,
            validation_errors,
            body: body[..body.len().min(DIAGNOSTIC_LIMIT)].to_vec(),
            truncated: body.len() > DIAGNOSTIC_LIMIT,
        }
    }

    /// HTTP status returned by MailerLite.
    pub fn status(&self) -> StatusCode {
        self.metadata.status
    }
    /// Server's message, when the error body contains one.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
    /// Field names mapped to their validation messages.
    pub fn validation_errors(&self) -> &BTreeMap<String, Vec<String>> {
        &self.validation_errors
    }
    /// Selected HTTP response metadata, including rate-limit timing.
    pub fn metadata(&self) -> &ResponseMetadata {
        &self.metadata
    }
    /// Whether the server returned HTTP 422 (validation failure).
    ///
    /// This checks the status, even if the response has no structured field errors.
    /// Inspect [`Self::validation_errors`] for any available validation details.
    pub fn is_validation(&self) -> bool {
        self.status() == StatusCode::UNPROCESSABLE_ENTITY
    }
    /// Whether the server returned HTTP 401 (authentication required or rejected).
    ///
    /// HTTP 403 (forbidden) is a separate status and returns `false`.
    pub fn is_unauthorized(&self) -> bool {
        self.status() == StatusCode::UNAUTHORIZED
    }
    /// Whether the server returned a status in the HTTP 500–599 range.
    ///
    /// This classification does not imply that retrying the operation is safe.
    /// Automatic retries remain disabled unless explicitly configured.
    pub fn is_server_error(&self) -> bool {
        self.status().is_server_error()
    }
    /// Whether the server rejected this request due to a rate limit.
    pub fn is_rate_limited(&self) -> bool {
        self.status() == StatusCode::TOO_MANY_REQUESTS
    }
    /// Bounded original response bytes. These may contain sensitive information.
    pub fn body(&self) -> &[u8] {
        &self.body
    }
    /// Whether the retained response body was truncated.
    pub fn body_truncated(&self) -> bool {
        self.truncated
    }
}

impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiError")
            .field("status", &self.status())
            .field("body_truncated", &self.truncated)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MailerLite returned HTTP {}", self.status())
    }
}
impl std::error::Error for ApiError {}

/// A response that could not be decoded into its expected model.
#[derive(Clone)]
pub struct DecodeError {
    metadata: Box<ResponseMetadata>,
    body: Vec<u8>,
    truncated: bool,
    detail: String,
}
impl DecodeError {
    pub(crate) fn new(metadata: ResponseMetadata, body: &[u8], detail: String) -> Self {
        Self {
            metadata: Box::new(metadata),
            body: body[..body.len().min(DIAGNOSTIC_LIMIT)].to_vec(),
            truncated: body.len() > DIAGNOSTIC_LIMIT,
            detail,
        }
    }
    pub(crate) fn incomplete(metadata: ResponseMetadata, body: &[u8], detail: String) -> Self {
        let mut error = Self::new(metadata, body, detail);
        error.truncated = true;
        error
    }
    /// HTTP status accompanying the malformed or unexpected response.
    pub fn status(&self) -> StatusCode {
        self.metadata.status
    }
    /// Selected HTTP response metadata.
    pub fn metadata(&self) -> &ResponseMetadata {
        &self.metadata
    }
    /// Parsing detail, potentially containing sensitive response values.
    pub fn detail(&self) -> &str {
        &self.detail
    }
    /// Bounded original response bytes. These may contain sensitive information.
    pub fn body(&self) -> &[u8] {
        &self.body
    }
    /// Whether the retained bytes were truncated or the response body was incomplete.
    pub fn body_truncated(&self) -> bool {
        self.truncated
    }
}
impl fmt::Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DecodeError")
            .field("status", &self.status())
            .field("body_truncated", &self.truncated)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected response for HTTP {}", self.status())
    }
}
impl std::error::Error for DecodeError {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn booking_conversion_propagates_with_question_mark_and_preserves_redacted_details() {
        use crate::resources::ecommerce::params::{ProductAction, ProductSettings};

        fn convert(name: &str) -> Result<ProductSettings> {
            Ok(ProductSettings::new(ProductAction::MoveToGroup)
                .try_replace_custom_availability_days(BTreeMap::from([(name.into(), None)]))?)
        }

        assert!(convert("monday").is_ok());
        let Err(error) = convert("private-server-key") else {
            panic!("unknown weekday must fail conversion");
        };
        assert!(!format!("{error:?} {error}").contains("private-server-key"));
        match error {
            Error::BookingWeekday(error) => assert_eq!(error.value(), "private-server-key"),
            other => panic!("unexpected error: {other:?}"),
        }
    }

    fn metadata(status: StatusCode) -> ResponseMetadata {
        ResponseMetadata {
            status,
            rate_limit: None,
            rate_remaining: None,
            retry_after: None,
            request_id: None,
        }
    }
    #[test]
    fn validation_is_available_without_debug_leakage() {
        let error = ApiError::new(
            metadata(StatusCode::UNPROCESSABLE_ENTITY),
            br#"{"message":"secret-token","errors":{"email":["secret-token"]}}"#,
        );
        assert_eq!(error.message(), Some("secret-token"));
        assert_eq!(error.validation_errors()["email"], ["secret-token"]);
        assert!(!format!("{error:?} {error}").contains("secret-token"));
    }
    #[test]
    fn string_error_and_malformed_body_retain_status() {
        let error = ApiError::new(metadata(StatusCode::BAD_REQUEST), br#""Invalid campaign""#);
        assert_eq!(error.message(), Some("Invalid campaign"));
        let error = ApiError::new(
            metadata(StatusCode::BAD_GATEWAY),
            b"<html>bad gateway</html>",
        );
        assert_eq!(error.status(), StatusCode::BAD_GATEWAY);
        assert!(error.message().is_none());
        assert_eq!(error.body(), b"<html>bad gateway</html>");
    }
    #[test]
    fn diagnostic_bytes_are_bounded() {
        let body = vec![b'x'; DIAGNOSTIC_LIMIT + 1];
        let error = DecodeError::new(metadata(StatusCode::OK), &body, "secret".into());
        assert_eq!(error.body().len(), DIAGNOSTIC_LIMIT);
        assert!(error.body_truncated());
        assert!(!format!("{error:?} {error}").contains("secret"));
    }
}
