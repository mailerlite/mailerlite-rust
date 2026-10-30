//! List all variants of a product.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_PRODUCT_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let product_id = std::env::var("MAILERLITE_PRODUCT_ID")?;

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .products()
        .variants(product_id)
        .list()
        .await?;

    for variant in response.data {
        println!("{}", variant.id);
    }

    Ok(())
}
