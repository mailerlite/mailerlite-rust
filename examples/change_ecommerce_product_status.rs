//! Disable (or, with `enable`, re-enable) a product: `PUT /ecommerce/shops/{shop_id}/products/{id}/change-status`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example change_ecommerce_product_status
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let product = shop.products().disable("Your Product ID").await?;

    println!("{product:#?}");

    Ok(())
}
