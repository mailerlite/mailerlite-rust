//! List webhook subscriptions. Requires MAILERLITE_API_TOKEN.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let response = mailerlite.webhooks().list().await?;
    for webhook in response.data.data {
        println!("{}: {}", webhook.id, webhook.url);
    }
    Ok(())
}
