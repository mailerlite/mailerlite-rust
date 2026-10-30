//! Count e-commerce shops without loading their records.
//!
//! Required environment: `MAILERLITE_API_TOKEN`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let response = mailerlite.ecommerce().count().await?;

    println!("{}", response.data.total);

    Ok(())
}
