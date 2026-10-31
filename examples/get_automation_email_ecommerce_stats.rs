//! Read email e-commerce overview statistics.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_SHOP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_actions::AutomationReportQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let email_id = std::env::var("MAILERLITE_EMAIL_ID")?;

    let params = AutomationReportQuery::new().shop_id(std::env::var("MAILERLITE_SHOP_ID")?);

    let response = mailerlite
        .automations()
        .email_ecommerce_stats(id, email_id, params)
        .await?;

    println!(
        "Orders: {}; revenue: {}",
        response.data.ecommerce.orders_count, response.data.ecommerce.revenue
    );

    Ok(())
}
