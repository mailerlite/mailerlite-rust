//! Update an existing e-commerce cart item.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_CART_ID`, `MAILERLITE_CART_ITEM_ID`, `MAILERLITE_PRODUCT_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::SaveCartItem;
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let cart_id = std::env::var("MAILERLITE_CART_ID")?;
    let cart_item_id = std::env::var("MAILERLITE_CART_ITEM_ID")?;
    let product_id = std::env::var("MAILERLITE_PRODUCT_ID")?;

    let params = SaveCartItem::new(product_id)
        .quantity(1)
        .price(Decimal::new(1999, 2));

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .carts()
        .items(cart_id)
        .update(cart_item_id, params)
        .await?;

    println!("{}", response.data.id);

    Ok(())
}
