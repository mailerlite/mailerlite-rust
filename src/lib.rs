#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
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
pub use config::{ApiVersion, DEFAULT_API_VERSION, MailerLiteBuilder};
pub use error::{ApiError, DecodeError, Error, Result};
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
