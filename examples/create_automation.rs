//! Create a disabled automation draft.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_NAME`.
use mailerlite_rs::{MailerLite, resources::automations::CreateAutomation};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let name = std::env::var("MAILERLITE_AUTOMATION_NAME")?;

    let response = mailerlite
        .automations()
        .create(CreateAutomation::new(name))
        .await?;

    println!(
        "Draft automation {}: {}",
        response.data.id, response.data.name
    );

    Ok(())
}
