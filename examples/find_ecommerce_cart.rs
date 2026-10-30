//! Get a cart: `GET /ecommerce/shops/{shop_id}/carts/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example find_ecommerce_cart
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let cart = shop.carts().get("Your Cart ID").await?;

    println!("{cart:#?}");

    Ok(())
}
