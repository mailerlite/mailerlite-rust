//! Get a webhook: `GET /webhooks/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example find_webhook
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let webhook = client.webhooks().get("Your Webhook ID").await?;

    println!("{webhook:#?}");

    Ok(())
}
