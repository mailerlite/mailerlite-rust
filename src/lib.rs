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
