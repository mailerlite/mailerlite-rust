//! Fetch subscribers queued at an automation step.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_STEP_ID`, `MAILERLITE_SUBSCRIBER_EMAIL`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_steps::{StepSubscriberStatus, StepSubscribers};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let automation_id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let step_id = std::env::var("MAILERLITE_STEP_ID")?;
    let subscriber_email = std::env::var("MAILERLITE_SUBSCRIBER_EMAIL")?;
    let response = mailerlite
        .automation_steps()
        .subscribers(
            automation_id,
            step_id,
            StepSubscribers::new(StepSubscriberStatus::Queued)
                .search(subscriber_email)
                .limit(25),
        )
        .await?;
    println!(
        "{} subscriber executions; next page: {:?}",
        response.data.data.len(),
        response.data.next_page()
    );
    Ok(())
}
