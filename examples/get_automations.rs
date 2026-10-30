//! List the first page of enabled automations.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let response = mailerlite
        .automations()
        .list()
        .enabled(true)
        .limit(10)
        .send()
        .await?;
    for automation in response.data.data {
        println!("{}: {}", automation.id, automation.name);
    }
    Ok(())
}
