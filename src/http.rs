//! The shared request helper of the email, brand style and file methods.
//!
//! The older methods do not use it, so their behavior does not change.

use crate::{
    api_response::ApiResponse,
    error::{ApiError, Error, TransportError},
    MailerLite,
};
pub(crate) use reqwest::Method;
use reqwest::{header::HeaderValue, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

/// Use this value for a request that has no body.
pub(crate) const NO_BODY: Option<&()> = None;

/// Sends a request to `path` below the base URL and decodes a 2xx body into `T`.
///
/// - The SDK sends `body` as JSON only when it is `Some`. For `None`, the raw body is empty, not `null`.
/// - The SDK sends the query pairs as they are. An empty value is sent as `key=`.
/// - A 4xx or 5xx status gives `Error::Api`. A 2xx body that is not valid gives `Error::Decode`.
pub(crate) async fn send<T: DeserializeOwned, B: Serialize + ?Sized>(
    mailerlite: &MailerLite,
    method: Method,
    path: &str,
    query: &[(&str, String)],
    body: Option<&B>,
    idempotency_key: Option<&str>,
) -> Result<ApiResponse<T>, Error> {
    let url: String = format!("{}{}", mailerlite.base_url, path);
    let mut request: RequestBuilder = mailerlite.client.request.request(method, url);

    if !query.is_empty() {
        request = request.query(query);
    }
    if let Some(key) = idempotency_key {
        let value: HeaderValue =
            HeaderValue::from_str(key).map_err(|_| Error::InvalidIdempotencyKey)?;
        request = request.header("Idempotency-Key", value);
    }
    if let Some(body) = body {
        request = request.json(body);
    }

    let response = request
        .send()
        .await
        .map_err(|error| Error::Transport(TransportError(error)))?;
    let status_code: StatusCode = response.status();
    let headers = response.headers().clone();
    let bytes = response
        .bytes()
        .await
        .map_err(|error| Error::Transport(TransportError(error)))?;

    if !status_code.is_success() {
        let body: String = String::from_utf8_lossy(&bytes).into_owned();

        return Err(Error::Api(ApiError::from_response(
            status_code,
            headers,
            body,
        )));
    }

    match serde_json::from_slice::<T>(&bytes) {
        Ok(body) => Ok(ApiResponse::new(status_code, headers, body)),
        Err(source) => Err(Error::Decode {
            status_code,
            body: String::from_utf8_lossy(&bytes).into_owned(),
            source,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use mockito::{Matcher, Mock, Server, ServerGuard};
    use serde_json::{json, Value};

    fn mailerlite(server: &ServerGuard) -> MailerLite {
        MailerLite::new("api_key".to_string()).with_base_url(server.url())
    }

    #[tokio::test]
    async fn patch_sends_the_json_body() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("PATCH", "/test/1")
            .match_header("Authorization", "Bearer api_key")
            .match_header("Accept", "application/json")
            .match_header("Content-Type", "application/json")
            .match_body(Matcher::Json(json!({"etag": "42", "name": "x"})))
            .with_status(200)
            .with_body(r#"{"etag": "43"}"#)
            .create_async()
            .await;

        let response: ApiResponse<Value> = send(
            &mailerlite(&server),
            Method::PATCH,
            "test/1",
            &[],
            Some(&json!({"etag": "42", "name": "x"})),
            None,
        )
        .await
        .unwrap();

        mock.assert_async().await;
        assert_eq!(response.status_code, 200);
        assert_eq!(response.body, json!({"etag": "43"}));
    }

    #[tokio::test]
    async fn delete_sends_a_body_when_there_is_one() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("DELETE", "/test/1")
            .match_body(Matcher::Json(json!({"etag": "42"})))
            .with_status(200)
            .with_body(r#"{"etag": "43"}"#)
            .create_async()
            .await;

        send::<Value, Value>(
            &mailerlite(&server),
            Method::DELETE,
            "test/1",
            &[],
            Some(&json!({"etag": "42"})),
            None,
        )
        .await
        .unwrap();

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn no_body_sends_an_empty_raw_body() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("POST", "/test/1")
            .match_request(|request| request.body().map(|body| body.is_empty()).unwrap_or(false))
            .with_status(201)
            .with_body(r#"{"data": {"id": "1"}}"#)
            .create_async()
            .await;

        let response: ApiResponse<Value> = send(
            &mailerlite(&server),
            Method::POST,
            "test/1",
            &[],
            NO_BODY,
            None,
        )
        .await
        .unwrap();

        mock.assert_async().await;
        assert_eq!(response.status_code, 201);
    }

    #[tokio::test]
    async fn query_pairs_encode_a_space_as_plus() {
        let mut server: ServerGuard = Server::new_async().await;
        let mock: Mock = server
            .mock("GET", Matcher::Any)
            .match_request(|request| {
                request.path_and_query() == "/test?filter%5Bname%5D=summer+sale&filter%5Btype%5D="
            })
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        send::<Value, ()>(
            &mailerlite(&server),
            Method::GET,
            "test",
            &[
                ("filter[name]", "summer sale".to_string()),
                ("filter[type]", String::new()),
            ],
            NO_BODY,
            None,
        )
        .await
        .unwrap();

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn idempotency_key_is_sent_only_when_given() {
        let mut server: ServerGuard = Server::new_async().await;
        let with_key: Mock = server
            .mock("POST", "/test/1")
            .match_header("Idempotency-Key", "key-1")
            .with_status(201)
            .with_body("{}")
            .create_async()
            .await;
        let without_key: Mock = server
            .mock("POST", "/test/2")
            .match_header("Idempotency-Key", Matcher::Missing)
            .with_status(201)
            .with_body("{}")
            .create_async()
            .await;
        let mailerlite: MailerLite = mailerlite(&server);

        send::<Value, ()>(
            &mailerlite,
            Method::POST,
            "test/1",
            &[],
            NO_BODY,
            Some("key-1"),
        )
        .await
        .unwrap();
        send::<Value, ()>(&mailerlite, Method::POST, "test/2", &[], NO_BODY, None)
            .await
            .unwrap();

        with_key.assert_async().await;
        without_key.assert_async().await;
    }

    #[tokio::test]
    async fn a_409_gives_the_code_and_the_etag() {
        let mut server: ServerGuard = Server::new_async().await;
        server
            .mock("PATCH", "/test/1")
            .with_status(409)
            .with_header("Content-Type", "application/json")
            .with_body(
                r#"{"message": "The email changed.", "code": "version_conflict", "etag": "44"}"#,
            )
            .create_async()
            .await;

        let error: Error = send::<Value, Value>(
            &mailerlite(&server),
            Method::PATCH,
            "test/1",
            &[],
            Some(&json!({"etag": "42"})),
            None,
        )
        .await
        .unwrap_err();

        match error {
            Error::Api(error) => {
                assert_eq!(error.status_code, 409);
                assert_eq!(error.code.as_deref(), Some("version_conflict"));
                assert_eq!(error.etag.as_deref(), Some("44"));
                assert_eq!(error.message.as_deref(), Some("The email changed."));
                assert_eq!(
                    error.headers.get("Content-Type").unwrap(),
                    "application/json"
                );
                assert!(error.is_version_conflict());
            }
            other => panic!("expected Error::Api, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn a_422_gives_the_errors() {
        let mut server: ServerGuard = Server::new_async().await;
        server
            .mock("POST", "/test/1")
            .with_status(422)
            .with_body(r#"{"message": "The type field is required.", "errors": {"type": ["The type field is required."]}}"#)
            .create_async()
            .await;

        let error: Error = send::<Value, Value>(
            &mailerlite(&server),
            Method::POST,
            "test/1",
            &[],
            Some(&json!({})),
            None,
        )
        .await
        .unwrap_err();

        match error {
            Error::Api(error) => {
                assert_eq!(error.status_code, 422);
                assert_eq!(
                    error.errors.unwrap()["type"],
                    vec!["The type field is required."]
                );
            }
            other => panic!("expected Error::Api, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn a_503_with_html_gives_the_raw_body() {
        let mut server: ServerGuard = Server::new_async().await;
        server
            .mock("GET", "/test/1")
            .with_status(503)
            .with_header("Content-Type", "text/html")
            .with_body("<html>Service Unavailable</html>")
            .create_async()
            .await;

        let error: Error = send::<Value, ()>(
            &mailerlite(&server),
            Method::GET,
            "test/1",
            &[],
            NO_BODY,
            None,
        )
        .await
        .unwrap_err();

        match error {
            Error::Api(error) => {
                assert_eq!(error.status_code, 503);
                assert_eq!(error.body, "<html>Service Unavailable</html>");
                assert!(error.message.is_none());
            }
            other => panic!("expected Error::Api, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn a_2xx_body_that_is_not_json_gives_a_decode_error() {
        let mut server: ServerGuard = Server::new_async().await;
        server
            .mock("GET", "/test/1")
            .with_status(200)
            .with_body("not json")
            .create_async()
            .await;

        let error: Error = send::<Value, ()>(
            &mailerlite(&server),
            Method::GET,
            "test/1",
            &[],
            NO_BODY,
            None,
        )
        .await
        .unwrap_err();

        match error {
            Error::Decode {
                status_code, body, ..
            } => {
                assert_eq!(status_code, 200);
                assert_eq!(body, "not json");
            }
            other => panic!("expected Error::Decode, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn a_network_error_gives_a_transport_error() {
        let mailerlite: MailerLite =
            MailerLite::new("api_key".to_string()).with_base_url("http://127.0.0.1:1/");

        let error: Error =
            send::<Value, ()>(&mailerlite, Method::GET, "test/1", &[], NO_BODY, None)
                .await
                .unwrap_err();

        assert!(matches!(error, Error::Transport(_)));
        assert!(std::error::Error::source(&error).is_some());
    }
}
