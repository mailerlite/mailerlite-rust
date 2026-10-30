//! Update a cart: `PUT /ecommerce/shops/{shop_id}/carts/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_ecommerce_cart
//! ```

use mailerlite_rs::{MailerLite, ecommerce::UpdateCart};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let cart = shop
        .carts()
        .update(
            "Your Cart ID",
            &UpdateCart::new()
                .checkout_url("https://shop.example.com/checkout")
                .cart_total(39.98),
        )
        .await?;

    println!("{cart:#?}");

    Ok(())
}
