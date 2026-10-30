//! Select automation ab test winner.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::{SelectAutomationWinner, WorkflowBranch};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automation_steps()
        .select_ab_test_winner(
            "Your Automation ID",
            "Your Step or Email ID",
            &SelectAutomationWinner::new(WorkflowBranch::APath),
        )
        .await?;
    println!("{response:#?}");
    Ok(())
}
