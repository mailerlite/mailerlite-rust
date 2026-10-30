//! Delete an e-commerce cart item and display the returned ID.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_CART_ID`, `MAILERLITE_CART_ITEM_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let cart_id = std::env::var("MAILERLITE_CART_ID")?;
    let cart_item_id = std::env::var("MAILERLITE_CART_ITEM_ID")?;

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .carts()
        .items(cart_id)
        .delete(cart_item_id)
        .await?;
    println!("{}", response.data.id);
    Ok(())
}
