//! Request pipeline: URL building, sending, status handling and decoding.

use bytes::Bytes;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{ApiError, Error, MailerLite, Result};

/// A request to any MailerLite endpoint, including ones this SDK does not model yet.
///
/// Created with [`MailerLite::request`]. Authentication, `Accept`, `X-Version`, the user agent
/// and the timeout are applied exactly as for the typed methods.
///
/// ```no_run
/// # async fn run(client: mailerlite_rs::MailerLite) -> mailerlite_rs::Result<()> {
/// use reqwest::Method;
/// use serde_json::Value;
///
/// let body: Value = client
///     .request(Method::GET, "subscribers")
///     .query(&[("limit", "5")])
///     .send()
///     .await?;
/// # Ok(()) }
/// ```
#[derive(Debug)]
#[must_use = "a request does nothing until it is sent"]
pub struct RawRequest<'a> {
    client: &'a MailerLite,
    builder: reqwest::RequestBuilder,
}

/// A successful response with its body fully read.
pub(crate) struct Response {
    pub(crate) status: StatusCode,
    pub(crate) body: Bytes,
}

impl MailerLite {
    /// Starts a request to `path`, relative to `https://connect.mailerlite.com/api/`
    /// (e.g. `"subscribers/123/activity-log"`).
    ///
    /// Each `/`-separated segment is percent-encoded. Prefer the typed resource methods; this
    /// is an escape hatch for endpoints the SDK does not cover.
    pub fn request(&self, method: Method, path: &str) -> RawRequest<'_> {
        let segments: Vec<&str> = path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect();
        self.call(method, &segments)
    }

    /// Starts a request to `BASE_URL/segment/segment/...`, percent-encoding every segment.
    pub(crate) fn call(&self, method: Method, segments: &[&str]) -> RawRequest<'_> {
        let inner = &self.inner;
        let mut url = inner.base_url.clone();
        if let Ok(mut path) = url.path_segments_mut() {
            path.pop_if_empty().extend(segments);
        }

        let mut builder = inner
            .http
            .request(method, url)
            .headers(inner.headers.clone());
        if let Some(timeout) = inner.timeout {
            builder = builder.timeout(timeout);
        }

        RawRequest {
            client: self,
            builder,
        }
    }
}

impl RawRequest<'_> {
    /// Adds query parameters (anything `serde_urlencoded` can serialize: a struct, a map,
    /// or a slice of pairs).
    pub fn query<Q: Serialize + ?Sized>(mut self, query: &Q) -> Self {
        self.builder = self.builder.query(query);
        self
    }

    /// Sets a JSON request body.
    pub fn json<B: Serialize + ?Sized>(mut self, body: &B) -> Self {
        self.builder = self.builder.json(body);
        self
    }

    /// Sends the request and deserializes the whole response body into `T`.
    ///
    /// Use `serde_json::Value` for untyped access. An empty body (e.g. `204`) decodes as
    /// JSON `null`, so `Option<T>` and `()` both work for endpoints without content.
    pub async fn send<T: DeserializeOwned>(self) -> Result<T> {
        self.execute().await?.json()
    }

    /// Sends the request and discards the response body.
    pub async fn send_empty(self) -> Result<()> {
        self.execute().await.map(drop)
    }

    /// Sends the request and unwraps the `{"data": ...}` envelope.
    pub(crate) async fn data<T: DeserializeOwned>(self) -> Result<T> {
        self.execute().await?.data()
    }

    /// Sends the request and returns the status and raw body. Non-2xx becomes [`Error::Api`].
    pub(crate) async fn execute(self) -> Result<Response> {
        let request = self.builder.build()?;

        #[cfg(test)]
        if let Some(mock) = &self.client.inner.mock {
            return finish(mock.handle(request)).await;
        }

        finish(self.client.inner.http.execute(request).await?).await
    }
}

async fn finish(response: reqwest::Response) -> Result<Response> {
    let status = response.status();
    if status.is_success() {
        let body = response.bytes().await?;
        Ok(Response { status, body })
    } else {
        let headers = response.headers().clone();
        let body = response.bytes().await?;
        Err(ApiError::from_parts(status, &headers, &body).into())
    }
}

