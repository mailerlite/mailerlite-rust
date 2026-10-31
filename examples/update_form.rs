//! Rename an existing form.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`, `MAILERLITE_FORM_ID`, `MAILERLITE_FORM_NAME`.
use mailerlite_rs::{MailerLite, resources::forms::UpdateForm};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_FORM_ID")?;
    let name = std::env::var("MAILERLITE_FORM_NAME")?;

    let response = mailerlite.forms().update(id, UpdateForm::new(name)).await?;

    println!("Updated form {}: {}", response.data.id, response.data.name);

    Ok(())
}
