//! Update a webhook: `PUT /webhooks/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_webhook
//! ```

use mailerlite_rs::{MailerLite, webhooks::UpdateWebhook};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let webhook = client
        .webhooks()
        .update(
            "Your Webhook ID",
            &UpdateWebhook::new().name("Renamed webhook"),
        )
        .await?;

    println!("{webhook:#?}");

    Ok(())
}
