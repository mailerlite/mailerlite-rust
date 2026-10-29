use mailerlite_rs::{
    api_response::ApiResponse,
    email::{BlockType, NewBlock, Validation},
    MailerLite,
};
use serde_json::{json, Map, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

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

    let response: ApiResponse<Validation> = mailerlite.email().validate_block(&block).await?;

    if !response.valid {
        println!("{:#?}", response.errors);
    }

    Ok(())
}
