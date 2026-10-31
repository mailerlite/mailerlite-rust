//! Create an e-commerce shop.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_URL`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::SaveShop;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_url = std::env::var("MAILERLITE_SHOP_URL")?;

    let params = SaveShop::new("Example store", shop_url, "USD");

    let response = mailerlite.ecommerce().create(params).await?;

    println!("{}", response.data.id);

    Ok(())
}
