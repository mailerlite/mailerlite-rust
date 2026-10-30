//! Create automation trigger.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::{CreateAutomationTrigger, TriggerType};
use mailerlite_rs::serde_json::{self, json};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automation_triggers()
        .create(
            "Your Automation ID",
            &CreateAutomationTrigger::new(TriggerType::SubscriberJoinsGroup).data(
                serde_json::Map::from_iter([("group_ids".into(), json!(["9"]))]),
            ),
        )
        .await?;
    println!("{response:#?}");
    Ok(())
}
