//! Update email content.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_EMAIL_HTML`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_actions::UpdateAutomationEmailContent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let email_id = std::env::var("MAILERLITE_EMAIL_ID")?;

    let params = UpdateAutomationEmailContent::new(std::env::var("MAILERLITE_EMAIL_HTML")?);

    let response = mailerlite
        .automations()
        .update_email_content(id, email_id, params)
        .await?;

    println!("Updated email {}", response.data.id);

    Ok(())
}
