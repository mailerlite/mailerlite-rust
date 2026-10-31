//! Validate email block. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{
    MailerLite, Response,
    models::emails::{BlockType, Validation},
    resources::emails::NewBlock,
};
use serde_json::{Map, Value, json};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let editor_content: Map<String, Value> = json!({
        "attrs": {"href": "https://example.com"},
        "content": [{"type": "text", "text": "Buy now"}]
    })
    .as_object()
    .cloned()
    .unwrap_or_default();

    let block: NewBlock = NewBlock {
        block_type: BlockType::Button,
        editor_content,
        placement: None,
    };

    let response: Response<Validation> = mailerlite.emails().validate_block(&block).await?;

    if !response.data.valid {
        println!("{:#?}", response.data.errors);
    }

    Ok(())
}
