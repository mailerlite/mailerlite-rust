//! Replace a webhook's name.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_WEBHOOK_ID`, `MAILERLITE_WEBHOOK_NAME`.
use mailerlite_rs::{MailerLite, resources::webhooks::UpdateWebhook};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_WEBHOOK_ID")?;
    let name = std::env::var("MAILERLITE_WEBHOOK_NAME")?;

    let response = mailerlite
        .webhooks()
        .update(id, UpdateWebhook::new().name(name))
        .await?;

    println!("Updated webhook: {}", response.data.id);

    Ok(())
}
