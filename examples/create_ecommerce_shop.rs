//! Create a shop: `POST /ecommerce/shops`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_ecommerce_shop
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateShop};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client
        .ecommerce()
        .shops()
        .create(&CreateShop::new(
            "My shop",
            "https://shop.example.com",
            "EUR",
        ))
        .await?;

    println!("{shop:#?}");

    Ok(())
}
