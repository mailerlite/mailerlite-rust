//! Create email block.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`.
use mailerlite_rs::{
    MailerLite, Response,
    models::emails::{BlockType, BlockWriteResult, EmailDocument, EmailResponseMetadata, WithEtag},
    resources::emails::{NewBlock, Placement},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let email: Response<WithEtag<EmailDocument>, EmailResponseMetadata> =
        mailerlite.emails().get(&email_id).await?;

    let block = NewBlock::new(
        BlockType::Paragraph,
        serde_json::json!({"content": [{"type":"text", "text":"Hello"}]}),
    )
    .placement(Placement::start());

    let response: Response<BlockWriteResult, EmailResponseMetadata> = mailerlite
        .emails()
        .create_block(&email_id, &email.data.etag, &block)
        .await?;

    println!("Block ID: {}", response.data.data.id);
    println!("New etag: {}", response.data.etag);

    if let Some(rejected) = &response.data.rejected {
        println!("Rejected keys: {:?}", rejected.keys);
    }

    Ok(())
}
