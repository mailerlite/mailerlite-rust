//! Update an existing e-commerce order.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_ORDER_ID`.
//! Also required: `MAILERLITE_ORDER_STATUS` (`pending` or `complete`), which replaces the order status.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::{OrderStatus, UpdateOrder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let order_id = std::env::var("MAILERLITE_ORDER_ID")?;
    let status = match std::env::var("MAILERLITE_ORDER_STATUS")?.as_str() {
        "pending" => OrderStatus::Pending,
        "complete" => OrderStatus::Complete,
        _ => return Err("MAILERLITE_ORDER_STATUS must be pending or complete".into()),
    };

    let params = UpdateOrder::new().status(status);

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .orders()
        .update(order_id, params)
        .await?;

    println!("{}", response.data.id);

    Ok(())
}
