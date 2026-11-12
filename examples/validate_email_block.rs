//! Validate email block.
//!
//! Requires `MAILERLITE_API_TOKEN`.
use mailerlite_rs::{
    MailerLite, Response,
    models::emails::{BlockType, EmailResponseMetadata, Validation},
    resources::emails::NewBlock,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let content = serde_json::json!({
        "attrs": {"url": "https://example.com"},
        "content": [{"type": "text", "text": "Buy now"}]
    });
    let block = NewBlock::new(BlockType::Button, content);

    let response: Response<Validation, EmailResponseMetadata> =
        mailerlite.emails().validate_block(&block).await?;

    if !response.data.valid {
        println!("{:#?}", response.data.errors);
    } else if let Some(rejected) = &response.data.rejected {
        println!("Rejected keys: {:?}", rejected.keys);
    }

    Ok(())
}
