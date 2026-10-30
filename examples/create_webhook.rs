//! Subscribe to subscriber creation events.
//! Requires MAILERLITE_API_TOKEN and MAILERLITE_WEBHOOK_URL.
use mailerlite_rs::{
    MailerLite, models::webhooks::WebhookEvent, resources::webhooks::CreateWebhook,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let url = std::env::var("MAILERLITE_WEBHOOK_URL")?;
    let params =
        CreateWebhook::new(url, [WebhookEvent::SubscriberCreated]).name("Subscriber notifications");
    let response = mailerlite.webhooks().create(params).await?;
    println!("Webhook ID: {}", response.data.id);
    // Store the signing secret securely through its explicit accessor when needed.
    Ok(())
}
