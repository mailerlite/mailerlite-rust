//! Delete an e-commerce order and display the returned ID.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_ORDER_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let order_id = std::env::var("MAILERLITE_ORDER_ID")?;

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .orders()
        .delete(order_id)
        .await?;
    println!("{}", response.data.id);
    Ok(())
}
