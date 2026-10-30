//! Read a page of automation order activity.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_actions::{
    AutomationOrderQuery, AutomationOrderTimeFrame,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let params = AutomationOrderQuery::new(AutomationOrderTimeFrame::AllTime);
    let response = mailerlite
        .automations()
        .ecommerce_orders(id, params)
        .await?;
    for order in response.data.data {
        println!(
            "Subscriber {}: order total {}",
            order.subscriber.id, order.total
        );
    }
    Ok(())
}
