use mailerlite_rs::{
    api_response::{ApiResponse, WithEtag},
    email::EmailHtml,
    error::Error,
    MailerLite,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let email_id: String = String::from("Your Email ID");

    let response: ApiResponse<WithEtag<EmailHtml>> = mailerlite.email().html(email_id).await?;

    println!("{}", response.data.html);

    Ok(())
}
