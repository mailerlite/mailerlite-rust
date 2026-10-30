//! Create or update a subscriber by email.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_SUBSCRIBER_EMAIL`.
use mailerlite_rs::{MailerLite, resources::subscribers::UpsertSubscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let email = std::env::var("MAILERLITE_SUBSCRIBER_EMAIL")?;

    let response = mailerlite
        .subscribers()
        .upsert(UpsertSubscriber::new(email).field("name", "Jane"))
        .await?;

    println!("Subscriber ID: {}", response.data.id);

    Ok(())
}
