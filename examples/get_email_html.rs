//! Get email html. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{MailerLite, Response, models::emails::EmailHtml, models::emails::WithEtag};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let response: Response<WithEtag<EmailHtml>> = mailerlite.emails().html(email_id).await?;

    println!("{}", response.data.data.html);

    Ok(())
}
