//! Error types returned by every fallible SDK call.

use std::{fmt, time::Duration};

use reqwest::{StatusCode, header::HeaderMap};
use serde::{
    Deserialize, Deserializer,
    de::{self, DeserializeOwned, MapAccess, SeqAccess, Visitor},
};
use serde_json::Value;

/// Result alias used throughout the crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Anything that can go wrong while talking to MailerLite.
///
/// The SDK never panics on network, HTTP or decoding failures; they all surface here. The errors
/// you are most likely to handle in code have their own variant, so you can `match` on them:
///
/// ```no_run
/// # async fn run(client: mailerlite_rs::MailerLite) -> mailerlite_rs::Result<()> {
/// use mailerlite_rs::{Error, subscribers::CreateSubscriber};
///
/// match client.subscribers().create(&CreateSubscriber::new("not-an-email")).await {
///     Ok(created) => println!("subscribed {}", created.data.email),
///     Err(Error::Validation(invalid)) => {
///         for error in invalid.iter() {
///             eprintln!("{}: {}", error.field(), error.message());
///         }
///     }
///     Err(Error::RateLimited(limit)) => {
///         eprintln!("slow down, retry in {:?}", limit.retry_after());
///     }
///     Err(Error::NotFound(_)) => eprintln!("not found"),
///     Err(other) => return Err(other),
/// }
/// # Ok(()) }
/// ```
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// `422 Unprocessable Entity`: the request failed validation. Holds the per-field errors.
    #[error(transparent)]
    Validation(Box<ValidationError>),

    /// `429 Too Many Requests`: back off for [`RateLimit::retry_after`]. MailerLite allows 120
    /// requests per minute (20 per minute for imports); the SDK never retries on its own.
    #[error(transparent)]
    RateLimited(Box<RateLimit>),

    /// `401 Unauthorized`: the API key is missing, invalid or revoked.
    #[error(transparent)]
    Unauthorized(Box<ApiError>),

    /// `404 Not Found`: the resource does not exist.
    #[error(transparent)]
    NotFound(Box<ApiError>),

    /// Any other non-2xx response (`400`, `403`, `5xx`, ...).
    #[error(transparent)]
    Api(Box<ApiError>),

    /// The request could not be sent or the response could not be read
    /// (DNS, TLS, connection reset, timeout, ...).
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// MailerLite answered 2xx but the body did not match the expected shape.
    #[error(
        "failed to decode MailerLite response{}: {source}",
        .path.as_deref().map(|path| format!(" at `{path}`")).unwrap_or_default()
    )]
    #[non_exhaustive]
    Decode {
        /// Underlying `serde_json` error.
        #[source]
        source: serde_json::Error,
        /// Location of the mismatch, e.g. `data.enabled` or `data.items[2].price`.
        path: Option<String>,
        /// The start of the offending body (truncated to 1 KiB), for debugging.
        body: String,
    },

    /// The client could not be built, e.g. the API key contains characters that are not
    /// valid in an HTTP header.
    #[error("invalid client configuration: {0}")]
    Config(String),
}

