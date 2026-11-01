//! Update email block.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_BLOCK_ID`,
//! `MAILERLITE_EMAIL_ETAG`, `MAILERLITE_EDITOR_CONTENT_JSON`.
//! Content JSON includes its type, for example
//! `{"type":"paragraph","editor_content":{"content":[{"type":"text","text":"Hello"}]}}`.
//! Reads the existing block and checks that the replacement has the same type.
//! The supplied etag still controls optimistic concurrency.
//! Replaces the complete block document; omitted content keys are removed.
use mailerlite_rs::{
    MailerLite, Response,
    models::emails::{BlockContent, EmailBlock, EmailResponseMetadata, WithEtag},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let block_id: String = std::env::var("MAILERLITE_BLOCK_ID")?;

    let etag: String = std::env::var("MAILERLITE_EMAIL_ETAG")?;

    let editor_content: BlockContent =
        serde_json::from_str(&std::env::var("MAILERLITE_EDITOR_CONTENT_JSON")?)?;

    let existing = mailerlite.emails().get_block(&email_id, &block_id).await?;

    let response: Response<WithEtag<EmailBlock>, EmailResponseMetadata> = mailerlite
        .emails()
        .update_block(email_id, &existing.data.data, etag, &editor_content)
        .await?;

    println!("New etag: {}", response.data.etag);

    Ok(())
}
