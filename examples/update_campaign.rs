//! Update the subject and sender of an existing draft campaign.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`, `MAILERLITE_CAMPAIGN_ID`, `MAILERLITE_SENDER_EMAIL`, `MAILERLITE_SENDER_NAME`.
use mailerlite_rs::{
    MailerLite,
    resources::campaigns::{Email, UpdateCampaign},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_CAMPAIGN_ID")?;
    let sender = std::env::var("MAILERLITE_SENDER_EMAIL")?;
    let sender_name = std::env::var("MAILERLITE_SENDER_NAME")?;
    let email = Email::new("Updated newsletter subject", sender_name, sender);
    let response = mailerlite
        .campaigns()
        .update(id, UpdateCampaign::new("Updated newsletter", email))
        .await?;
    println!(
        "Updated campaign {}: {}",
        response.data.id, response.data.name
    );
    Ok(())
}
