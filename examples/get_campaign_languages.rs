//! List languages available for campaign unsubscribe templates.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let response = mailerlite.reference().campaign_languages().await?;
    for language in response.data {
        println!("{}: {} ({})", language.id, language.name, language.iso639);
    }
    Ok(())
}
