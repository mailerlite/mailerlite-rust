//! Create a category: `POST /ecommerce/shops/{shop_id}/categories`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_ecommerce_category
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateCategory};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let category = shop
        .categories()
        .create(&CreateCategory::new("T-shirts"))
        .await?;

    println!("{category:#?}");

    Ok(())
}
