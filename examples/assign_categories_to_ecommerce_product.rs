//! Add category membership to a product while preserving existing memberships.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_PRODUCT_ID`, `MAILERLITE_CATEGORY_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let product_id = std::env::var("MAILERLITE_PRODUCT_ID")?;
    let category_id = std::env::var("MAILERLITE_CATEGORY_ID")?;

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .products()
        .assign_categories(product_id, vec![category_id], false)
        .await?;
    println!("{}", response.data.id);
    Ok(())
}
