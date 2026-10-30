//! Copy automation.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client.automations().copy("Your Automation ID").await?;
    println!("{response:#?}");
    Ok(())
}
