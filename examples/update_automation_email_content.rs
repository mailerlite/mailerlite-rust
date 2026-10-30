//! Update automation email content.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automations()
        .update_email_content(
            "Your Automation ID",
            "Your Step or Email ID",
            "<p>Hello</p>",
        )
        .await?;
    println!("{response:#?}");
    Ok(())
}
