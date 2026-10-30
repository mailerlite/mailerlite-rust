//! List a shop's carts: `GET /ecommerce/shops/{shop_id}/carts`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_ecommerce_carts
//! ```

use mailerlite_rs::{MailerLite, ecommerce::ListCarts};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let page = shop.carts().list(&ListCarts::new()).await?;

    println!("{page:#?}");

    Ok(())
}
