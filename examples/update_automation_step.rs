//! Update automation step.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::UpdateAutomationStep;
use mailerlite_rs::serde_json::{self, json};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automation_steps()
        .update(
            "Your Automation ID",
            "Your Step or Email ID",
            &UpdateAutomationStep::new(serde_json::Map::from_iter([
                ("unit".into(), json!("days")),
                ("value".into(), json!(2)),
            ])),
        )
        .await?;
    println!("{response:#?}");
    Ok(())
}
