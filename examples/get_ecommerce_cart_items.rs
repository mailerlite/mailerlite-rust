//! List a cart's items: `GET /ecommerce/shops/{shop_id}/carts/{cart_id}/items`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_ecommerce_cart_items
//! ```

use mailerlite_rs::{MailerLite, ecommerce::ListCartItems};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let page = shop
        .cart_items("Your Cart ID")
        .list(&ListCartItems::new())
        .await?;

    println!("{page:#?}");

    Ok(())
}
