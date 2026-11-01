//! Restore email version.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_VERSION_ID`,
//! `MAILERLITE_EMAIL_ETAG`.
use mailerlite_rs::{
    MailerLite, Response,
    models::emails::{EmailDocument, EmailResponseMetadata, WithEtag},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let version_id: String = std::env::var("MAILERLITE_VERSION_ID")?;

    let etag: String = std::env::var("MAILERLITE_EMAIL_ETAG")?;

    let response: Response<WithEtag<EmailDocument>, EmailResponseMetadata> = mailerlite
        .emails()
        .restore_version(email_id, version_id, etag)
        .await?;

    println!("New etag: {}", response.data.etag);

    Ok(())
}
