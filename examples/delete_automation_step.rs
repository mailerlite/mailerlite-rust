//! Delete an automation step and its branches.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_STEP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_steps::{DeleteBranch, DeleteStep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let automation_id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let step_id = std::env::var("MAILERLITE_STEP_ID")?;

    let response = mailerlite
        .automation_steps()
        .delete(
            automation_id,
            step_id,
            DeleteStep::new().branch_to_delete(DeleteBranch::All),
        )
        .await?;

    println!("HTTP {}", response.metadata.status);

    Ok(())
}
