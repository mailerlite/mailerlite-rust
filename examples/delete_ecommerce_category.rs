//! Delete a category: `DELETE /ecommerce/shops/{shop_id}/categories/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_ecommerce_category
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let category = shop.categories().delete("Your Category ID").await?;

    println!("deleted {category:#?}");

    Ok(())
}
