//! List orders attributed to a campaign: `GET /campaigns/{id}/reports/ecommerce/orders`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_campaign_ecommerce_orders
//! ```

use mailerlite_rs::{MailerLite, reports::ListReportOrders};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let orders = client
        .campaigns()
        .ecommerce_orders("Your Campaign ID", &ListReportOrders::new("Your Shop ID"))
        .await?;

    println!("{orders:#?}");

    Ok(())
}
