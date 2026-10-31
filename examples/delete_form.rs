//! Delete an existing form.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`, `MAILERLITE_FORM_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_FORM_ID")?;

    let response = mailerlite.forms().delete(&id).await?;

    println!("Deleted form {id}: {}", response.metadata.status);

    Ok(())
}
