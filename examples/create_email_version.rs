use mailerlite_rs::{
    api_response::{ApiResponse, DataOnly},
    email::EmailVersion,
    MailerLite,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let email_id: String = String::from("Your Email ID");

    let response: ApiResponse<DataOnly<EmailVersion>> =
        mailerlite.email().create_version(email_id).await?;

    let is_new: bool = response.status_code == 201;

    println!("Version ID: {}, new: {}", response.data.id, is_new);

    Ok(())
}
