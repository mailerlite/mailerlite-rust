//! List a product's variants: `GET /ecommerce/shops/{shop_id}/products/{id}/variants`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_ecommerce_product_variants
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let variants = shop.variants("Your Product ID").list().await?;

    println!("{variants:#?}");

    Ok(())
}
