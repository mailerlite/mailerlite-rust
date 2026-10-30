//! Batching: <https://developers.mailerlite.com/api/batching>
//!
//! Sends up to [`MAX_REQUESTS`] API calls in one HTTP request. Results come back in the same
//! order; a failed item does not fail the batch, so check each [`BatchResult`].
//!
//! ```no_run
//! # async fn run(client: mailerlite_rs::MailerLite) -> Result<(), Box<dyn std::error::Error>> {
//! use mailerlite_rs::{batch::BatchRequest, fields::Field, subscribers::Subscriber};
//! use serde_json::json;
//!
//! let request = BatchRequest::new()
//!     .get("api/fields")
//!     .post("api/subscribers", &json!({"email": "ann@example.com"}))?
//!     .delete("subscribers/123");
//! let response = client.batch().send(&request).await?;
//!
//! let fields: Vec<Field> = response.responses[0].decode()?;
//! if response.responses[1].is_success() {
//!     let ann: Subscriber = response.responses[1].decode()?;
//! }
//! # Ok(()) }
//! ```
//!
//! A batch consisting only of `POST api/subscribers` items is processed as a bulk import and
//! counts against the import rate limit (5 per minute) in addition to the global limit.
//! Webhook endpoints are not supported inside batches.

use reqwest::Method;
use serde::{Deserialize, Serialize, Serializer, de::DeserializeOwned};
use serde_json::Value;

use crate::{Error, MailerLite, Result};

/// Maximum number of items in one batch.
pub const MAX_REQUESTS: usize = 50;

/// One API call inside a [`BatchRequest`].
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct BatchItem {
    /// HTTP method: `GET`, `POST`, `PUT`, `DELETE` or `PATCH`.
    #[serde(serialize_with = "method_str")]
    pub method: Method,
    /// Path starting with `api/`, e.g. `api/subscribers/123`.
    pub path: String,
    /// JSON body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}

fn method_str<S: Serializer>(method: &Method, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(method.as_str())
}

impl BatchItem {
    /// A call to `path` without a body.
    ///
    /// `path` is relative to `https://connect.mailerlite.com/`; a leading `/` is dropped and
    /// `api/` is prepended when missing, so `"subscribers"`, `"/api/subscribers"` and
    /// `"api/subscribers"` are equivalent.
    pub fn new(method: Method, path: impl AsRef<str>) -> Self {
        let path = path.as_ref().trim_start_matches('/');
        let path = if path.starts_with("api/") {
            path.to_owned()
        } else {
            format!("api/{path}")
        };
        Self {
            method,
            path,
            body: None,
        }
    }

    /// Sets the body to a JSON value.
    #[must_use]
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// Sets the body by serializing `body` (e.g. a
    /// [`CreateSubscriber`](crate::subscribers::CreateSubscriber)).
    ///
    /// Fails only if `body` cannot be represented as JSON (e.g. a map with non-string keys).
    pub fn json<B: Serialize + ?Sized>(self, body: &B) -> serde_json::Result<Self> {
        Ok(self.body(serde_json::to_value(body)?))
    }
}

/// Body of [`Batch::send`]: an ordered list of up to [`MAX_REQUESTS`] calls.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[non_exhaustive]
pub struct BatchRequest {
    /// The calls, in execution order.
    pub requests: Vec<BatchItem>,
}

impl BatchRequest {
    /// An empty batch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a call.
    #[must_use]
    pub fn push(mut self, item: BatchItem) -> Self {
        self.requests.push(item);
        self
    }

    /// Appends a `GET` of `path` (see [`BatchItem::new`] for path rules).
    #[must_use]
    pub fn get(self, path: impl AsRef<str>) -> Self {
        self.push(BatchItem::new(Method::GET, path))
    }

    /// Appends a `DELETE` of `path`.
    #[must_use]
    pub fn delete(self, path: impl AsRef<str>) -> Self {
        self.push(BatchItem::new(Method::DELETE, path))
    }

    /// Appends a `POST` of `body` to `path`. Fails only if `body` cannot be represented as
    /// JSON; use [`push`](Self::push) with [`BatchItem::body`] for an infallible variant.
    pub fn post<B: Serialize + ?Sized>(
        self,
        path: impl AsRef<str>,
        body: &B,
    ) -> serde_json::Result<Self> {
        Ok(self.push(BatchItem::new(Method::POST, path).json(body)?))
    }

    /// Appends a `PUT` of `body` to `path`.
    pub fn put<B: Serialize + ?Sized>(
        self,
        path: impl AsRef<str>,
        body: &B,
    ) -> serde_json::Result<Self> {
        Ok(self.push(BatchItem::new(Method::PUT, path).json(body)?))
    }

    /// Appends a `PATCH` of `body` to `path`.
    pub fn patch<B: Serialize + ?Sized>(
        self,
        path: impl AsRef<str>,
        body: &B,
    ) -> serde_json::Result<Self> {
        Ok(self.push(BatchItem::new(Method::PATCH, path).json(body)?))
    }

    /// Number of calls.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requests.len()
    }

    /// `true` if the batch has no calls.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }
}

