use mailerlite_rs::{
    api_response::ApiResponse,
    email::{EmailVersion, ListVersionsOptions},
    pagination::Paginated,
    MailerLite,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let email_id: String = String::from("Your Email ID");

    let options: ListVersionsOptions = ListVersionsOptions {
        page: Some(1),
        limit: Some(10),
    };

    let response: ApiResponse<Paginated<EmailVersion>> =
        mailerlite.email().versions(email_id, options).await?;

    for version in &response.data {
        println!("{} {} {}", version.id, version.created_at, version.current);
    }

    Ok(())
}
