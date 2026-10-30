//! Assign selected automation subscribers to groups.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_GROUP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_actions::{
    AddAutomationSubscribersToGroup, AutomationActivityFilter,
};
use mailerlite_rs::resources::automations::ActivityStatus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let params = AddAutomationSubscribersToGroup::new(
        std::env::var("MAILERLITE_GROUP_ID")?,
        AutomationActivityFilter::new(ActivityStatus::Active),
    );
    let response = mailerlite
        .automations()
        .add_subscribers_to_group(id, params)
        .await?;
    println!("HTTP {}", response.metadata.status);
    Ok(())
}
