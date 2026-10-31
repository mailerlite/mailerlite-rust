//! Delete email block. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{MailerLite, Response, models::emails::EtagOnly};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let block_id: String = std::env::var("MAILERLITE_BLOCK_ID")?;

    let etag: String = std::env::var("MAILERLITE_EMAIL_ETAG")?;

    let response: Response<EtagOnly> = mailerlite
        .emails()
        .delete_block(email_id, block_id, etag)
        .await?;

    println!("New etag: {}", response.data.etag);

    Ok(())
}
