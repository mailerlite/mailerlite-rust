use mailerlite_rs::{MailerLite, RetryPolicy};
use serde_json::Value;
use std::time::Duration;
use wiremock::MockServer;

/// Resource-test client using the SDK's default no-retry policy.
#[allow(
    dead_code,
    reason = "Only some integration targets need a no-retry client"
)]
pub fn default_client(server: &MockServer) -> MailerLite {
    MailerLite::builder("test-token")
        .base_url(format!("{}/api", server.uri()))
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .build()
        .unwrap()
}

/// Expect one resource request, including the shared HTTP headers.
#[allow(dead_code, reason = "Only some integration targets use resource mocks")]
pub async fn resource_mock(
    server: &MockServer,
    verb: &str,
    route: &str,
    status: u16,
    response: Option<Value>,
    body: Option<Value>,
) {
    use wiremock::{
        Mock, ResponseTemplate,
        matchers::{body_json, header, method, path},
    };
    let mut request = Mock::given(method(verb))
        .and(path(route))
        .and(header("authorization", "Bearer test-token"))
        .and(header("accept", "application/json"))
        .and(header("content-type", "application/json"))
        .and(|request: &wiremock::Request| !request.headers.contains_key("x-version"));
    if let Some(body) = body {
        request = request.and(body_json(body));
    }
    let mut template = ResponseTemplate::new(status);
    if let Some(response) = response {
        template = template.set_body_json(response);
    }
    request.respond_with(template).expect(1).mount(server).await;
}

pub fn subscriber() -> Value {
    serde_json::from_str(include_str!("../fixtures/subscriber.json")).unwrap()
}
/// Client that explicitly enables immediate safe-read retries for policy tests.
pub fn client(server: &MockServer) -> MailerLite {
    MailerLite::builder("private-test-token")
        .base_url(format!("{}/api", server.uri()))
        .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
        .retry_policy(
            RetryPolicy::safe_reads()
                .initial_delay(Duration::ZERO)
                .max_delay(Duration::ZERO),
        )
        .build()
        .unwrap()
}
