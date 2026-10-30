//! Schedule an existing campaign draft for a future date.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`, `MAILERLITE_CAMPAIGN_ID`,
//! and `MAILERLITE_SEND_DATE` (YYYY-MM-DD, in the future).
use mailerlite_rs::{MailerLite, resources::campaigns::ScheduleCampaign};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let campaign = std::env::var("MAILERLITE_CAMPAIGN_ID")?;
    let date = std::env::var("MAILERLITE_SEND_DATE")?; // YYYY-MM-DD, in the future.
    let result = mailerlite
        .campaigns()
        .schedule(campaign, ScheduleCampaign::scheduled(date, "10", "00"))
        .await?;
    println!("Campaign {}: {}", result.data.id, result.data.status);
    Ok(())
}
