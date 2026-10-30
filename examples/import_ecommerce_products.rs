//! Import e-commerce products using a typed array payload.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::{ImportProduct, SaveProduct};
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;

    let entry =
        ImportProduct::new(SaveProduct::new("Example product").price(Decimal::new(1999, 2)));
    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .products()
        .import(vec![entry])
        .await?;
    println!("Returned {} entries", response.data.len());
    Ok(())
}
