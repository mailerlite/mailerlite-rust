//! Typed, async Rust SDK for the [MailerLite API](https://developers.mailerlite.com/).
//!
//! # Quick start
//!
//! ```no_run
//! use mailerlite_rs::{MailerLite, subscribers::CreateSubscriber};
//!
//! #[tokio::main]
//! async fn main() -> mailerlite_rs::Result<()> {
//!     let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
//!
//!     let upserted = client
//!         .subscribers()
//!         .create(&CreateSubscriber::new("ann@example.com").field("name", "Ann"))
//!         .await?;
//!     println!("{} (new: {})", upserted.data.id, upserted.created);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Resources
//!
//! Every API area has a handle on [`MailerLite`]:
//! [`subscribers`](MailerLite::subscribers), [`groups`](MailerLite::groups),
//! [`segments`](MailerLite::segments), [`fields`](MailerLite::fields),
//! [`forms`](MailerLite::forms), [`automations`](MailerLite::automations),
//! [`campaigns`](MailerLite::campaigns), [`webhooks`](MailerLite::webhooks),
//! [`batch`](MailerLite::batch), [`timezones`](MailerLite::timezones) and
//! [`ecommerce`](MailerLite::ecommerce). Endpoints the SDK does not model yet are reachable
//! through [`MailerLite::request`].
//!
//! # Errors and rate limits
//!
//! Every call returns [`Result`]. The errors you handle in code have their own [`Error`] variant:
//! [`Error::Validation`] (`422`, with per-field errors in the order the API sent them),
//! [`Error::RateLimited`] (`429`, with [`RateLimit::retry_after`]), [`Error::Unauthorized`]
//! (`401`), [`Error::NotFound`] (`404`) and [`Error::Api`] for any other non-2xx response.
//! The SDK never retries on its own; MailerLite allows 120 requests per minute (20 per minute
//! for imports).
//!
//! ```no_run
//! # async fn run(client: mailerlite_rs::MailerLite) -> mailerlite_rs::Result<()> {
//! use mailerlite_rs::{Error, subscribers::CreateSubscriber};
//!
//! match client.subscribers().create(&CreateSubscriber::new("ann@example")).await {
//!     Ok(created) => println!("subscribed {}", created.data.email),
//!     Err(Error::Validation(invalid)) => {
//!         for error in invalid.iter() {
//!             eprintln!("{}: {}", error.field(), error.message());
//!         }
//!     }
//!     Err(Error::RateLimited(limit)) => eprintln!("retry in {:?}", limit.retry_after()),
//!     Err(Error::NotFound(_)) => eprintln!("no such subscriber"),
//!     Err(other) => return Err(other),
//! }
//! # Ok(()) }
//! ```
//!
//! # Pagination
//!
//! List calls return one [`Page`] or [`CursorPage`]. With the `stream` feature (default),
//! `list_all` walks every page lazily:
//!
//! ```no_run
//! # #[cfg(feature = "stream")]
//! # async fn run(client: mailerlite_rs::MailerLite) -> mailerlite_rs::Result<()> {
//! use futures_util::TryStreamExt;
//!
//! let subscribers = client.subscribers();
//! let mut all = std::pin::pin!(subscribers.list_all(Default::default()));
//! while let Some(subscriber) = all.try_next().await? {
//!     println!("{}", subscriber.email);
//! }
//! # Ok(()) }
//! ```
//!
//! # Features
//!
//! | Feature             | Default | Description                                              |
//! |---------------------|---------|----------------------------------------------------------|
//! | `rustls`            | yes     | TLS via rustls.                                          |
//! | `native-tls`        |         | TLS via the platform's native stack.                     |
//! | `stream`            | yes     | `list_all` helpers returning a `futures` `Stream`.       |
//! | `webhook-signature` |         | `webhook_signature::verify` for incoming webhooks.       |
//!
//! Enable exactly one TLS feature; without one, requests fail because MailerLite is HTTPS-only.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;
mod error;
mod http;
pub mod pagination;
pub mod types;

pub mod automations;
pub mod batch;
pub mod campaigns;
pub mod ecommerce;
pub mod fields;
pub mod forms;
pub mod groups;
pub mod reports;
pub mod segments;
pub mod subscribers;
pub mod timezones;
pub mod webhooks;

#[cfg(feature = "webhook-signature")]
pub mod webhook_signature;

pub use client::{MailerLite, MailerLiteBuilder};
pub use error::{ApiError, Error, FieldError, PathSegment, RateLimit, Result, ValidationError};
pub use http::RawRequest;
pub use pagination::{CursorPage, Page};
pub use reqwest::{Method, StatusCode};

/// The HTTP client the SDK is built on, re-exported so you can build a matching
/// [`reqwest::Client`] for [`MailerLiteBuilder::http_client`].
pub use reqwest;

/// Re-exported because loosely typed API fields use [`serde_json::Value`].
pub use serde_json;
pub use types::{FieldValue, Id, Rate, Sort, Timestamp};

/// Every request goes to production. For a one-off local run, edit this constant and
/// revert it before committing.
const BASE_URL: &str = "https://connect.mailerlite.com/api";

// Clients, errors and responses are shared across tasks and threads; fail the build if a
// change makes any of them `!Send` or `!Sync`.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MailerLite>();
    assert_send_sync::<MailerLiteBuilder>();
    assert_send_sync::<Error>();
    assert_send_sync::<Page<subscribers::Subscriber>>();
    assert_send_sync::<CursorPage<subscribers::Subscriber>>();
    assert_send_sync::<ecommerce::ShopScope<'static>>();
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn futures_do_not_borrow_the_resource_handle() {
        let client = MailerLite::new("key").unwrap();
        let query = subscribers::ListSubscribers::new();

        // The handle is a temporary; the future must not borrow it.
        let future = client.subscribers().list(&query);
        drop(future);
    }

    #[test]
    fn futures_and_streams_are_send() {
        fn assert_send<T: Send>(_: &T) {}

        let client = MailerLite::new("key").unwrap();
        let query = subscribers::ListSubscribers::new();
        assert_send(&client.subscribers().list(&query));
        assert_send(
            &client
                .subscribers()
                .create(&subscribers::CreateSubscriber::new("a@b.c")),
        );

        let shop = client.ecommerce().shop("1");
        let orders = ecommerce::ListOrders::new();
        assert_send(&shop.orders().list(&orders));

        #[cfg(feature = "stream")]
        assert_send(&client.groups().list_all(groups::ListGroups::new()));
    }
}
