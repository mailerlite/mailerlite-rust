//! Delete automation step.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::{DeleteAutomationStep, WorkflowBranch};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    client
        .automation_steps()
        .delete(
            "Your Automation ID",
            "Your Step or Email ID",
            &DeleteAutomationStep::new().branch_to_delete(WorkflowBranch::All),
        )
        .await?;
    println!("Request accepted");
    Ok(())
}
