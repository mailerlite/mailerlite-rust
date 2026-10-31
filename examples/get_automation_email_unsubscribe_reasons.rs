//! Read email unsubscribe reasons.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_EMAIL_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let email_id = std::env::var("MAILERLITE_EMAIL_ID")?;

    let response = mailerlite
        .automations()
        .email_unsubscribe_reasons(id, email_id)
        .await?;

    for reason in response.data {
        println!(
            "{}: {} ({:.2}%)",
            reason.label, reason.count, reason.percentage
        );
    }

    Ok(())
}
