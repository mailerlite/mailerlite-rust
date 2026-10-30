//! Bulk import categories: `POST /ecommerce/shops/{shop_id}/categories/import`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example import_ecommerce_categories
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateCategory};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let categories = shop
        .imports()
        .categories(&[CreateCategory::new("T-shirts"), CreateCategory::new("Mugs")])
        .await?;

    println!("{categories:#?}");

    Ok(())
}
