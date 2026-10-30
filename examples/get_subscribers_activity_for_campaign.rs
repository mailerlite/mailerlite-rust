//! List click activity for a sent campaign.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`, `MAILERLITE_CAMPAIGN_ID`.
use mailerlite_rs::{MailerLite, resources::campaigns::ActivityType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_CAMPAIGN_ID")?;
    let response = mailerlite
        .campaigns()
        .activity(id)
        .kind(ActivityType::Clicked)
        .limit(25)
        .send()
        .await?;
    for activity in response.data.data {
        println!("{}: {:?} clicks", activity.id, activity.clicks_count);
    }
    Ok(())
}
