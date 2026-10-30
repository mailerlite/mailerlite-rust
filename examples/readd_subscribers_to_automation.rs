//! Readd subscribers to automation.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::{
    AutomationActivityStatus, AutomationRunFilter, ReaddAutomationSubscribers, ReaddTo,
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    client
        .automations()
        .readd_subscribers(
            "Your Automation ID",
            &ReaddAutomationSubscribers::new(
                AutomationRunFilter::new(AutomationActivityStatus::Completed),
                ReaddTo::SelectedStep,
            )
            .selected_step_id("Your Step or Email ID"),
        )
        .await?;
    println!("Request accepted");
    Ok(())
}
