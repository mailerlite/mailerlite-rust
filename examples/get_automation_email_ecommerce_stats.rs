//! Get e-commerce stats of one automation email: `GET /automations/{id}/emails/{email_id}/ecommerce-overview-stats`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_automation_email_ecommerce_stats
//! ```

use mailerlite_rs::{MailerLite, automations::AutomationEcommerceStatsQuery};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let stats = client
        .automations()
        .email_ecommerce_stats(
            "Your Automation ID",
            "Your Email ID",
            &AutomationEcommerceStatsQuery::new(),
        )
        .await?;

    println!("{stats:#?}");

    Ok(())
}
