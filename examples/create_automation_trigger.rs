//! Create a subscriber-joins-group automation trigger.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_GROUP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_triggers::CreateAutomationTrigger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let automation_id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let group_id = std::env::var("MAILERLITE_GROUP_ID")?;

    let response = mailerlite
        .automation_triggers()
        .create(
            automation_id,
            CreateAutomationTrigger::new("subscriber_joins_group").group_ids([group_id]),
        )
        .await?;

    println!("HTTP {}", response.metadata.status);

    Ok(())
}
