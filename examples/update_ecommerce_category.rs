//! Update a category: `PUT /ecommerce/shops/{shop_id}/categories/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_ecommerce_category
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateCategory};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let category = shop
        .categories()
        .update("Your Category ID", &CreateCategory::new("Shirts"))
        .await?;

    println!("{category:#?}");

    Ok(())
}
