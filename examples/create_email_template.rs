//! Create email template. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{MailerLite, Response, models::emails::EmailTemplate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let response: Response<EmailTemplate> = mailerlite
        .emails()
        .create_template(email_id, std::env::var("MAILERLITE_TEMPLATE_NAME")?)
        .await?;

    println!("Template ID: {}", response.data.id);

    Ok(())
}
