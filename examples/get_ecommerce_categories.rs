//! List one page of e-commerce categories.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .categories()
        .list()
        .limit(25)
        .send()
        .await?;

    for item in response.data.data {
        println!("{}", item.id);
    }

    Ok(())
}
