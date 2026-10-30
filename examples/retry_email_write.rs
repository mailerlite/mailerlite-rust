use mailerlite_rs::{
    api_response::{ApiResponse, WithEtag},
    email::{BlockType, Email, EmailBlock, NewBlock},
    error::Error,
    MailerLite,
};
use serde_json::Map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let email_id: String = String::from("Your Email ID");

    let etag: String = String::from("The etag from the last response");

    // Use a new key for each write, for example a UUID.
    let idempotency_key: String = String::from("A new key for this write");

    let block: NewBlock = NewBlock {
        block_type: BlockType::Divider,
        editor_content: Map::new(),
        placement: None,
    };

    let email: Email = mailerlite.email().with_idempotency_key(idempotency_key);

    let response: ApiResponse<WithEtag<EmailBlock>> =
        match email.create_block(&email_id, &etag, &block).await {
            // A network error: send the same write again with the same key.
            Err(Error::Transport(_)) => email.create_block(&email_id, &etag, &block).await?,
            result => result?,
        };

    println!(
        "Block ID: {}, replay: {}",
        response.data.id,
        response.is_idempotent_replay()
    );

    Ok(())
}
