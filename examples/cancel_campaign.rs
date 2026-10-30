//! Cancel an existing campaign that is in the ready state.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`, `MAILERLITE_CAMPAIGN_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_CAMPAIGN_ID")?;
    let response = mailerlite.campaigns().cancel(id).await?;
    println!("Campaign {}: {}", response.data.id, response.data.status);
    Ok(())
}
