//! Delete a variant: `DELETE /ecommerce/shops/{shop_id}/products/{id}/variants/{variant_id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_ecommerce_product_variant
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let product = shop
        .variants("Your Product ID")
        .delete("Your Variant ID")
        .await?;

    println!("{product:#?}");

    Ok(())
}
