//! The response type and the body types of the email, brand style and file methods.

use reqwest::{header::HeaderMap, StatusCode};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::ops::Deref;

/// A successful response: the status code, the headers and the decoded body.
///
/// The response acts like the body, so `response.etag` and `response.data` work.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ApiResponse<T> {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub body: T,
}

impl<T> ApiResponse<T> {
    pub(crate) fn new(status_code: StatusCode, headers: HeaderMap, body: T) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }

    /// Returns true if the API sent a stored response for an idempotency key that it already knew.
    /// The SDK ignores the case and the spaces of the `Idempotent-Replayed` header value.
    pub fn is_idempotent_replay(&self) -> bool {
        self.headers
            .get("Idempotent-Replayed")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.trim().eq_ignore_ascii_case("true"))
    }

    /// Returns the body and drops the status code and the headers.
    pub fn into_body(self) -> T {
        self.body
    }
}

impl<T> Deref for ApiResponse<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.body
    }
}

/// A body with `data` and the new `etag` of the email.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct WithEtag<T> {
    pub data: T,
    /// Send this etag with the next write to the email.
    pub etag: String,
}

/// A body with only `data`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct DataOnly<T> {
    pub data: T,
}

/// A body with only the new `etag` of the email.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct EtagOnly {
    /// Send this etag with the next write to the email.
    pub etag: String,
}

/// Decodes a JSON object, and also accepts `[]` as an empty object. PHP encodes an empty array as `[]`.
pub(crate) fn object_or_empty_array<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + serde::de::DeserializeOwned,
{
    match Value::deserialize(deserializer)? {
        Value::Array(items) if items.is_empty() => Ok(T::default()),
        Value::Null => Ok(T::default()),
        value => serde_json::from_value(value).map_err(serde::de::Error::custom),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{HeaderMap, HeaderValue};

    fn response(replayed: Option<&'static str>) -> ApiResponse<EtagOnly> {
        let mut headers: HeaderMap = HeaderMap::new();
        if let Some(value) = replayed {
            headers.insert("Idempotent-Replayed", HeaderValue::from_static(value));
        }

        ApiResponse::new(
            StatusCode::OK,
            headers,
            EtagOnly {
                etag: "45".to_string(),
            },
        )
    }

    #[test]
    fn replay_flag_is_true_for_true_in_any_case() {
        assert!(response(Some("true")).is_idempotent_replay());
        assert!(response(Some("True")).is_idempotent_replay());
        assert!(response(Some(" TRUE ")).is_idempotent_replay());
    }

    #[test]
    fn replay_flag_is_false_without_the_header() {
        assert!(!response(None).is_idempotent_replay());
        assert!(!response(Some("false")).is_idempotent_replay());
    }

    #[test]
    fn response_acts_like_the_body() {
        let response: ApiResponse<EtagOnly> = response(None);

        assert_eq!(response.etag, "45");
        assert_eq!(response.status_code, 200);
        assert_eq!(response.into_body().etag, "45");
    }
}
