//! Bulk import products: `POST /ecommerce/shops/{shop_id}/products/import`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example import_ecommerce_products
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateProduct};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let products = shop
        .imports()
        .products(&[
            CreateProduct::new("T-shirt").price(19.99),
            CreateProduct::new("Mug").price(9.99),
        ])
        .await?;

    println!("{products:#?}");

    Ok(())
}
