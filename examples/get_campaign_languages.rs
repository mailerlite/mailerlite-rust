//! List campaign languages: `GET /campaigns/languages`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_campaign_languages
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let languages = client.campaigns().languages().await?;

    println!("{languages:#?}");

    Ok(())
}
