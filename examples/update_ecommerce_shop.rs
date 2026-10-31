//! Update an existing e-commerce shop.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_SHOP_URL`.
//! Also required: `MAILERLITE_SHOP_NAME`, `MAILERLITE_SHOP_CURRENCY`.
//! Replaces the shop's name, URL, and currency with these supplied values.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::SaveShop;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let shop_url = std::env::var("MAILERLITE_SHOP_URL")?;
    let shop_name = std::env::var("MAILERLITE_SHOP_NAME")?;
    let shop_currency = std::env::var("MAILERLITE_SHOP_CURRENCY")?;

    let params = SaveShop::new(shop_name, shop_url, shop_currency);

    let response = mailerlite.ecommerce().update(shop_id, params).await?;

    println!("{}", response.data.id);

    Ok(())
}