impl Response {
    /// Decodes the whole body. An empty body decodes as `null`.
    pub(crate) fn json<T: DeserializeOwned>(&self) -> Result<T> {
        let body: &[u8] = if self.body.is_empty() {
            b"null"
        } else {
            &self.body
        };
        serde_json::from_slice(body).map_err(|err| Error::decode(err, &self.body))
    }

    /// Decodes `{"data": T}`. Also accepts a bare `T`, which a few e-commerce delete
    /// endpoints return instead of the usual envelope.
    pub(crate) fn data<T: DeserializeOwned>(&self) -> Result<T> {
        #[derive(Deserialize)]
        struct Envelope<T> {
            data: T,
        }

        match serde_json::from_slice::<Envelope<T>>(&self.body) {
            Ok(envelope) => Ok(envelope.data),
            Err(err) => {
                serde_json::from_slice::<T>(&self.body).map_err(|_| Error::decode(err, &self.body))
            }
        }
    }
}

/// In-process transport for unit tests: records requests and replays canned responses
/// without opening sockets and without changing the production base URL.
#[cfg(test)]
pub(crate) mod mock {
    #![allow(clippy::needless_pass_by_value)]

    use std::{
        collections::VecDeque,
        sync::{Arc, Mutex},
    };

    use reqwest::{Method, StatusCode};
    use serde_json::Value;

    use crate::MailerLite;

    #[derive(Debug, Default)]
    pub(crate) struct Mock {
        responses: Mutex<VecDeque<http::Response<Vec<u8>>>>,
        requests: Mutex<Vec<Recorded>>,
    }

    /// A request as it would have gone over the wire.
    #[derive(Clone, Debug)]
    pub(crate) struct Recorded {
        pub(crate) method: Method,
        pub(crate) url: String,
        pub(crate) headers: reqwest::header::HeaderMap,
        pub(crate) body: Option<Value>,
    }

    impl Mock {
        pub(crate) fn handle(&self, request: reqwest::Request) -> reqwest::Response {
            let body = request
                .body()
                .and_then(reqwest::Body::as_bytes)
                .map(|bytes| serde_json::from_slice(bytes).unwrap());
            self.requests.lock().unwrap().push(Recorded {
                method: request.method().clone(),
                url: request.url().to_string(),
                headers: request.headers().clone(),
                body,
            });

            let response = self
                .responses
                .lock()
                .unwrap()
                .pop_front()
                .expect("no mock response queued");
            reqwest::Response::from(response)
        }

        /// Queues a JSON response.
        pub(crate) fn respond(&self, status: u16, body: Value) -> &Self {
            let body = if body.is_null() {
                Vec::new()
            } else {
                serde_json::to_vec(&body).unwrap()
            };
            self.respond_raw(status, &[], body)
        }

        /// Queues a response with explicit headers and raw body.
        pub(crate) fn respond_raw(
            &self,
            status: u16,
            headers: &[(&str, &str)],
            body: Vec<u8>,
        ) -> &Self {
            let mut response =
                http::Response::builder().status(StatusCode::from_u16(status).unwrap());
            for (name, value) in headers {
                response = response.header(*name, *value);
            }
            self.responses
                .lock()
                .unwrap()
                .push_back(response.body(body).unwrap());
            self
        }

        /// Takes the single request made since the last call.
        pub(crate) fn request(&self) -> Recorded {
            let mut requests = self.requests.lock().unwrap();
            assert_eq!(
                requests.len(),
                1,
                "expected exactly one request, got {requests:#?}"
            );
            requests.remove(0)
        }

        /// Takes every request made since the last call.
        pub(crate) fn requests(&self) -> Vec<Recorded> {
            std::mem::take(&mut *self.requests.lock().unwrap())
        }
    }

    impl Recorded {
        /// Asserts method and URL (relative to the API base, including the query string).
        #[track_caller]
        pub(crate) fn assert(&self, method: Method, path_and_query: &str) -> &Self {
            assert_eq!(self.method, method);
            assert_eq!(
                self.url,
                format!("https://connect.mailerlite.com/api/{path_and_query}")
            );
            self
        }

        /// Asserts the JSON body.
        #[track_caller]
        pub(crate) fn body(&self, expected: &Value) -> &Self {
            assert_eq!(self.body.as_ref(), Some(expected));
            self
        }

        /// Asserts there is no body.
        #[track_caller]
        pub(crate) fn no_body(&self) -> &Self {
            assert_eq!(self.body, None);
            self
        }
    }