impl Error {
    /// HTTP status of an API error; `None` for transport, decode and configuration errors.
    #[must_use]
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Validation(_) => Some(StatusCode::UNPROCESSABLE_ENTITY),
            Self::RateLimited(_) => Some(StatusCode::TOO_MANY_REQUESTS),
            Self::Unauthorized(err) | Self::NotFound(err) | Self::Api(err) => Some(err.status),
            _ => None,
        }
    }

    /// The message MailerLite sent with an API error.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Validation(err) => Some(err.message()),
            Self::RateLimited(err) => Some(err.message()),
            Self::Unauthorized(err) | Self::NotFound(err) | Self::Api(err) => Some(err.message()),
            _ => None,
        }
    }

    /// The validation details of a `422`.
    #[must_use]
    pub fn validation(&self) -> Option<&ValidationError> {
        match self {
            Self::Validation(err) => Some(err),
            _ => None,
        }
    }

    /// The rate limit details of a `429`.
    #[must_use]
    pub fn rate_limit(&self) -> Option<&RateLimit> {
        match self {
            Self::RateLimited(err) => Some(err),
            _ => None,
        }
    }

    /// The details of a `401`, `404` or any other non-2xx response without its own variant.
    #[must_use]
    pub fn as_api(&self) -> Option<&ApiError> {
        match self {
            Self::Unauthorized(err) | Self::NotFound(err) | Self::Api(err) => Some(err),
            _ => None,
        }
    }

    /// `true` for [`Error::Validation`].
    #[must_use]
    pub fn is_validation(&self) -> bool {
        matches!(self, Self::Validation(_))
    }

    /// `true` for [`Error::RateLimited`].
    #[must_use]
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, Self::RateLimited(_))
    }

    /// `true` for [`Error::Unauthorized`].
    #[must_use]
    pub fn is_unauthorized(&self) -> bool {
        matches!(self, Self::Unauthorized(_))
    }

    /// `true` for [`Error::NotFound`].
    #[must_use]
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound(_))
    }

    /// Maps a non-2xx response to the matching variant.
    pub(crate) fn from_response(status: StatusCode, headers: &HeaderMap, body: &[u8]) -> Self {
        let (message, errors) = parse_body(status, body);
        let retry_after = header_u64(headers, "retry-after").map(Duration::from_secs);

        match status {
            StatusCode::UNPROCESSABLE_ENTITY => {
                Self::Validation(Box::new(ValidationError { message, errors }))
            }
            StatusCode::TOO_MANY_REQUESTS => Self::RateLimited(Box::new(RateLimit {
                message,
                retry_after,
                limit: header_u32(headers, "x-ratelimit-limit"),
                remaining: header_u32(headers, "x-ratelimit-remaining"),
            })),
            _ => {
                let err = Box::new(ApiError {
                    status,
                    message,
                    errors,
                    retry_after,
                });
                match status {
                    StatusCode::UNAUTHORIZED => Self::Unauthorized(err),
                    StatusCode::NOT_FOUND => Self::NotFound(err),
                    _ => Self::Api(err),
                }
            }
        }
    }

    /// A decode failure of `body` as `T`. Only on this (already failed) path, the body is
    /// decoded again with `serde_path_to_error` to report where it went wrong, so successful
    /// responses pay nothing for it.
    pub(crate) fn decode<T: DeserializeOwned>(source: serde_json::Error, body: &[u8]) -> Self {
        let path = serde_path_to_error::deserialize::<_, T>(
            &mut serde_json::Deserializer::from_slice(body),
        )
        .err()
        .map(|err| err.path().to_string())
        .filter(|path| path != ".");
        Self::Decode {
            source,
            path,
            body: truncate(body, 1024),
        }
    }
}

/// A `422 Unprocessable Entity`: the request failed validation.
///
/// MailerLite answers
///
/// ```json
/// {"message": "The given data was invalid.",
///  "errors": {"email": ["The email must be a valid email address."]}}
/// ```
///
/// and this type keeps both parts, with the field errors in the order the API sent them. Some
/// business-rule failures (e.g. updating a campaign that isn't a draft) come back as a `422`
/// with only a message, in which case [`iter`](Self::iter) is empty.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationError {
    message: String,
    errors: Vec<FieldError>,
}

impl ValidationError {
    /// The API's summary message, e.g. `The given data was invalid.` or, with several errors,
    /// the first one followed by `(and N more errors)`.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Every field error, in the order the API sent them.
    pub fn iter(&self) -> std::slice::Iter<'_, FieldError> {
        self.errors.iter()
    }

    /// The messages for `field` (e.g. `"email"` or `"emails.0.from"`), if it has any.
    #[must_use]
    pub fn get(&self, field: &str) -> Option<&[String]> {
        self.errors
            .iter()
            .find(|error| error.field == field)
            .map(FieldError::messages)
    }

    /// The first field error, which is the one the API's summary message repeats.
    #[must_use]
    pub fn first(&self) -> Option<&FieldError> {
        self.errors.first()
    }

    /// Number of fields with errors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// `true` when the API sent only a message and no field errors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl<'a> IntoIterator for &'a ValidationError {
    type Item = &'a FieldError;
    type IntoIter = std::slice::Iter<'a, FieldError>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// One line listing every field error once, or the message when there are none:
///
/// ```text
/// MailerLite API error 422 Unprocessable Entity: email: The email field must be a valid email address.; name: The name field is required.
/// MailerLite API error 422 Unprocessable Entity: Campaign is not with status draft
/// ```
///
/// The summary message isn't repeated next to the field errors, because MailerLite derives it
/// from them.
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MailerLite API error {}: ",
            StatusCode::UNPROCESSABLE_ENTITY
        )?;
        write_details(f, &self.message, &self.errors)
    }
}

