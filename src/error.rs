//! The error type of the email, brand style and file methods.
//!
//! The older methods do not use this type. They return a `Response` as before.

use reqwest::{header::HeaderMap, StatusCode};
use serde_json::Value;
use std::{collections::BTreeMap, error::Error as StdError, fmt};

/// An error from a method that returns `Result`.
///
/// The variants up to `EtagInSettings` are client-side errors. For them, the SDK sends no request.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An ID is empty.
    EmptyId { name: &'static str },
    /// An ID contains a character that is not an ASCII digit.
    InvalidId { name: &'static str, value: String },
    /// A write has an empty etag.
    EmptyEtag,
    /// The idempotency key is not 1 to 255 printable ASCII characters, or it starts or ends with a space.
    InvalidIdempotencyKey,
    /// The `extra` map of the settings has an `etag` key. Give the etag as an argument.
    EtagInSettings,
    /// The API returned a 4xx or 5xx status. The error is boxed to keep `Error` small. You can read its
    /// fields directly, for example `error.code`.
    Api(Box<ApiError>),
    /// The request did not complete, for example because of a network error.
    Transport(TransportError),
    /// The API returned a 2xx status, but the SDK cannot decode the body.
    Decode {
        status_code: StatusCode,
        body: String,
        source: serde_json::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EmptyId { name } => write!(f, "{name} must not be empty"),
            Error::InvalidId { name, value } => {
                write!(f, "{name} must contain only the digits 0-9, got {value:?}")
            }
            Error::EmptyEtag => write!(f, "the etag must not be empty"),
            Error::InvalidIdempotencyKey => write!(
                f,
                "the idempotency key must be 1 to 255 printable ASCII characters, without a space at the start or the end"
            ),
            Error::EtagInSettings => write!(
                f,
                "the settings must not contain an \"etag\" key; give the etag as an argument"
            ),
            Error::Api(error) => error.fmt(f),
            Error::Transport(error) => write!(f, "the request failed: {error}"),
            Error::Decode {
                status_code,
                source,
                ..
            } => write!(f, "cannot decode the response body ({status_code}): {source}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Transport(error) => Some(&error.0),
            Error::Decode { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<ApiError> for Error {
    fn from(error: ApiError) -> Self {
        Error::Api(Box::new(error))
    }
}

/// An error response of the API (a 4xx or 5xx status).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ApiError {
    pub status_code: StatusCode,
    /// The `message` of the body, if the body is JSON and has one.
    pub message: Option<String>,
    /// The `code` of the body, for example `"version_conflict"`.
    pub code: Option<String>,
    /// The current etag of the email. A `409` with the code `"version_conflict"` has it.
    pub etag: Option<String>,
    /// The errors for each field of a `422` response.
    pub errors: Option<BTreeMap<String, Vec<String>>>,
    pub headers: HeaderMap,
    /// The raw body. It can be other than JSON, for example for a `503`.
    pub body: String,
}

impl ApiError {
    pub(crate) fn from_response(status_code: StatusCode, headers: HeaderMap, body: String) -> Self {
        let json: Value = serde_json::from_str(&body).unwrap_or(Value::Null);
        let text = |key: &str| json.get(key).and_then(Value::as_str).map(str::to_string);
        let errors: Option<BTreeMap<String, Vec<String>>> = json
            .get("errors")
            .and_then(|errors| serde_json::from_value(errors.clone()).ok());

        Self {
            status_code,
            message: text("message"),
            code: text("code"),
            etag: text("etag"),
            errors,
            headers,
            body,
        }
    }

    /// Returns true if the email changed after you read the etag. Then `etag` has the current etag.
    pub fn is_version_conflict(&self) -> bool {
        self.code.as_deref() == Some(crate::email::VERSION_CONFLICT)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API error {}", self.status_code)?;
        match (&self.code, &self.message) {
            (Some(code), Some(message)) => write!(f, ": {code}: {message}"),
            (Some(code), None) => write!(f, ": {code}"),
            (None, Some(message)) => write!(f, ": {message}"),
            (None, None) => Ok(()),
        }
    }
}

impl StdError for ApiError {}

/// A request that did not complete. `source()` of the `Error` gives the error of the HTTP client.
#[derive(Debug)]
pub struct TransportError(pub(crate) reqwest::Error);

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderMap;

    fn api_error(status: u16, body: &str) -> ApiError {
        ApiError::from_response(
            StatusCode::from_u16(status).unwrap(),
            HeaderMap::new(),
            body.to_string(),
        )
    }

    #[test]
    fn error_is_small_enough_for_a_result() {
        // Clippy `result_large_err` warns at 128 bytes. The API error is boxed to stay below it.
        assert!(
            std::mem::size_of::<Error>() < 128,
            "size_of::<Error>() is {}",
            std::mem::size_of::<Error>()
        );
    }

    #[test]
    fn api_error_reads_the_error_fields() {
        let error: ApiError = api_error(
            409,
            r#"{"message": "The email changed.", "code": "version_conflict", "etag": "43"}"#,
        );

        assert_eq!(error.status_code, 409);
        assert_eq!(error.message.as_deref(), Some("The email changed."));
        assert_eq!(error.code.as_deref(), Some("version_conflict"));
        assert_eq!(error.etag.as_deref(), Some("43"));
        assert!(error.errors.is_none());
        assert!(error.is_version_conflict());
    }

    #[test]
    fn api_error_reads_the_field_errors() {
        let error: ApiError = api_error(
            422,
            r#"{"message": "The etag field is required.", "errors": {"etag": ["The etag field is required."]}}"#,
        );

        let errors: BTreeMap<String, Vec<String>> = error.errors.clone().unwrap();
        assert_eq!(errors["etag"], vec!["The etag field is required."]);
        assert!(error.code.is_none());
        assert!(!error.is_version_conflict());
    }

    #[test]
    fn api_error_keeps_a_body_that_is_not_json() {
        let error: ApiError = api_error(503, "<html>Service Unavailable</html>");

        assert_eq!(error.body, "<html>Service Unavailable</html>");
        assert!(error.message.is_none());
        assert!(error.code.is_none());
    }

    #[test]
    fn a_409_without_the_code_is_not_a_version_conflict() {
        let error: ApiError = api_error(409, r#"{"message": "The campaign is not a draft."}"#);

        assert!(!error.is_version_conflict());
    }

    #[test]
    fn display_shows_the_status_and_the_code() {
        let error: Error = Error::from(api_error(
            409,
            r#"{"message": "The email changed.", "code": "version_conflict", "etag": "43"}"#,
        ));

        assert_eq!(
            error.to_string(),
            "API error 409 Conflict: version_conflict: The email changed."
        );
    }

    #[test]
    fn display_shows_the_message_without_a_code() {
        let error: Error = Error::from(api_error(404, r#"{"message": "Not found."}"#));

        assert_eq!(error.to_string(), "API error 404 Not Found: Not found.");
    }

    #[test]
    fn display_shows_the_client_side_errors() {
        assert_eq!(
            Error::EmptyId { name: "email_id" }.to_string(),
            "email_id must not be empty"
        );
        assert_eq!(
            Error::InvalidId {
                name: "block_id",
                value: "1/html".to_string()
            }
            .to_string(),
            "block_id must contain only the digits 0-9, got \"1/html\""
        );
        assert_eq!(Error::EmptyEtag.to_string(), "the etag must not be empty");
    }
}
