//! Update an existing e-commerce product.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_PRODUCT_ID`.
//! Also required: `MAILERLITE_PRODUCT_NAME`, `MAILERLITE_PRODUCT_PRICE` (decimal text).
//! Replaces the product's name and price with these supplied values.
use mailerlite_rs::resources::ecommerce::params::SaveProduct;
use mailerlite_rs::{Decimal, MailerLite};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let product_id = std::env::var("MAILERLITE_PRODUCT_ID")?;
    let product_name = std::env::var("MAILERLITE_PRODUCT_NAME")?;
    let product_price: Decimal = std::env::var("MAILERLITE_PRODUCT_PRICE")?.parse()?;

    let params = SaveProduct::new(product_name).price(product_price);

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .products()
        .update(product_id, params)
        .await?;

    println!("{}", response.data.id);

    Ok(())
}
