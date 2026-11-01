//! Move email block.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_BLOCK_ID`,
//! `MAILERLITE_EMAIL_ETAG`, `MAILERLITE_ADJACENT_BLOCK_ID`.
//! Moves the selected block after the supplied adjacent block at the same nesting level.
use mailerlite_rs::models::emails::EmailResponse;
use mailerlite_rs::{
    MailerLite, models::emails::EmailBlock, models::emails::WithEtag, resources::emails::Placement,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let block_id: String = std::env::var("MAILERLITE_BLOCK_ID")?;

    let etag: String = std::env::var("MAILERLITE_EMAIL_ETAG")?;

    let placement: Placement = Placement::after(std::env::var("MAILERLITE_ADJACENT_BLOCK_ID")?);

    let response: EmailResponse<WithEtag<EmailBlock>> = mailerlite
        .emails()
        .move_block(email_id, block_id, etag, &placement)
        .await?;

    println!("New etag: {}", response.data.etag);

    Ok(())
}
