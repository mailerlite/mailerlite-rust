//! Configure and inject a reusable HTTP client.
//!
//! Requires `MAILERLITE_API_TOKEN` and `MAILERLITE_API_VERSION` (your chosen `YYYY-MM-DD` date).
//! Makes no API request. MailerLite recommends the date when you implement the integration:
//! <https://developers.mailerlite.com/getting-started#versioning>.
use mailerlite_rs::{ApiVersion, MailerLite, RetryPolicy};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version_date = std::env::var("MAILERLITE_API_VERSION")?;

    let http = MailerLite::http_client_builder()
        .connect_timeout(Duration::from_secs(5))
        .build()?;

    let mailerlite = MailerLite::builder(std::env::var("MAILERLITE_API_TOKEN")?)
        .http_client(http)
        // Optional: pin your chosen date instead of using the latest version.
        .api_version(ApiVersion::pinned(version_date)?)
        .request_timeout(Duration::from_secs(30))
        .operation_timeout(Duration::from_secs(60))
        // Optional: enable retries for safe reads (disabled by default).
        .retry_policy(RetryPolicy::safe_reads().max_retries(2))
        .build()?;

    println!("{mailerlite:?}"); // Token is redacted.

    Ok(())
}
