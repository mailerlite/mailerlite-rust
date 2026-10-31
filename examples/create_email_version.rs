//! Create email version. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{MailerLite, Response, models::emails::EmailVersion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let response: Response<EmailVersion> = mailerlite.emails().create_version(email_id).await?;

    let is_new: bool = response.metadata.status == 201;

    println!("Version ID: {}, new: {}", response.data.id, is_new);

    Ok(())
}
