//! Get a product: `GET /ecommerce/shops/{shop_id}/products/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example find_ecommerce_product
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let product = shop.products().get("Your Product ID").await?;

    println!("{product:#?}");

    Ok(())
}
