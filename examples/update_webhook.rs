//! Rename a webhook. Requires MAILERLITE_API_TOKEN and MAILERLITE_WEBHOOK_ID.
use mailerlite_rs::{MailerLite, resources::webhooks::UpdateWebhook};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_WEBHOOK_ID")?;

    let response = mailerlite
        .webhooks()
        .update(id, UpdateWebhook::new().name("Updated notifications"))
        .await?;

    println!("Updated webhook: {}", response.data.id);

    Ok(())
}
