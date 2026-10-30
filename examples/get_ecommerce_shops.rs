//! List one page of e-commerce shops.
//!
//! Required environment: `MAILERLITE_API_TOKEN`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let response = mailerlite.ecommerce().list().limit(25).send().await?;
    for item in response.data.data {
        println!("{}", item.id);
    }
    Ok(())
}
