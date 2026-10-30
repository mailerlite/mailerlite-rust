//! List a shop's products while retaining exact decimal prices.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop = std::env::var("MAILERLITE_SHOP_ID")?;
    let response = mailerlite
        .ecommerce()
        .shop(shop)
        .products()
        .list()
        .limit(25)
        .send()
        .await?;
    for product in response.data.data {
        match product.price {
            Some(price) => println!("{}: {price}", product.id),
            None => println!("{}: price not returned", product.id),
        }
    }
    Ok(())
}
