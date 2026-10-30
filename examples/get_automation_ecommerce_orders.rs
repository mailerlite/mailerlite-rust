//! List orders attributed to an automation: `GET /automations/{id}/order-activity`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_automation_ecommerce_orders
//! ```

use mailerlite_rs::{
    MailerLite,
    automations::{ListAutomationOrders, TimeFrame},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let orders = client
        .automations()
        .ecommerce_orders(
            "Your Automation ID",
            &ListAutomationOrders::new(TimeFrame::Month),
        )
        .await?;

    println!("{orders:#?}");

    Ok(())
}
