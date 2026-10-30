//! Copy an automation step.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_STEP_ID`, `MAILERLITE_PARENT_STEP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_steps::CopyStep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let automation_id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let step_id = std::env::var("MAILERLITE_STEP_ID")?;
    let parent_step_id = std::env::var("MAILERLITE_PARENT_STEP_ID")?;
    let response = mailerlite
        .automation_steps()
        .copy(
            automation_id,
            step_id,
            CopyStep::new().parent_id(parent_step_id),
        )
        .await?;
    println!("Updated automation {}", response.data.id);
    Ok(())
}
