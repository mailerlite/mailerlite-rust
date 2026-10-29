use mailerlite_rs::{
    api_response::{ApiResponse, WithEtag},
    email::EmailDocument,
    error::Error,
    MailerLite,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let email_id: String = String::from("Your Email ID");

    let response: ApiResponse<WithEtag<EmailDocument>> = mailerlite.email().find(email_id).await?;

    println!("{:#?}", response.data);
    println!("Etag: {}", response.etag);

    Ok(())
}
