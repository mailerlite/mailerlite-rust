//! Find email block.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_BLOCK_ID`.
use mailerlite_rs::{
    MailerLite, Response,
    models::emails::{EmailBlock, EmailResponseMetadata, WithEtag},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let block_id: String = std::env::var("MAILERLITE_BLOCK_ID")?;

    let response: Response<WithEtag<EmailBlock>, EmailResponseMetadata> =
        mailerlite.emails().get_block(email_id, block_id).await?;

    println!("{:#?}", response.data.data);

    Ok(())
}
