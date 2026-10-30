//! Update an existing e-commerce cart.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_CART_ID`, `MAILERLITE_CHECKOUT_URL`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::UpdateCart;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let cart_id = std::env::var("MAILERLITE_CART_ID")?;
    let checkout_url = std::env::var("MAILERLITE_CHECKOUT_URL")?;

    let params = UpdateCart::new().checkout_url(checkout_url);
    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .carts()
        .update(cart_id, params)
        .await?;
    println!("{}", response.data.id);
    Ok(())
}
