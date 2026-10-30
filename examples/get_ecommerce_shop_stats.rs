//! Read shop revenue statistics over an inclusive YYYY-MM-DD date range.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_START_DATE`, `MAILERLITE_END_DATE`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let start_date = std::env::var("MAILERLITE_START_DATE")?;
    let end_date = std::env::var("MAILERLITE_END_DATE")?;

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .stats(start_date, end_date)
        .await?;

    println!(
        "Orders: {}; revenue: {}",
        response.data.ecommerce.orders_count, response.data.ecommerce.revenue
    );

    Ok(())
}
