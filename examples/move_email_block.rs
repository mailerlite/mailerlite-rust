//! Move email block. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{
    MailerLite, Response, models::emails::EmailBlock, models::emails::WithEtag,
    resources::emails::Placement,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let block_id: String = std::env::var("MAILERLITE_BLOCK_ID")?;

    let etag: String = std::env::var("MAILERLITE_EMAIL_ETAG")?;

    let placement: Placement = Placement::after(std::env::var("MAILERLITE_ADJACENT_BLOCK_ID")?);

    let response: Response<WithEtag<EmailBlock>> = mailerlite
        .emails()
        .move_block(email_id, block_id, etag, &placement)
        .await?;

    println!("New etag: {}", response.data.etag);

    Ok(())
}
