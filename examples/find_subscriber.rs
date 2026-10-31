//! Fetch a subscriber by ID or email.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_SUBSCRIBER_ID_OR_EMAIL`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id_or_email = std::env::var("MAILERLITE_SUBSCRIBER_ID_OR_EMAIL")?;

    let subscriber = mailerlite.subscribers().get(id_or_email).await?;

    println!(
        "Subscriber: {} ({})",
        subscriber.data.id, subscriber.data.email
    );

    Ok(())
}
