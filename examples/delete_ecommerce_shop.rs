//! Delete a shop: `DELETE /ecommerce/shops/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_ecommerce_shop
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shops().delete("Your Shop ID").await?;

    println!("deleted {shop:#?}");

    Ok(())
}