impl std::error::Error for ValidationError {}

/// One field's validation errors. Always holds at least one message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldError {
    field: String,
    messages: Vec<String>,
}

impl FieldError {
    /// The field key exactly as the API sent it, e.g. `email`, `emails.0.from` or
    /// `requests.3.path`. Empty when the API reported messages without a field.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// The first (usually only) message. Never empty: fields the API reports without a message
    /// are dropped while parsing.
    #[must_use]
    pub fn message(&self) -> &str {
        self.messages.first().map_or("", String::as_str)
    }

    /// Every message for this field.
    #[must_use]
    pub fn messages(&self) -> &[String] {
        &self.messages
    }

    /// The field key split into its parts: `emails.0.from` is
    /// `[Key("emails"), Index(0), Key("from")]`.
    pub fn path(&self) -> impl Iterator<Item = PathSegment<'_>> {
        self.field
            .split('.')
            .filter(|segment| !segment.is_empty())
            .map(|segment| {
                segment
                    .parse()
                    .map_or(PathSegment::Key(segment), PathSegment::Index)
            })
    }

    /// The index right after `key` in the path, mapping an error back to an item of your
    /// request: `groups.1` → `Some(1)` for `"groups"`, `requests.3.path` → `Some(3)` for
    /// `"requests"`.
    #[must_use]
    pub fn index_after(&self, key: &str) -> Option<usize> {
        let mut path = self.path();
        while let Some(segment) = path.next() {
            if segment == PathSegment::Key(key) {
                return match path.next() {
                    Some(PathSegment::Index(index)) => Some(index),
                    _ => None,
                };
            }
        }
        None
    }
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.field.is_empty() {
            write!(f, "{}: ", self.field)?;
        }
        f.write_str(&self.messages.join(" "))
    }
}

/// One part of a [`FieldError::path`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PathSegment<'a> {
    /// An object key, e.g. `emails`.
    Key(&'a str),
    /// An array index, e.g. the `0` in `emails.0.from`.
    Index(usize),
}

/// A `429 Too Many Requests`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RateLimit {
    message: String,
    retry_after: Option<Duration>,
    limit: Option<u32>,
    remaining: Option<u32>,
}

impl RateLimit {
    /// How long to wait before retrying, from the `Retry-After` header.
    #[must_use]
    pub fn retry_after(&self) -> Option<Duration> {
        self.retry_after
    }

    /// Requests allowed per window, from `X-RateLimit-Limit` (120 per minute globally).
    #[must_use]
    pub fn limit(&self) -> Option<u32> {
        self.limit
    }

    /// Requests left in the current window, from `X-RateLimit-Remaining`.
    #[must_use]
    pub fn remaining(&self) -> Option<u32> {
        self.remaining
    }

    /// The API's message, e.g. `You're being rate limited. Consider using batch requests: …` or
    /// `You're being rate limited on import creation.`.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for RateLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MailerLite API error {}: {}",
            StatusCode::TOO_MANY_REQUESTS,
            self.message
        )?;
        if let Some(retry_after) = self.retry_after {
            write!(f, " (retry after {}s)", retry_after.as_secs())?;
        }
        Ok(())
    }
}

impl std::error::Error for RateLimit {}

/// A non-2xx response without its own [`Error`] variant (`401`, `404`, `400`, `403`, `5xx`, ...).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiError {
    status: StatusCode,
    message: String,
    errors: Vec<FieldError>,
    retry_after: Option<Duration>,
}

