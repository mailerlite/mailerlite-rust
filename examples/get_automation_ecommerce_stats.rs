//! Read automation e-commerce monthly totals.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_actions::AutomationReportQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;

    let params = AutomationReportQuery::new();

    let response = mailerlite.automations().ecommerce_stats(id, params).await?;

    println!(
        "Orders: {}; revenue: {}",
        response.data.ecommerce.orders_count, response.data.ecommerce.revenue
    );

    Ok(())
}
