//! Read e-commerce tracking and attribution settings.
//!
//! Required environment: `MAILERLITE_API_TOKEN`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let response = mailerlite.ecommerce().settings().await?;

    println!(
        "Attribution window: {} days",
        response.data.ecommerce_attribution_window
    );

    Ok(())
}
