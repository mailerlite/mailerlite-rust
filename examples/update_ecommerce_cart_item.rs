//! Update an existing e-commerce cart item.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_CART_ID`, `MAILERLITE_CART_ITEM_ID`, `MAILERLITE_PRODUCT_ID`.
//! Also required: `MAILERLITE_CART_ITEM_QUANTITY` (unsigned integer), `MAILERLITE_CART_ITEM_PRICE` (decimal text).
//! Replaces the item's product, quantity, and price with the supplied values.
use mailerlite_rs::resources::ecommerce::params::SaveCartItem;
use mailerlite_rs::{Decimal, MailerLite};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let cart_id = std::env::var("MAILERLITE_CART_ID")?;
    let cart_item_id = std::env::var("MAILERLITE_CART_ITEM_ID")?;
    let product_id = std::env::var("MAILERLITE_PRODUCT_ID")?;
    let quantity: u64 = std::env::var("MAILERLITE_CART_ITEM_QUANTITY")?.parse()?;
    let price: Decimal = std::env::var("MAILERLITE_CART_ITEM_PRICE")?.parse()?;

    let params = SaveCartItem::new(product_id)
        .quantity(quantity)
        .price(price);

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
