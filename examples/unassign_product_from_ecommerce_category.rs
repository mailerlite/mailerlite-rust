//! Remove a product from a category: `DELETE /ecommerce/shops/{shop_id}/categories/{id}/products/{product_id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example unassign_product_from_ecommerce_category
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let product = shop
        .categories()
        .unassign_product("Your Category ID", "Your Product ID")
        .await?;

    println!("{product:#?}");

    Ok(())
}