    /// A client wired to a fresh mock transport.
    pub(crate) fn client() -> (MailerLite, Arc<Mock>) {
        let mock = Arc::new(Mock::default());
        let client = MailerLite::builder("test-key")
            .mock(Arc::clone(&mock))
            .build()
            .unwrap();
        (client, mock)
    }

    /// Loads `tests/fixtures/<name>.json`.
    pub(crate) fn fixture(name: &str) -> Value {
        let path = format!("{}/tests/fixtures/{name}.json", env!("CARGO_MANIFEST_DIR"));
        let text = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("{path}: {err}"));
        serde_json::from_str(&text).unwrap_or_else(|err| panic!("{path}: {err}"))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use reqwest::{Method, header};
    use serde_json::{Value, json};

    use super::mock;

    #[tokio::test]
    async fn sends_auth_and_json_headers_and_encodes_path_segments() {
        let (client, mock) = mock::client();
        mock.respond(200, json!({"ok": true}));

        let body: Value = client
            .call(Method::GET, &["subscribers", "ann+tag@example.com"])
            .query(&[("limit", "1")])
            .send()
            .await
            .unwrap();

        assert_eq!(body, json!({"ok": true}));
        let request = mock.request();
        request
            .assert(Method::GET, "subscribers/ann+tag@example.com?limit=1")
            .no_body();
        assert_eq!(request.headers[header::AUTHORIZATION], "Bearer test-key");
        assert!(request.headers[header::AUTHORIZATION].is_sensitive());
        assert_eq!(request.headers[header::ACCEPT], "application/json");
        assert!(
            request.headers[header::USER_AGENT]
                .to_str()
                .unwrap()
                .starts_with("mailerlite-rs/")
        );

        mock.respond(200, json!({}));
        let _: Value = client
            .request(Method::GET, "/forms/a b/")
            .send()
            .await
            .unwrap();
        mock.request().assert(Method::GET, "forms/a%20b");
    }

    #[tokio::test]
    async fn maps_error_statuses() {
        let (client, mock) = mock::client();
        mock.respond_raw(
            429,
            &[("retry-after", "30"), ("x-ratelimit-remaining", "0")],
            br#"{"message":"Too Many Attempts."}"#.to_vec(),
        );

        let err = client
            .request(Method::GET, "groups")
            .send_empty()
            .await
            .unwrap_err();
        let api = err.as_api().unwrap();
        assert!(err.is_rate_limited());
        assert_eq!(api.message, "Too Many Attempts.");
        assert_eq!(api.retry_after, Some(Duration::from_secs(30)));

        mock.respond(401, json!({"message": "Unauthenticated."}));
        let err = client
            .request(Method::GET, "groups")
            .send_empty()
            .await
            .unwrap_err();
        assert!(err.as_api().unwrap().is_unauthorized());
    }

    #[tokio::test]
    async fn reports_decode_errors_with_body() {
        let (client, mock) = mock::client();
        mock.respond(200, json!({"data": {"id": []}}));

        let err = client
            .call(Method::GET, &["fields"])
            .data::<String>()
            .await
            .unwrap_err();
        assert!(matches!(&err, crate::Error::Decode { body, .. } if body.contains(r#""id":[]"#)));
    }

    #[tokio::test]
    async fn data_accepts_bare_bodies() {
        let (client, mock) = mock::client();
        mock.respond(200, json!({"data": "wrapped"}))
            .respond(200, json!("bare"));

        let wrapped: String = client.call(Method::GET, &["x"]).data().await.unwrap();
        let bare: String = client.call(Method::GET, &["x"]).data().await.unwrap();
        assert_eq!((wrapped.as_str(), bare.as_str()), ("wrapped", "bare"));
    }

    #[tokio::test]
    async fn applies_api_version_header() {
        let mock = std::sync::Arc::new(mock::Mock::default());
        let client = crate::MailerLite::builder("k")
            .api_version("2038-01-19")
            .mock(std::sync::Arc::clone(&mock))
            .build()
            .unwrap();
        mock.respond(204, Value::Null);

        let none: Option<Value> = client
            .request(Method::DELETE, "groups/1")
            .send()
            .await
            .unwrap();

        assert_eq!(none, None);
        assert_eq!(mock.request().headers["x-version"], "2038-01-19");
    }
}
