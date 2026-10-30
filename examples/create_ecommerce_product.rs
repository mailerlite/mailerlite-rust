//! Create a product: `POST /ecommerce/shops/{shop_id}/products`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_ecommerce_product
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateProduct};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let product = shop
        .products()
        .create(
            &CreateProduct::new("T-shirt")
                .price(19.99)
                .url("https://shop.example.com/t-shirt"),
        )
        .await?;

    println!("{product:#?}");

    Ok(())
}
