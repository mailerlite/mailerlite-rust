//! Find email block. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{MailerLite, Response, models::emails::EmailBlock, models::emails::WithEtag};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let block_id: String = std::env::var("MAILERLITE_BLOCK_ID")?;

    let response: Response<WithEtag<EmailBlock>> =
        mailerlite.emails().get_block(email_id, block_id).await?;

    println!("{:#?}", response.data.data);

    Ok(())
}
