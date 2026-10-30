//! Get the account's e-commerce settings: `GET /ecommerce/settings`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example find_ecommerce_settings
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let settings = client.ecommerce().settings().get().await?;

    println!("{settings:#?}");

    Ok(())
}
