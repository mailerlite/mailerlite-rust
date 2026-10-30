//! List timezone IDs available for campaign schedules.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let response = mailerlite.reference().timezones().await?;

    for timezone in response.data {
        println!("{}: {}", timezone.id, timezone.name_for_humans);
    }

    Ok(())
}
