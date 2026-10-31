//! Read run counts for all automation triggers.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let automation_id = std::env::var("MAILERLITE_AUTOMATION_ID")?;

    let response = mailerlite
        .automation_triggers()
        .run_counts(automation_id)
        .await?;

    println!("HTTP {}", response.metadata.status);

    Ok(())
}
