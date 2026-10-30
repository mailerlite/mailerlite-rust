//! Read attributed purchases for a campaign in a shop.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_CAMPAIGN_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let campaign_id = std::env::var("MAILERLITE_CAMPAIGN_ID")?;

    let response = mailerlite
        .ecommerce()
        .campaign_stats(campaign_id, shop_id)
        .await?;

    println!(
        "Orders: {}; revenue: {}",
        response.data.ecommerce.orders_count, response.data.ecommerce.revenue
    );

    Ok(())
}
