//! Schedule a subscriber's data for complete forgetting.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_SUBSCRIBER_ID`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_SUBSCRIBER_ID")?;
    let receipt = mailerlite.subscribers().forget(id).await?;
    println!("{}", receipt.data.message);
    Ok(())
}
