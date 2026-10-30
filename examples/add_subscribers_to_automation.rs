//! Add subscribers to automation.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::AddAutomationSubscribers;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    client
        .automations()
        .add_subscribers(
            "Your Automation ID",
            &AddAutomationSubscribers::new()
                .emails(vec!["ann@example.com".into()])
                .groups(vec!["9".into()])
                .step_id("Your Step or Email ID"),
        )
        .await?;
    println!("Request accepted");
    Ok(())
}
