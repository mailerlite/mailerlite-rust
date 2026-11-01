//! Retry email write.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_EMAIL_ETAG`,
//! `MAILERLITE_IDEMPOTENCY_KEY`.
//! Opts into bounded client retries with the same key, etag, and payload. The API retains keys for
//! 24 hours; after that, reconcile the original write before retrying it.
use mailerlite_rs::{
    Error, MailerLite, RetryPolicy,
    models::emails::BlockContent,
    resources::emails::{Emails, NewBlock},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::builder(api_key)
        .retry_policy(RetryPolicy::safe_reads().with_idempotent_writes())
        .build()?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let etag: String = std::env::var("MAILERLITE_EMAIL_ETAG")?;

    // Persist one key per logical write, for example a UUID. Keep it across retries,
    // including later process runs if the outcome is still uncertain.
    let idempotency_key: String = std::env::var("MAILERLITE_IDEMPOTENCY_KEY")?;

    let block: NewBlock = NewBlock::new(BlockContent::Divider(Default::default()));

    let email: Emails = mailerlite.emails().with_idempotency_key(idempotency_key);

    // The client retries transient failures within one deadline. Any final error
    // can still leave the write's outcome uncertain; retain the original request.
    let response = match email.create_block(&email_id, &etag, &block).await {
        Ok(response) => response,
        Err(Error::Api(error)) if error.is_idempotency_in_progress() => {
            eprintln!(
                "The original write is still running. Wait, then retry with the same key, body, and etag."
            );
            return Err(Error::Api(error).into());
        }
        Err(Error::Api(error)) if error.is_version_conflict() => {
            eprintln!(
                "The email version changed. Reconcile the original write before preparing another write."
            );
            return Err(Error::Api(error).into());
        }
        Err(error) => return Err(error.into()),
    };

    println!(
        "Block ID: {}, replay: {}",
        response.data.data.id,
        response.metadata.is_idempotent_replay()
    );

    Ok(())
}
