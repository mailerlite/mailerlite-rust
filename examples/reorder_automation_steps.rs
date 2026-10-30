//! Reorder automation steps.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::{ReorderAutomationSteps, WorkflowBranch};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automation_steps()
        .reorder(
            "Your Automation ID",
            &ReorderAutomationSteps::new("Your Step or Email ID".into())
                .new_parent_step_id("3")
                .branch(WorkflowBranch::Yes)
                .branch_for_child(WorkflowBranch::No),
        )
        .await?;
    println!("{response:#?}");
    Ok(())
}
