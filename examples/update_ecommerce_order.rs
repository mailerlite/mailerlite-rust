//! Update an existing e-commerce order.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_ORDER_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::{OrderStatus, UpdateOrder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let order_id = std::env::var("MAILERLITE_ORDER_ID")?;

    let params = UpdateOrder::new().status(OrderStatus::Complete);

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .orders()
        .update(order_id, params)
        .await?;

    println!("{}", response.data.id);

    Ok(())
}
