//! Create email template.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_TEMPLATE_NAME`.
use mailerlite_rs::models::emails::EmailResponse;
use mailerlite_rs::{MailerLite, models::emails::EmailTemplate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let response: EmailResponse<EmailTemplate> = mailerlite
        .emails()
        .create_template(email_id, std::env::var("MAILERLITE_TEMPLATE_NAME")?)
        .await?;

    println!("Template ID: {}", response.data.id);

    Ok(())
}
