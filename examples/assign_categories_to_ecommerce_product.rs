//! Add a product to categories: `PUT /ecommerce/shops/{shop_id}/products/{id}/categories/multiple`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example assign_categories_to_ecommerce_product
//! ```

use mailerlite_rs::{MailerLite, ecommerce::AssignCategories};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let product = shop
        .products()
        .assign_categories(
            "Your Product ID",
            &AssignCategories::new(["Your Category ID"]),
        )
        .await?;

    println!("{product:#?}");

    Ok(())
}
