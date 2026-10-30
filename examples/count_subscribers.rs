//! Fetch the total subscriber count.
//!
//! Requires `MAILERLITE_API_TOKEN`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let count = mailerlite.subscribers().count().await?;
    println!("Total subscribers: {}", count.data.total);
    Ok(())
}
