//! E-commerce endpoint contracts, grouped by resource and operation.
use mailerlite_rs::{
    MailerLite,
    resources::ecommerce::{Sort, params::*},
};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path, query_param},
};

mod cart_items;
mod carts;
mod categories;
mod customers;
mod imports;
mod orders;
mod product_details;
mod products;
mod reports;
mod settings;
mod shops;
mod variants;

fn client(server: &MockServer) -> MailerLite {
    crate::support::default_client(server)
}

async fn mount(
    server: &MockServer,
    verb: &str,
    route: &str,
    payload: Option<Value>,
    response: Value,
    status: u16,
) {
    let mut mock = Mock::given(method(verb))
        .and(path(format!("/api{route}")))
        .and(header("authorization", "Bearer test-token"))
        .and(header("accept", "application/json"))
        .and(header("content-type", "application/json"))
        .and(|request: &wiremock::Request| !request.headers.contains_key("x-version"));
    if let Some(payload) = payload {
        mock = mock.and(body_json(payload));
    }
    mock.respond_with(ResponseTemplate::new(status).set_body_json(response))
        .expect(1)
        .mount(server)
        .await;
}
