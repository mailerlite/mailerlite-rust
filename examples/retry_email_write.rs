//! Retry email write. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{
    Error, MailerLite, Response,
    models::emails::WithEtag,
    models::emails::{BlockType, EmailBlock},
    resources::emails::{Emails, NewBlock},
};
use serde_json::Map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let etag: String = std::env::var("MAILERLITE_EMAIL_ETAG")?;

    // Use a new key for each write, for example a UUID.
    let idempotency_key: String = std::env::var("MAILERLITE_IDEMPOTENCY_KEY")?;

    let block: NewBlock = NewBlock {
        block_type: BlockType::Divider,
        editor_content: Map::new(),
        placement: None,
    };

    let email: Emails = mailerlite.emails().with_idempotency_key(idempotency_key);

    let response: Response<WithEtag<EmailBlock>> =
        match email.create_block(&email_id, &etag, &block).await {
            // A network error: send the same write again with the same key.
            Err(Error::Transport(_)) => email.create_block(&email_id, &etag, &block).await?,
            result => result?,
        };

    println!(
        "Block ID: {}, replay: {}",
        response.data.data.id,
        response.metadata.is_idempotent_replay()
    );

    Ok(())
}
