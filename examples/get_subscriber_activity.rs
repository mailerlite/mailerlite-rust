//! Fetch one page of a subscriber's activity log.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_SUBSCRIBER_ID`.

use mailerlite_rs::{MailerLite, models::subscribers::ActivityProperties};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_SUBSCRIBER_ID")?;
    let activity = mailerlite
        .subscribers()
        .activity(id)
        .page(1)
        .limit(25)
        .send()
        .await?;
    for entry in activity.data {
        println!("{}: {}", entry.id, entry.log_name);
        match entry.properties {
            ActivityProperties::CampaignEmail(email) => {
                println!("  Campaign: {}", email.campaign_id);
            }
            ActivityProperties::AutomationEmail(email) => {
                println!(
                    "  Automation: {}; email: {}",
                    email.automation_id, email.email_id
                );
            }
            ActivityProperties::Group(group) => println!("  Group: {}", group.group_id),
            _ => {}
        }
    }
    Ok(())
}
