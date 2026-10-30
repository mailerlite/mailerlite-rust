//! Update the group associated with an automation trigger.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_TRIGGER_ID`, `MAILERLITE_GROUP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_triggers::UpdateAutomationTrigger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let automation_id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let trigger_id = std::env::var("MAILERLITE_TRIGGER_ID")?;
    let group_id = std::env::var("MAILERLITE_GROUP_ID")?;

    let response = mailerlite
        .automation_triggers()
        .update(
            automation_id,
            trigger_id,
            UpdateAutomationTrigger::new().group_ids([group_id]),
        )
        .await?;

    println!("HTTP {}", response.metadata.status);

    Ok(())
}
