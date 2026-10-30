//! List a shop's products: `GET /ecommerce/shops/{shop_id}/products`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_ecommerce_products
//! ```

use mailerlite_rs::{MailerLite, ecommerce::ListProducts};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let page = shop.products().list(&ListProducts::new()).await?;

    println!("{page:#?}");

    Ok(())
}
