//! Update email block. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{MailerLite, Response, models::emails::EmailBlock, models::emails::WithEtag};
use serde_json::{Map, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let block_id: String = std::env::var("MAILERLITE_BLOCK_ID")?;

    let etag: String = std::env::var("MAILERLITE_EMAIL_ETAG")?;

    let editor_content: Map<String, Value> =
        serde_json::from_str(&std::env::var("MAILERLITE_EDITOR_CONTENT_JSON")?)?;

    let response: Response<WithEtag<EmailBlock>> = mailerlite
        .emails()
        .update_block(email_id, block_id, etag, &editor_content)
        .await?;

    println!("New etag: {}", response.data.etag);

    Ok(())
}
