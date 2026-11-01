//! Create email block.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_EMAIL_ID`.
use mailerlite_rs::models::emails::EmailResponse;
use mailerlite_rs::{
    MailerLite,
    models::emails::WithEtag,
    models::emails::{BlockType, EmailBlock, EmailDocument},
    resources::emails::{NewBlock, Placement},
};
use serde_json::{Map, Value, json};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let email: EmailResponse<WithEtag<EmailDocument>> = mailerlite.emails().get(&email_id).await?;

    let editor_content: Map<String, Value> = json!({
        "attrs": {"textAlign": "left"},
        "content": [{"type": "text", "text": "Hello"}]
    })
    .as_object()
    .cloned()
    .unwrap_or_default();

    let block: NewBlock =
        NewBlock::new(BlockType::Paragraph, editor_content).placement(Placement::start());

    let response: EmailResponse<WithEtag<EmailBlock>> = mailerlite
        .emails()
        .create_block(&email_id, &email.data.etag, &block)
        .await?;

    println!("Block ID: {}", response.data.data.id);
    println!("New etag: {}", response.data.etag);

    Ok(())
}
