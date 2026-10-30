//! Create an e-commerce product.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::SaveProduct;
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;

    let params = SaveProduct::new("Example product").price(Decimal::new(1999, 2));
    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .products()
        .create(params)
        .await?;
    println!("{}", response.data.id);
    Ok(())
}
