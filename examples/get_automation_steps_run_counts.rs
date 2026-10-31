//! Fetch run counts for all automation steps.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let automation_id = std::env::var("MAILERLITE_AUTOMATION_ID")?;

    let response = mailerlite
        .automation_steps()
        .run_counts(automation_id)
        .await?;

    println!("{} step counts", response.data.len());

    Ok(())
}
