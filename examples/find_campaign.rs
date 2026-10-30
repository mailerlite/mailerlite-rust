//! Get a campaign: `GET /campaigns/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example find_campaign
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let campaign = client.campaigns().get("Your Campaign ID").await?;

    println!("{campaign:#?}");

    Ok(())
}
