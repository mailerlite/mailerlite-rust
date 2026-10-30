//! Update automation.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::UpdateAutomation;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automations()
        .update("Your Automation ID", &UpdateAutomation::new("Updated"))
        .await?;
    println!("{response:#?}");
    Ok(())
}
