//! Get a subscriber's orders: `GET /subscribers/{id}/reports/ecommerce/orders`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_subscriber_ecommerce_orders
//! ```

use mailerlite_rs::{MailerLite, reports::ListReportOrders};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let orders = client
        .subscribers()
        .ecommerce_orders("Your Subscriber ID", &ListReportOrders::new("Your Shop ID"))
        .await?;

    println!("{orders:#?}");

    Ok(())
}
