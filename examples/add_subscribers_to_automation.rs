//! Enroll subscribers; this can send email.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_SUBSCRIBER_EMAIL`, `MAILERLITE_GROUP_ID`, `MAILERLITE_STEP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_actions::AddAutomationSubscribers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;

    let params = AddAutomationSubscribers::new()
        .email(std::env::var("MAILERLITE_SUBSCRIBER_EMAIL")?)
        .group(std::env::var("MAILERLITE_GROUP_ID")?)
        .step_id(std::env::var("MAILERLITE_STEP_ID")?);

    let response = mailerlite.automations().add_subscribers(id, params).await?;

    println!("HTTP {}", response.metadata.status);

    Ok(())
}
