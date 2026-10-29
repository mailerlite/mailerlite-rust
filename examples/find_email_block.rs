use mailerlite_rs::{
    api_response::{ApiResponse, WithEtag},
    email::EmailBlock,
    MailerLite,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let email_id: String = String::from("Your Email ID");

    let block_id: String = String::from("Your Block ID");

    let response: ApiResponse<WithEtag<EmailBlock>> =
        mailerlite.email().find_block(email_id, block_id).await?;

    println!("{:#?}", response.data);

    Ok(())
}
