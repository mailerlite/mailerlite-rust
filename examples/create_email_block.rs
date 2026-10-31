//! Create email block. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{
    MailerLite, Response,
    models::emails::WithEtag,
    models::emails::{BlockType, EmailBlock, EmailDocument},
    resources::emails::{NewBlock, Placement},
};
use serde_json::{Map, Value, json};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let email: Response<WithEtag<EmailDocument>> = mailerlite.emails().get(&email_id).await?;

    let editor_content: Map<String, Value> = json!({
        "attrs": {"textAlign": "left"},
        "content": [{"type": "text", "text": "Hello"}]
    })
    .as_object()
    .cloned()
    .unwrap_or_default();

    let block: NewBlock = NewBlock {
        block_type: BlockType::Paragraph,
        editor_content,
        placement: Some(Placement::start()),
    };

    let response: Response<WithEtag<EmailBlock>> = mailerlite
        .emails()
        .create_block(&email_id, &email.data.etag, &block)
        .await?;

    println!("Block ID: {}", response.data.data.id);
    println!("New etag: {}", response.data.etag);

    Ok(())
}
