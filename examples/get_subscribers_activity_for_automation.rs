//! List completed subscriber executions for an automation.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`.
use mailerlite_rs::{MailerLite, resources::automations::ActivityStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let response = mailerlite
        .automations()
        .activity(id, ActivityStatus::Completed)
        .limit(10)
        .send()
        .await?;
    for activity in response.data.data {
        println!("{}: {} ({:?})", activity.id, activity.status, activity.date);
    }
    Ok(())
}
