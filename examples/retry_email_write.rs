//! Retry email write.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_EMAIL_ETAG`,
//! `MAILERLITE_IDEMPOTENCY_KEY`.
//! Reuses the same key, etag, and payload for one retry. The API retains keys for
//! 24 hours; after that, reconcile the original write before retrying it.
use mailerlite_rs::{
    Error, MailerLite, Response,
    models::emails::{BlockType, EmailBlock, EmailResponseMetadata, WithEtag},
    resources::emails::{Emails, NewBlock},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let etag: String = std::env::var("MAILERLITE_EMAIL_ETAG")?;

    // Persist one key per logical write, for example a UUID. Keep it across retries,
    // including later process runs if the outcome is still uncertain.
    let idempotency_key: String = std::env::var("MAILERLITE_IDEMPOTENCY_KEY")?;

    let block: NewBlock = NewBlock::new(BlockType::Divider, serde_json::json!({}));

    let email: Emails = mailerlite.emails().with_idempotency_key(idempotency_key);

    let response = create_block_with_retry(&email, &email_id, &etag, &block).await?;

    println!(
        "Block ID: {}, replay: {}",
        response.data.data.id,
        response.metadata.is_idempotent_replay()
    );

    Ok(())
}

// A bounded retry cannot establish success if the server keeps returning an unreadable
// response. Keep the original key and payload for reconciliation or a later retry.
pub(crate) async fn create_block_with_retry(
    email: &Emails<'_>,
    email_id: &str,
    etag: &str,
    block: &NewBlock,
) -> mailerlite_rs::Result<Response<WithEtag<EmailBlock>, EmailResponseMetadata>> {
    match email.create_block(email_id, etag, block).await {
        // The server may have committed the write before the failure. Retry once
        // with the same key, etag, and body, even if its response was unreadable.
        Err(Error::Transport(_) | Error::Timeout | Error::Decode(_)) => {
            email.create_block(email_id, etag, block).await
        }
        result => result,
    }
}
