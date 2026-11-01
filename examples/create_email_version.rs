//! Create email version.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`.
use mailerlite_rs::{
    MailerLite, Response,
    models::emails::{EmailResponseMetadata, EmailVersion},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let response: Response<EmailVersion, EmailResponseMetadata> =
        mailerlite.emails().create_version(email_id).await?;

    let is_new: bool = response.metadata.http.status == 201;

    println!("Version ID: {}, new: {}", response.data.id, is_new);

    Ok(())
}
