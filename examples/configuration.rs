//! Configure and inject a reusable HTTP client.
//!
//! Requires `MAILERLITE_API_TOKEN`. Uses `ApiVersion::Latest` unless the optional
//! `MAILERLITE_API_VERSION` is set to your chosen `YYYY-MM-DD` date. Present but
//! blank values are errors. `ApiVersion::pinned` trims surrounding whitespace.
//! Makes no API request. MailerLite recommends the date when you implement the integration:
//! <https://developers.mailerlite.com/getting-started#versioning>.
use mailerlite_rs::{ApiVersion, MailerLite, RetryPolicy};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_version = match std::env::var("MAILERLITE_API_VERSION") {
        Ok(date) => ApiVersion::pinned(date)?,
        Err(std::env::VarError::NotPresent) => ApiVersion::Latest,
        Err(error) => return Err(error.into()),
    };

    let http = MailerLite::http_client_builder()
        .connect_timeout(Duration::from_secs(5))
        .build()?;

    let mailerlite = MailerLite::builder(std::env::var("MAILERLITE_API_TOKEN")?)
        .http_client(http)
        .api_version(api_version)
        .request_timeout(Duration::from_secs(30))
        .operation_timeout(Duration::from_secs(60))
        // Optional: enable retries for safe reads (disabled by default).
        .retry_policy(RetryPolicy::safe_reads().max_retries(2))
        .build()?;

    println!("{mailerlite:?}"); // Token is redacted.

    Ok(())
}
