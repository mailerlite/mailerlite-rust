//! Update a product: `PUT /ecommerce/shops/{shop_id}/products/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_ecommerce_product
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateProduct};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let product = shop
        .products()
        .update(
            "Your Product ID",
            &CreateProduct::new("T-shirt").price(24.99),
        )
        .await?;

    println!("{product:#?}");

    Ok(())
}