impl ApiError {
    /// HTTP status code.
    #[must_use]
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Human readable message from the API (falls back to the status' canonical reason).
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Field errors, if the API sent any with this status.
    #[must_use]
    pub fn errors(&self) -> &[FieldError] {
        &self.errors
    }

    /// `Retry-After`, which MailerLite may send with `503`.
    #[must_use]
    pub fn retry_after(&self) -> Option<Duration> {
        self.retry_after
    }

    /// `true` for `403 Forbidden` (the action is denied for this account or API key).
    #[must_use]
    pub fn is_forbidden(&self) -> bool {
        self.status == StatusCode::FORBIDDEN
    }

    /// `true` for `5xx` and `408`, which MailerLite documents as errors on its side.
    #[must_use]
    pub fn is_server_error(&self) -> bool {
        self.status.is_server_error() || self.status == StatusCode::REQUEST_TIMEOUT
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MailerLite API error {}: ", self.status)?;
        write_details(f, &self.message, &self.errors)
    }
}

impl std::error::Error for ApiError {}

/// Field errors when there are any (the message is derived from them), else the message.
fn write_details(f: &mut fmt::Formatter<'_>, message: &str, errors: &[FieldError]) -> fmt::Result {
    let mut fields = errors
        .iter()
        .filter(|error| !error.messages.is_empty())
        .peekable();
    if fields.peek().is_none() {
        return f.write_str(message);
    }
    for (index, error) in fields.enumerate() {
        if index > 0 {
            f.write_str("; ")?;
        }
        write!(f, "{error}")?;
    }
    Ok(())
}

/// Reads an error body: `{"message": .., "errors": {..}}`, a bare JSON string, or anything
/// else (HTML from a proxy, empty). The message falls back to the status' canonical reason.
fn parse_body(status: StatusCode, body: &[u8]) -> (String, Vec<FieldError>) {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Body {
        // Both fields are read loosely: PHP sends an empty `errors` map as `[]`, and a shape
        // mismatch here must not discard the message.
        Object {
            #[serde(default)]
            message: Option<Value>,
            #[serde(default)]
            errors: Option<FieldErrors>,
        },
        // Some campaign endpoints answer with a bare JSON string.
        Message(String),
    }

    let (message, errors) = match serde_json::from_slice::<Body>(body) {
        Ok(Body::Object { message, errors }) => (
            message.and_then(|message| match message {
                Value::String(text) => Some(text),
                Value::Null => None,
                other => Some(other.to_string()),
            }),
            errors.map(|errors| errors.0).unwrap_or_default(),
        ),
        Ok(Body::Message(message)) => (Some(message), Vec::new()),
        Err(_) if body.is_empty() => (None, Vec::new()),
        Err(_) => (Some(truncate(body, 512)), Vec::new()),
    };

    let message = message
        .filter(|message| !message.is_empty())
        .or_else(|| status.canonical_reason().map(str::to_owned))
        .unwrap_or_default();

    (message, errors)
}

/// The `errors` member, read in document order (a `serde_json::Map` would sort the keys) and
/// whatever shape it arrives in: an object of strings or lists, `[]`, a bare list or string.
struct FieldErrors(Vec<FieldError>);

impl<'de> Deserialize<'de> for FieldErrors {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct FieldErrorsVisitor;

