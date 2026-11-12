//! Update email block.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_BLOCK_ID`,
//! `MAILERLITE_EMAIL_ETAG`, `MAILERLITE_EDITOR_CONTENT_JSON`.
//! Content JSON is the document itself, for example
//! `{"content":[{"type":"text","text":"Hello"}]}`.
//! Sends one update by block ID; the server validates against the stored block kind.
//! The supplied etag controls optimistic concurrency.
//! Replaces the complete block document; omitted content keys are removed.
use mailerlite_rs::{
    MailerLite, Response,
    models::emails::{BlockWriteResult, EmailResponseMetadata},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let block_id: String = std::env::var("MAILERLITE_BLOCK_ID")?;

    let etag: String = std::env::var("MAILERLITE_EMAIL_ETAG")?;

    let editor_content: serde_json::Value =
        serde_json::from_str(&std::env::var("MAILERLITE_EDITOR_CONTENT_JSON")?)?;

    let response: Response<BlockWriteResult, EmailResponseMetadata> = mailerlite
        .emails()
        .update_block(email_id, block_id, etag, &editor_content)
        .await?;

    println!("New etag: {}", response.data.etag);

    if let Some(rejected) = &response.data.rejected {
        println!("Rejected keys: {:?}", rejected.keys);
    }

    Ok(())
}
