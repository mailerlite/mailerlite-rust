//! Get a shop's sales stats: `GET /ecommerce/shops/{id}/reports/stats`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_ecommerce_shop_stats
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let stats = client
        .ecommerce()
        .shops()
        .stats("Your Shop ID", "2026-01-01", "2026-12-31")
        .await?;

    println!("{stats:#?}");

    Ok(())
}
