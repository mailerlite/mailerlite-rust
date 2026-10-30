//! Add an item to a cart: `POST /ecommerce/shops/{shop_id}/carts/{cart_id}/items`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_ecommerce_cart_item
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateCartItem};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let item = shop
        .cart_items("Your Cart ID")
        .create(
            &CreateCartItem::new("Your Product ID")
                .quantity(1)
                .price(19.99),
        )
        .await?;

    println!("{item:#?}");

    Ok(())
}
