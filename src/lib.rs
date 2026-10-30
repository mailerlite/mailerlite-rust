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
//! The SDK never retries on its own; MailerLite allows 120 requests per minute (5 per minute
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
pub use reqwest::Method;
pub use types::{FieldValue, Id, Rate, Sort, Timestamp};

/// Every request goes to production. For a one-off local run, edit this constant and
/// revert it before committing.
const BASE_URL: &str = "https://connect.mailerlite.com/api";
