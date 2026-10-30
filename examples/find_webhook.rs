//! Fetch a webhook. Requires MAILERLITE_API_TOKEN and MAILERLITE_WEBHOOK_ID.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_WEBHOOK_ID")?;
    let response = mailerlite.webhooks().get(id).await?;
    println!("{}: {}", response.data.id, response.data.url);
    Ok(())
}
