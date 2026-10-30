//! Create a webhook: `POST /webhooks`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_webhook
//! ```

use mailerlite_rs::{
    MailerLite,
    webhooks::{CreateWebhook, WebhookEvent},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let webhook = client
        .webhooks()
        .create(
            &CreateWebhook::new(
                "https://example.com/webhook",
                [WebhookEvent::SubscriberCreated],
            )
            .name("New webhook"),
        )
        .await?;

    println!("{webhook:#?}");

    Ok(())
}
