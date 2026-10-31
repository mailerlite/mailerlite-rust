//! List the first page of embedded forms.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`.
use mailerlite_rs::{MailerLite, resources::forms::FormType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let response = mailerlite
        .forms()
        .list(FormType::Embedded)
        .limit(25)
        .send()
        .await?;

    for form in response.data.data {
        println!("{}: {}", form.id, form.name);
    }

    Ok(())
}
