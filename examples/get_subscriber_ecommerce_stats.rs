//! Read attributed purchases for a subscriber in a shop.
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
        .subscriber_stats(subscriber_id, shop_id)
        .await?;
    println!(
        "Orders: {}; revenue: {}",
        response.data.ecommerce.orders_count, response.data.ecommerce.revenue
    );
    Ok(())
}
