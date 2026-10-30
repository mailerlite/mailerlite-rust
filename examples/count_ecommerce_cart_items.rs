//! Count e-commerce cart items without loading their records.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_CART_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let cart_id = std::env::var("MAILERLITE_CART_ID")?;

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .carts()
        .items(cart_id)
        .count()
        .await?;
    println!("{}", response.data.total);
    Ok(())
}
