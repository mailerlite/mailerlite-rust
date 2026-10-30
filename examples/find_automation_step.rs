//! Find automation step.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automation_steps()
        .get("Your Automation ID", "Your Step or Email ID")
        .await?;
    println!("{response:#?}");
    Ok(())
}
