//! List a category's products: `GET /ecommerce/shops/{shop_id}/categories/{id}/products`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_ecommerce_category_products
//! ```

use mailerlite_rs::{MailerLite, ecommerce::ListCategoryProducts};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let page = shop
        .categories()
        .products("Your Category ID", &ListCategoryProducts::new())
        .await?;

    println!("{page:#?}");

    Ok(())
}
