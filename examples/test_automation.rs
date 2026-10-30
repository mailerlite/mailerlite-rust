//! Test automation.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    client
        .automations()
        .test("Your Automation ID", "ann@example.com")
        .await?;
    println!("Request accepted");
    Ok(())
}
