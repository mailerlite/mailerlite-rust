//! Update a shop: `PUT /ecommerce/shops/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_ecommerce_shop
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateShop};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client
        .ecommerce()
        .shops()
        .update(
            "Your Shop ID",
            &CreateShop::new("Renamed shop", "https://shop.example.com", "EUR"),
        )
        .await?;

    println!("{shop:#?}");

    Ok(())
}
