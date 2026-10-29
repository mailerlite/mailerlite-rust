use mailerlite_rs::{
    api_response::{ApiResponse, WithEtag},
    email::{BlockType, EmailBlock, EmailDocument, NewBlock, Placement},
    MailerLite,
};
use serde_json::{json, Map, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let email_id: String = String::from("Your Email ID");

    let email: ApiResponse<WithEtag<EmailDocument>> = mailerlite.email().find(&email_id).await?;

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

    let response: ApiResponse<WithEtag<EmailBlock>> = mailerlite
        .email()
        .create_block(&email_id, &email.etag, &block)
        .await?;

    println!("Block ID: {}", response.data.id);
    println!("New etag: {}", response.etag);

    Ok(())
}
