//! Fetch run counts for an automation step.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_STEP_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let automation_id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let step_id = std::env::var("MAILERLITE_STEP_ID")?;
    let response = mailerlite
        .automation_steps()
        .run_count(automation_id, step_id)
        .await?;
    println!(
        "{} completed; {} queued",
        response.data.completed_step_runs, response.data.in_queue_step_runs
    );
    Ok(())
}
