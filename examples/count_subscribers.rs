//! Count subscribers: `GET /subscribers?limit=0`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example count_subscribers
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let total = client.subscribers().count().await?;

    println!("{total} subscribers");

    Ok(())
}
