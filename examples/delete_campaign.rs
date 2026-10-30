//! Delete an existing campaign.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`, `MAILERLITE_CAMPAIGN_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_CAMPAIGN_ID")?;
    let response = mailerlite.campaigns().delete(&id).await?;
    println!("Deleted campaign {id}: {}", response.metadata.status);
    Ok(())
}
