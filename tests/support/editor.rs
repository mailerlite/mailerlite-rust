//! Shared wiremock helpers for simple-editor HTTP contract scenarios.
#![allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]

use mailerlite_rs::MailerLite;
use serde_json::Value;
use wiremock::{Mock, MockGuard, MockServer, Request, ResponseTemplate, matchers};

pub const BAD_IDS: [&str; 4] = ["123/html", "123%2Fhtml", "123\n", "²"];

#[allow(clippy::expect_used, reason = "The named sanitized fixture must exist")]
pub fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/fixtures/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("the fixture exists")
}

pub fn mailerlite(server: &MockServer) -> MailerLite {
    MailerLite::builder("api_key")
        .base_url(server.uri())
        .rate_limit_intervals(std::time::Duration::ZERO, std::time::Duration::ZERO)
        .build()
        .unwrap()
}

pub async fn expect_request(
    server: &MockServer,
    verb: &str,
    path_and_query: &str,
    body: Option<Value>,
    status: u16,
    response: &str,
) -> MockGuard {
    let (path, query) = match path_and_query.split_once('?') {
        Some((path, query)) => (path.to_owned(), Some(query.to_owned())),
        None => (path_and_query.to_owned(), None),
    };
    let mock = Mock::given(matchers::method(verb))
        .and(move |request: &Request| {
            request.url.path() == path && request.url.query() == query.as_deref()
        })
        .and(matchers::header("Authorization", "Bearer api_key"))
        .and(matchers::header("Accept", "application/json"))
        .and(matchers::header("Content-Type", "application/json"));
    let mock = match body {
        Some(body) => mock.and(matchers::body_json(body)),
        None => mock.and(|request: &Request| request.body.is_empty()),
    };
    mock.respond_with(
        ResponseTemplate::new(status)
            .insert_header("Content-Type", "application/json")
            .set_body_string(response),
    )
    .expect(1)
    .named(format!("{verb} {path_and_query}"))
    .mount_as_scoped(server)
    .await
}

pub async fn assert_no_request(server: &MockServer) {
    assert!(server.received_requests().await.unwrap().is_empty());
}
