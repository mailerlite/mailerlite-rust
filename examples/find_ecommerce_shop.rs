//! Get a shop: `GET /ecommerce/shops/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example find_ecommerce_shop
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shops().get("Your Shop ID").await?;

    println!("{shop:#?}");

    Ok(())
}
