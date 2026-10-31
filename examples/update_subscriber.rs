//! Replace a subscriber's name custom field.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_SUBSCRIBER_ID`, `MAILERLITE_SUBSCRIBER_NAME`.

use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::subscribers::UpdateSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_SUBSCRIBER_ID")?;
    let name = std::env::var("MAILERLITE_SUBSCRIBER_NAME")?;

    let subscriber = mailerlite
        .subscribers()
        .update(id, UpdateSubscriber::new().field("name", name))
        .await?;

    println!("Updated subscriber: {}", subscriber.data.id);

    Ok(())
}
