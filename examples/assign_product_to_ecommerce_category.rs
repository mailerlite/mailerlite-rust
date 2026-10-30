//! Add a product to a category: `PUT /ecommerce/shops/{shop_id}/categories/{id}/products/{product_id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example assign_product_to_ecommerce_category
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let product = shop
        .categories()
        .assign_product("Your Category ID", "Your Product ID")
        .await?;

    println!("{product:#?}");

    Ok(())
}
