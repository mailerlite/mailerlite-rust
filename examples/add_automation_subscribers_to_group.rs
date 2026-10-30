//! Add automation subscribers to group.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::{
    AutomationActivityStatus, AutomationRunFilter, CopyAutomationSubscribers,
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    client
        .automations()
        .add_subscribers_to_group(
            "Your Automation ID",
            &CopyAutomationSubscribers::new(
                AutomationRunFilter::new(AutomationActivityStatus::Active),
                vec!["9".into()],
            ),
        )
        .await?;
    println!("Request accepted");
    Ok(())
}
