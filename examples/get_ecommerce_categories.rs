//! List a shop's categories: `GET /ecommerce/shops/{shop_id}/categories`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_ecommerce_categories
//! ```

use mailerlite_rs::{MailerLite, ecommerce::ListCategories};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let page = shop.categories().list(&ListCategories::new()).await?;

    println!("{page:#?}");

    Ok(())
}
