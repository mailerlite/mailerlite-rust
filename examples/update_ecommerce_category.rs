//! Update an existing e-commerce category.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_CATEGORY_ID`.
//! Also required: `MAILERLITE_CATEGORY_NAME`, which replaces the category name.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::SaveCategory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let category_id = std::env::var("MAILERLITE_CATEGORY_ID")?;
    let name = std::env::var("MAILERLITE_CATEGORY_NAME")?;

    let params = SaveCategory::new(name);

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .categories()
        .update(category_id, params)
        .await?;

    println!("{}", response.data.id);

    Ok(())
}
