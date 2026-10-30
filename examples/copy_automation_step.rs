//! Copy automation step.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::{CopyAutomationStep, WorkflowBranch};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automation_steps()
        .copy(
            "Your Automation ID",
            "Your Step or Email ID",
            &CopyAutomationStep::new()
                .parent_id("Your Step or Email ID")
                .branch(WorkflowBranch::Yes)
                .move_to_branch(WorkflowBranch::No),
        )
        .await?;
    println!("{response:#?}");
    Ok(())
}
