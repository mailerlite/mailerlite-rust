//! Get a cart item: `GET /ecommerce/shops/{shop_id}/carts/{cart_id}/items/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example find_ecommerce_cart_item
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let item = shop
        .cart_items("Your Cart ID")
        .get("Your Cart Item ID")
        .await?;

    println!("{item:#?}");

    Ok(())
}
