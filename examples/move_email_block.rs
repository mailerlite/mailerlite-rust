use mailerlite_rs::{
    api_response::{ApiResponse, WithEtag},
    email::{EmailBlock, Placement},
    MailerLite,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let email_id: String = String::from("Your Email ID");

    let block_id: String = String::from("Your Block ID");

    let etag: String = String::from("The etag from the last response");

    let placement: Placement = Placement::after("The ID of the block before the new place");

    let response: ApiResponse<WithEtag<EmailBlock>> = mailerlite
        .email()
        .move_block(email_id, block_id, etag, &placement)
        .await?;

    println!("New etag: {}", response.etag);

    Ok(())
}
