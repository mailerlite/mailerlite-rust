//! Get an automation's e-commerce totals: `GET /automations/{id}/monthly-totals/ecommerce`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_automation_ecommerce_stats
//! ```

use mailerlite_rs::{MailerLite, automations::AutomationEcommerceStatsQuery};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let stats = client
        .automations()
        .ecommerce_stats(
            "Your Automation ID",
            &AutomationEcommerceStatsQuery::new().period("2026-01-01", "2026-12-31"),
        )
        .await?;

    println!("{stats:#?}");

    Ok(())
}
