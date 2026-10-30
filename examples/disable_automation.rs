//! Disable an automation.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let response = mailerlite.automations().disable(id).await?;
    println!("Automation {}: {}", response.data.id, response.data.name);
    Ok(())
}
