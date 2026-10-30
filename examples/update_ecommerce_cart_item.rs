//! Update a cart item: `PUT /ecommerce/shops/{shop_id}/carts/{cart_id}/items/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_ecommerce_cart_item
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateCartItem};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    // Send the full item: omitted quantity and variant are reset by the API.
    let item = shop
        .cart_items("Your Cart ID")
        .update(
            "Your Cart Item ID",
            &CreateCartItem::new("Your Product ID")
                .quantity(3)
                .price(19.99),
        )
        .await?;

    println!("{item:#?}");

    Ok(())
}
