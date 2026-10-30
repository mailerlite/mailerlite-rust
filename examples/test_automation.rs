//! Send every sendable automation email to a verified recipient on an active account.
//! The server allows 10 requests per hour; success confirms background job acceptance.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_SUBSCRIBER_EMAIL`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_actions::TestAutomation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;

    let params = TestAutomation::new(std::env::var("MAILERLITE_SUBSCRIBER_EMAIL")?);

    let response = mailerlite.automations().test(id, params).await?;

    println!("HTTP {}", response.metadata.status);

    Ok(())
}
