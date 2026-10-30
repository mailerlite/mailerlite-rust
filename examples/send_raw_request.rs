//! Call an endpoint the SDK doesn't model yet, with the same auth and errors.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example send_raw_request
//! ```

use mailerlite_rs::{MailerLite, Method};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let body: serde_json::Value = client
        .request(Method::GET, "subscribers")
        .query(&[("limit", "5")])
        .send()
        .await?;

    println!("{body:#}");

    Ok(())
}
