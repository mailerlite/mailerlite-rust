use mailerlite_rs::{
    api_response::{ApiResponse, WithEtag},
    email::EmailBlock,
    MailerLite,
};
use serde_json::{json, Map, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let email_id: String = String::from("Your Email ID");

    let block_id: String = String::from("Your Block ID");

    let etag: String = String::from("The etag from the last response");

    let editor_content: Map<String, Value> = json!({
        "attrs": {"textAlign": "center"},
        "content": [{"type": "text", "text": "Hello again"}]
    })
    .as_object()
    .cloned()
    .unwrap_or_default();

    let response: ApiResponse<WithEtag<EmailBlock>> = mailerlite
        .email()
        .update_block(email_id, block_id, etag, &editor_content)
        .await?;

    println!("New etag: {}", response.etag);

    Ok(())
}