        impl<'de> Visitor<'de> for FieldErrorsVisitor {
            type Value = FieldErrors;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("validation errors")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<FieldErrors, A::Error> {
                let mut errors = Vec::new();
                while let Some((field, value)) = map.next_entry::<String, Value>()? {
                    // A field without messages carries no information; dropping it keeps the
                    // invariant that every FieldError has at least one message.
                    let messages = messages(value);
                    if !messages.is_empty() {
                        errors.push(FieldError { field, messages });
                    }
                }
                Ok(FieldErrors(errors))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<FieldErrors, A::Error> {
                let mut items = Vec::new();
                while let Some(item) = seq.next_element::<Value>()? {
                    items.push(item);
                }
                Ok(unnamed(Value::Array(items)))
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<FieldErrors, E> {
                Ok(unnamed(Value::String(value.to_owned())))
            }

            fn visit_bool<E: de::Error>(self, value: bool) -> Result<FieldErrors, E> {
                Ok(unnamed(Value::Bool(value)))
            }

            fn visit_i64<E: de::Error>(self, value: i64) -> Result<FieldErrors, E> {
                Ok(unnamed(Value::from(value)))
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<FieldErrors, E> {
                Ok(unnamed(Value::from(value)))
            }

            fn visit_f64<E: de::Error>(self, value: f64) -> Result<FieldErrors, E> {
                Ok(unnamed(Value::from(value)))
            }

            fn visit_unit<E: de::Error>(self) -> Result<FieldErrors, E> {
                Ok(FieldErrors(Vec::new()))
            }

            fn visit_none<E: de::Error>(self) -> Result<FieldErrors, E> {
                Ok(FieldErrors(Vec::new()))
            }

            fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<FieldErrors, D::Error> {
                d.deserialize_any(self)
            }
        }

        /// Messages without a field; an empty list means no errors at all.
        fn unnamed(value: Value) -> FieldErrors {
            let messages = messages(value);
            if messages.is_empty() {
                FieldErrors(Vec::new())
            } else {
                FieldErrors(vec![FieldError {
                    field: String::new(),
                    messages,
                }])
            }
        }

        deserializer.deserialize_any(FieldErrorsVisitor)
    }
}

fn messages(value: Value) -> Vec<String> {
    match value {
        Value::Null => Vec::new(),
        Value::String(text) => vec![text],
        Value::Array(items) => items.into_iter().flat_map(messages).collect(),
        other => vec![other.to_string()],
    }
}

fn header_u64(headers: &HeaderMap, name: &str) -> Option<u64> {
    headers.get(name)?.to_str().ok()?.trim().parse().ok()
}

fn header_u32(headers: &HeaderMap, name: &str) -> Option<u32> {
    header_u64(headers, name).and_then(|value| u32::try_from(value).ok())
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
    use reqwest::header::HeaderValue;

    use super::*;

    fn error(status: StatusCode, body: &[u8]) -> Error {
        Error::from_response(status, &HeaderMap::new(), body)
    }

    #[test]
    fn documented_validation_error() {
        let err = error(
            StatusCode::UNPROCESSABLE_ENTITY,
            br#"{"message":"The given data was invalid.","errors":{"email":["The email must be a valid email address."]}}"#,
        );

        let Error::Validation(invalid) = &err else {
            panic!("expected Error::Validation, got {err:?}");
        };
        assert_eq!(invalid.message(), "The given data was invalid.");
        assert_eq!(
            invalid.get("email"),
            Some(&["The email must be a valid email address.".to_owned()][..])
        );
        assert_eq!(invalid.len(), 1);
        assert_eq!(err.status(), Some(StatusCode::UNPROCESSABLE_ENTITY));
        assert_eq!(err.message(), Some("The given data was invalid."));
        assert_eq!(
            err.to_string(),
            "MailerLite API error 422 Unprocessable Entity: email: The email must be a valid email address."
        );
    }

    #[test]
    fn several_errors_keep_api_order_and_print_once() {
        let err = error(
            StatusCode::UNPROCESSABLE_ENTITY,
            br#"{"message":"The name field is required. (and 2 more errors)","errors":{
                "name":["The name field is required.","The name field must be at least 3 characters."],
                "email":["The email field must be a valid email address."],
                "groups.1":["The selected groups.1 is invalid."]}}"#,
        );
        let invalid = err.validation().unwrap();

        let fields: Vec<&str> = invalid.iter().map(FieldError::field).collect();
        assert_eq!(fields, ["name", "email", "groups.1"]);
        assert_eq!(
            invalid.first().unwrap().message(),
            "The name field is required."
        );
        assert_eq!(invalid.get("missing"), None);
        assert_eq!(
            err.to_string(),
            "MailerLite API error 422 Unprocessable Entity: \
             name: The name field is required. The name field must be at least 3 characters.; \
             email: The email field must be a valid email address.; \
             groups.1: The selected groups.1 is invalid."
        );
    }

    #[test]
    fn field_paths_map_back_to_request_items() {
        let err = error(
            StatusCode::UNPROCESSABLE_ENTITY,
            br#"{"errors":{"emails.0.from":["Campaigns cannot be sent from this domain."],"groups.12":["Invalid."],"name":["Required."]}}"#,
        );
        let invalid = err.validation().unwrap();
        let errors: Vec<&FieldError> = invalid.iter().collect();

        assert_eq!(
            errors[0].path().collect::<Vec<_>>(),
            [
                PathSegment::Key("emails"),
                PathSegment::Index(0),
                PathSegment::Key("from")
            ]
        );
        assert_eq!(errors[0].index_after("emails"), Some(0));
        assert_eq!(errors[1].index_after("groups"), Some(12));
        assert_eq!(errors[2].index_after("name"), None);
        assert_eq!(errors[2].index_after("groups"), None);
    }

    #[test]
    fn message_only_422_is_still_a_validation_error() {
        let err = error(
            StatusCode::UNPROCESSABLE_ENTITY,
            br#""Campaign is not with status draft""#,
        );

        let invalid = err.validation().unwrap();
        assert!(invalid.is_empty());
        assert_eq!(invalid.message(), "Campaign is not with status draft");
        assert_eq!(
            err.to_string(),
            "MailerLite API error 422 Unprocessable Entity: Campaign is not with status draft"
        );
    }

