//! Get email versions. Requires MAILERLITE_API_KEY.
use mailerlite_rs::{
    MailerLite, Response, models::emails::EmailVersion, pagination::NumberedPage,
    resources::emails::ListVersionsOptions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_KEY")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let email_id: String = std::env::var("MAILERLITE_EMAIL_ID")?;

    let options: ListVersionsOptions = ListVersionsOptions {
        page: Some(1),
        limit: Some(10),
    };

    let response: Response<NumberedPage<EmailVersion>> =
        mailerlite.emails().versions(email_id, options).await?;

    for version in &response.data.data {
        println!("{} {} {}", version.id, version.created_at, version.current);
    }

    Ok(())
}
