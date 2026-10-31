//! MailerLite Rust SDK.
//!
//! A typed, asynchronous client for the current MailerLite REST API. Requires
//! Rust 1.88 or newer and a Tokio runtime. Start with [`MailerLite`], choose an
//! operation under [`resources`], and read typed results from [`Response`].
//!
//! # Quick start
//!
//! Pass your bearer token directly; the SDK does not load `.env` files.
//! This example creates a subscriber or updates the subscriber with that email.
//!
//! ```no_run
//! use mailerlite_rs::{MailerLite, resources::subscribers::UpsertSubscriber};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
//!
//!     let response = mailerlite
//!         .subscribers()
//!         .upsert(UpsertSubscriber::new("jane@example.com").field("name", "Jane"))
//!         .await?;
//!
//!     println!("Subscriber ID: {}", response.data.id);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Configuration
//!
//! Reuse one client per configuration. Clones share HTTP connections, local
//! pacing, and server cooldowns. [`MailerLite::builder`] configures timeouts,
//! API version, transport, and retry policy; use [`MailerLite::http_client_builder`]
//! when supplying your own HTTP client.
//!
//! Defaults are [`ApiVersion::Latest`] (no `X-Version` header), a 30-second
//! request timeout, and a 60-second operation deadline. Automatic retries are
//! disabled. [`RetryPolicy::safe_reads`] enables bounded retries for reviewed
//! reads; writes and batches are never automatically replayed.
//!
//! # API navigation
//!
//! - [`resources`]: endpoint operations and typed request parameters, including
//!   subscribers, campaigns, automations, ecommerce, and webhook management.
//! - [`models`]: response fields, identifiers, and automation configuration.
//! - [`Response`] and [`ResponseMetadata`]: results and selected HTTP metadata.
//! - [`Error`] and [`ApiError`]: matchable failures, status, and validation details.
//!   Explicit message/body accessors may contain sensitive submitted data.
//! - [`pagination`]: [`CursorPage`](pagination::CursorPage) and
//!   [`NumberedPage`](pagination::NumberedPage). Items are in `response.data.data`.
//!   Enable `stream` for lazy pagination on supported list builders. Server links
//!   are never followed directly; subscriber activity remains manually paged.
//! - [`resources::imports`]: import receipts, progress checks, and explicit polling.
//! - [`resources::batch`]: batch requests whose item statuses must be checked
//!   independently of the outer HTTP status.
//! - [`webhooks`]: incoming payloads and optional signature verification. Verify
//!   the original body bytes with the signing secret before parsing JSON;
//!   signature validity does not prevent duplicate delivery or establish freshness.
//!
//! # Features
//!
//! | Feature | Behavior |
//! | --- | --- |
//! | `rustls-tls` | Default HTTPS backend |
//! | `native-tls` | System TLS; disable default features to select it alone |
//! | `stream` | Lazy pagination streams |
//! | `tracing` | Transport events without credentials or request/response bodies |
//! | `webhook-signatures` | HMAC-SHA256 verification of incoming webhook bytes |
//!
//! The [repository](https://github.com/mailerlite/mailerlite-rust) contains the
//! README, upgrade guide, contributor guide, and standalone examples. These are
//! repository files, not generated rustdoc pages.

mod client;
mod config;
mod error;
pub(crate) mod http;
pub mod models;
pub mod pagination;
pub mod resources;
pub mod response;
pub mod webhooks;
pub use client::MailerLite;
pub use config::{ApiVersion, ApiVersionDate, MailerLiteBuilder};
pub use error::{ApiError, DecodeError, Error, JsonDecodeError, Result};
pub use http::retry::RetryPolicy;
pub use response::{Response, ResponseMetadata};

/// HTTP status type returned in response metadata and API errors.
///
/// This is the same type as `reqwest::StatusCode`, not an SDK wrapper.
pub use reqwest::StatusCode;
/// Exact decimal type used for prices, monetary totals, and booking durations.
///
/// This is the same type as `rust_decimal::Decimal`, not an SDK wrapper.
///
/// # Examples
///
/// ```
/// use mailerlite_rs::{Decimal, resources::ecommerce::params::SaveProduct};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let price: Decimal = "19.99".parse()?;
/// let product = SaveProduct::new("Guide").price(price);
/// # let _ = product;
/// # Ok(())
/// # }
/// ```
pub use rust_decimal::Decimal;
