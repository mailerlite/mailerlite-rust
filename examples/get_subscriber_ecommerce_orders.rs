//! List attributed orders for a subscriber in a shop.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_SUBSCRIBER_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let subscriber_id = std::env::var("MAILERLITE_SUBSCRIBER_ID")?;

    let response = mailerlite
        .ecommerce()
        .subscriber_orders(subscriber_id, shop_id)
        .limit(25)
        .send()
        .await?;

    for order in response.data.data {
        println!("{}: {}", order.updated_at, order.total);
    }

    Ok(())
}
