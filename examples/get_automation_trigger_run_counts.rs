//! Get automation trigger run counts.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automation_triggers()
        .run_count("Your Automation ID", "Your Step or Email ID")
        .await?;
    println!("{response:#?}");
    Ok(())
}
