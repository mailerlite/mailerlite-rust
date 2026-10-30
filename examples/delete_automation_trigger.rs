//! Delete automation trigger.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    client
        .automation_triggers()
        .delete("Your Automation ID", "Your Step or Email ID")
        .await?;
    println!("Request accepted");
    Ok(())
}
