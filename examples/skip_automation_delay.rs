//! Skip automation delay.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::{
    AutomationActivityStatus, AutomationRunAction, AutomationRunFilter,
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    client
        .automations()
        .skip_delay(
            "Your Automation ID",
            &AutomationRunAction::new(
                AutomationRunFilter::new(AutomationActivityStatus::Active).id(vec!["7".into()]),
            ),
        )
        .await?;
    println!("Request accepted");
    Ok(())
}
