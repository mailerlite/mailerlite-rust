//! Rename an automation.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_AUTOMATION_NAME`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_actions::UpdateAutomation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let params = UpdateAutomation::new(std::env::var("MAILERLITE_AUTOMATION_NAME")?);
    let response = mailerlite.automations().update(id, params).await?;
    println!("Automation {}: {}", response.data.id, response.data.name);
    Ok(())
}
