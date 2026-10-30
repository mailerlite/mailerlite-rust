//! Create automation step.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::{CreateAutomationStep, StepType, WorkflowBranch};
use mailerlite_rs::serde_json::{self, json};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automation_steps()
        .create(
            "Your Automation ID",
            &CreateAutomationStep::new(StepType::Delay)
                .parent_id("Your Step or Email ID")
                .branch(WorkflowBranch::Yes)
                .move_to_branch(WorkflowBranch::No)
                .data(serde_json::Map::from_iter([
                    ("unit".into(), json!("days")),
                    ("value".into(), json!(1)),
                ])),
        )
        .await?;
    println!("{response:#?}");
    Ok(())
}
