//! List shops: `GET /ecommerce/shops`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_ecommerce_shops
//! ```

use mailerlite_rs::{MailerLite, ecommerce::ListShops};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client.ecommerce().shops().list(&ListShops::new()).await?;

    println!("{page:#?}");

    Ok(())
}