/// Response of [`Batch::send`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchResponse {
    /// Number of calls executed.
    #[serde(default)]
    pub total: u32,
    /// Calls that returned a 2xx status.
    #[serde(default)]
    pub successful: u32,
    /// Calls that failed.
    #[serde(default)]
    pub failed: u32,
    /// One result per call, in request order.
    #[serde(default)]
    pub responses: Vec<BatchResult>,
}

/// The outcome of one call in a batch.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchResult {
    /// HTTP status of the call.
    pub code: u16,
    /// Response body (`null` for `204 No Content`).
    #[serde(default)]
    pub body: Value,
}

impl BatchResult {
    /// `true` for a 2xx status.
    #[must_use]
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.code)
    }

    /// MailerLite's error message for a failed call, if the body has one.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.body.get("message").and_then(Value::as_str)
    }

    /// Decodes the body into `T`, unwrapping a `{"data": ...}` envelope when present
    /// (single resources and lists) and decoding the bare body otherwise.
    ///
    /// This does not look at [`code`](Self::code): check [`is_success`](Self::is_success)
    /// first. A body that does not match `T` becomes [`Error::Decode`].
    pub fn decode<T: DeserializeOwned>(&self) -> Result<T> {
        let enveloped = match self.body.get("data") {
            Some(data) => T::deserialize(data),
            None => T::deserialize(&self.body),
        };
        enveloped
            .or_else(|err| T::deserialize(&self.body).map_err(|_| err))
            .map_err(|err| Error::decode(err, &serde_json::to_vec(&self.body).unwrap_or_default()))
    }
}

/// The batch endpoint. Obtained with [`MailerLite::batch`].
#[derive(Clone, Copy, Debug)]
pub struct Batch<'a> {
    client: &'a MailerLite,
}

impl MailerLite {
    /// The batch endpoint.
    #[must_use]
    pub fn batch(&self) -> Batch<'_> {
        Batch { client: self }
    }
}

impl Batch<'_> {
    /// `POST /batch`: runs every call in `request` in order.
    ///
    /// Fails with `422` if the envelope itself is invalid (e.g. more than [`MAX_REQUESTS`]
    /// items or a path not starting with `api/`); failures of individual calls are reported
    /// in [`BatchResponse::responses`] instead.
    pub async fn send(&self, request: &BatchRequest) -> Result<BatchResponse> {
        self.client
            .call(Method::POST, &["batch"])
            .json(request)
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::json;

    use super::*;
    use crate::{
        fields::{Field, FieldType},
        http::mock::{client, fixture},
    };

    #[tokio::test]
    async fn send() {
        let (client, mock) = client();
        mock.respond(200, fixture("batch_response"));

        let request = BatchRequest::new()
            .get("api/fields")
            .post("subscribers", &json!({"email": "ann@example.com"}))
            .unwrap()
            .put("/api/groups/1", &json!({"name": "VIP"}))
            .unwrap()
            .patch("api/x", &json!({}))
            .unwrap()
            .delete("api/segments/2")
            .push(
                BatchItem::new(Method::POST, "fields").body(json!({"name": "n", "type": "text"})),
            );
        let response = client.batch().send(&request).await.unwrap();

        mock.request().assert(Method::POST, "batch").body(&json!({
            "requests": [
                {"method": "GET", "path": "api/fields"},
                {"method": "POST", "path": "api/subscribers", "body": {"email": "ann@example.com"}},
                {"method": "PUT", "path": "api/groups/1", "body": {"name": "VIP"}},
                {"method": "PATCH", "path": "api/x", "body": {}},
                {"method": "DELETE", "path": "api/segments/2"},
                {"method": "POST", "path": "api/fields", "body": {"name": "n", "type": "text"}}
            ]
        }));
        assert_eq!(request.len(), 6);
        assert_eq!(
            (response.total, response.successful, response.failed),
            (1, 1, 0)
        );
        let result = &response.responses[0];
        assert!(result.is_success());
        let fields: Vec<Field> = result.decode().unwrap();
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[1].key, "new_field");
        assert_eq!(fields[1].field_type, FieldType::Text);
    }

    #[tokio::test]
    async fn failed_item() {
        let (client, mock) = client();
        mock.respond(200, fixture("batch_failed_item"));

        let response = client
            .batch()
            .send(&BatchRequest::new().post("fields", &json!({})).unwrap())
            .await
            .unwrap();

        let result = &response.responses[0];
        assert_eq!(response.failed, 1);
        assert_eq!(result.code, 422);
        assert!(!result.is_success());
        assert_eq!(result.message(), Some("The given data was invalid."));
        let err = result.decode::<Field>().unwrap_err();
        assert!(
            matches!(&err, Error::Decode { body, .. } if body.contains("The given data was invalid."))
        );
    }

    #[test]
    fn decode_accepts_bare_bodies_and_empty_responses() {
        let bare = BatchResult {
            code: 200,
            body: json!({"import_progress_url": "u"}),
        };
        let value: std::collections::BTreeMap<String, String> = bare.decode().unwrap();
        assert_eq!(value["import_progress_url"], "u");

        let empty: BatchResult = serde_json::from_value(json!({"code": 204})).unwrap();
        assert!(empty.is_success());
        empty.decode::<()>().unwrap();
        assert_eq!(empty.decode::<Option<Field>>().unwrap(), None);
    }
}
