//! Skip delays for selected executions.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_STEP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_actions::AutomationActivityFilter;
use mailerlite_rs::resources::automations::ActivityStatus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let params = AutomationActivityFilter::new(ActivityStatus::Active)
        .next_step(std::env::var("MAILERLITE_STEP_ID")?);
    let response = mailerlite.automations().skip_delay(id, params).await?;
    println!("HTTP {}", response.metadata.status);
    Ok(())
}