    #[test]
    fn rate_limit_reads_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("retry-after", HeaderValue::from_static("119"));
        headers.insert("x-ratelimit-limit", HeaderValue::from_static("120"));
        headers.insert("x-ratelimit-remaining", HeaderValue::from_static("0"));

        let err = Error::from_response(
            StatusCode::TOO_MANY_REQUESTS,
            &headers,
            br#"{"message":"Too Many Attempts."}"#,
        );

        let Error::RateLimited(limit) = &err else {
            panic!("expected Error::RateLimited, got {err:?}");
        };
        assert_eq!(limit.retry_after(), Some(Duration::from_secs(119)));
        assert_eq!(limit.limit(), Some(120));
        assert_eq!(limit.remaining(), Some(0));
        assert_eq!(
            err.to_string(),
            "MailerLite API error 429 Too Many Requests: Too Many Attempts. (retry after 119s)"
        );
    }

    #[test]
    fn other_statuses_get_their_variant_and_fall_back_to_canonical_reason() {
        let unauthorized = error(
            StatusCode::UNAUTHORIZED,
            br#"{"message":"Unauthenticated."}"#,
        );
        assert!(unauthorized.is_unauthorized());
        assert_eq!(
            unauthorized.to_string(),
            "MailerLite API error 401 Unauthorized: Unauthenticated."
        );

        let not_found = error(StatusCode::NOT_FOUND, br#"{"message":""}"#);
        assert!(not_found.is_not_found());
        assert_eq!(not_found.message(), Some("Not Found"));

        let forbidden = error(StatusCode::FORBIDDEN, b"");
        assert!(forbidden.as_api().unwrap().is_forbidden());
        assert_eq!(forbidden.message(), Some("Forbidden"));

        let bad_gateway = error(StatusCode::BAD_GATEWAY, b"<html>oops</html>");
        let api = bad_gateway.as_api().unwrap();
        assert!(api.is_server_error());
        assert_eq!(api.message(), "<html>oops</html>");
        assert_eq!(bad_gateway.status(), Some(StatusCode::BAD_GATEWAY));
    }

    #[test]
    fn tolerates_unexpected_errors_shapes() {
        let empty = error(
            StatusCode::UNPROCESSABLE_ENTITY,
            br#"{"message":"Invalid data","errors":[]}"#,
        );
        assert!(empty.validation().unwrap().is_empty());
        assert_eq!(empty.message(), Some("Invalid data"));

        let mixed = error(
            StatusCode::UNPROCESSABLE_ENTITY,
            br#"{"message":"Bad","errors":{"email":"taken","requests.0":[["nested"]],"n":[1]}}"#,
        );
        let invalid = mixed.validation().unwrap();
        assert_eq!(invalid.get("email"), Some(&["taken".to_owned()][..]));
        assert_eq!(invalid.get("requests.0"), Some(&["nested".to_owned()][..]));
        assert_eq!(invalid.get("n"), Some(&["1".to_owned()][..]));

        let bare = error(
            StatusCode::BAD_REQUEST,
            br#"{"message":"Bad","errors":"something broke"}"#,
        );
        let api = bare.as_api().unwrap();
        assert_eq!(api.errors()[0].field(), "");
        assert_eq!(api.errors()[0].message(), "something broke");
        assert_eq!(
            bare.to_string(),
            "MailerLite API error 400 Bad Request: something broke"
        );
    }

    #[test]
    fn non_api_errors_have_no_status() {
        let err = Error::Config("x".into());
        assert_eq!(err.status(), None);
        assert_eq!(err.message(), None);
        assert!(err.validation().is_none() && err.rate_limit().is_none() && err.as_api().is_none());
    }

    #[test]
    fn truncates_on_char_boundary() {
        assert_eq!(truncate("ąčę".as_bytes(), 2), "ąč…");
        assert_eq!(truncate(b"short", 10), "short");
    }

    #[test]
    fn fields_without_messages_are_dropped() {
        let err = error(
            StatusCode::UNPROCESSABLE_ENTITY,
            br#"{"message":"Bad","errors":{"email":[],"name":null,"groups.0":["Invalid."]}}"#,
        );
        let invalid = err.validation().unwrap();

        assert_eq!(invalid.len(), 1);
        assert_eq!(invalid.first().map(FieldError::field), Some("groups.0"));
        assert_eq!(invalid.first().map(FieldError::message), Some("Invalid."));
    }
}
