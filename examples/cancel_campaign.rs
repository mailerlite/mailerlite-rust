//! Cancel a scheduled campaign: `POST /campaigns/{id}/cancel`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example cancel_campaign
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let campaign = client.campaigns().cancel("Your Campaign ID").await?;

    println!("{campaign:#?}");

    Ok(())
}
