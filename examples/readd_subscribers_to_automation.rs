//! Re-enroll selected subscribers.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_actions::{
    AutomationActivityFilter, ReaddAutomationSubscribers, ReaddDestination,
};
use mailerlite_rs::resources::automations::ActivityStatus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;

    let params = ReaddAutomationSubscribers::new(
        ReaddDestination::LastCompletedStep,
        AutomationActivityFilter::new(ActivityStatus::Completed),
    );

    let response = mailerlite
        .automations()
        .readd_subscribers(id, params)
        .await?;

    println!("HTTP {}", response.metadata.status);

    Ok(())
}
