//! List the first page of draft campaigns.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`.
use mailerlite_rs::{MailerLite, resources::campaigns::CampaignStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let response = mailerlite
        .campaigns()
        .list()
        .status(CampaignStatus::Draft)
        .limit(25)
        .send()
        .await?;

    println!("Next page: {:?}", response.data.next_page());

    for campaign in response.data.data {
        println!("{}: {} ({})", campaign.id, campaign.name, campaign.status);
    }

    Ok(())
}
