//! Get a campaign's e-commerce totals: `GET /campaigns/{id}/reports/ecommerce`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_campaign_ecommerce_stats
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let report = client
        .campaigns()
        .ecommerce_stats("Your Campaign ID", "Your Shop ID")
        .await?;

    println!("{report:#?}");

    Ok(())
}
