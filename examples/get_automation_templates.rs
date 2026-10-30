//! Read one page of automation templates.
//!
//! Requires `MAILERLITE_API_TOKEN`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let response = mailerlite
        .automation_templates()
        .list()
        .kind("welcome")
        .limit(10)
        .send()
        .await?;
    println!("HTTP {}", response.metadata.status);
    Ok(())
}
