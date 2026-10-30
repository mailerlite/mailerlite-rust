//! List subscriber activity of an automation: `GET /automations/{id}/activity`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_subscribers_activity_for_automation
//! ```

use mailerlite_rs::{
    MailerLite,
    automations::{AutomationActivityStatus, ListAutomationActivity},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client
        .automations()
        .activity(
            "Your Automation ID",
            &ListAutomationActivity::new(AutomationActivityStatus::Completed),
        )
        .await?;

    println!("{page:#?}");

    Ok(())
}
