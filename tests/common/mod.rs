//! Shared helpers for the integration tests.

#![allow(dead_code)]

use mailerlite_rs::MailerLite;
use mockito::{Matcher, Mock, ServerGuard};
use serde_json::Value;

/// The bad IDs that every ID check must reject. The types stop a null or a boolean ID at compile time.
pub const BAD_IDS: [&str; 4] = ["123/html", "123%2Fhtml", "123\n", "²"];

/// Returns the content of `tests/fixtures/<name>.json`.
pub fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/fixtures/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("the fixture exists")
}

/// Returns a client that sends its requests to the mock server.
pub fn mailerlite(server: &ServerGuard) -> MailerLite {
    MailerLite::new("api_key".to_string()).with_base_url(server.url())
}

/// Starts a mock that expects one request with the full path and query, the default headers and the
/// exact JSON body. For `body: None`, the mock expects an empty raw body.
pub async fn expect_request(
    server: &mut ServerGuard,
    method: &str,
    path_and_query: &str,
    body: Option<Value>,
    status: usize,
    response: &str,
) -> Mock {
    let mock: Mock = server
        .mock(method, path_and_query)
        .match_header("Authorization", "Bearer api_key")
        .match_header("Accept", "application/json")
        .match_header("Content-Type", "application/json");
    let mock: Mock = match body {
        Some(body) => mock.match_body(Matcher::Json(body)),
        None => mock.match_request(|request| request.body().is_ok_and(|body| body.is_empty())),
    };

    mock.with_status(status)
        .with_header("Content-Type", "application/json")
        .with_body(response)
        .expect(1)
        .create_async()
        .await
}

/// Starts mocks that fail the test if the SDK sends any request. Check them with `assert_no_request`.
pub async fn expect_no_request(server: &mut ServerGuard) -> Vec<Mock> {
    let mut mocks: Vec<Mock> = Vec::new();
    for method in ["GET", "POST", "PUT", "PATCH", "DELETE"] {
        mocks.push(
            server
                .mock(method, Matcher::Any)
                .expect(0)
                .create_async()
                .await,
        );
    }

    mocks
}

/// Checks that the mocks of `expect_no_request` got no request.
pub async fn assert_no_request(mocks: &[Mock]) {
    for mock in mocks {
        mock.assert_async().await;
    }
}
