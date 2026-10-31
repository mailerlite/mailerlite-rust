//! Update an existing e-commerce product.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_PRODUCT_ID`.
use mailerlite_rs::resources::ecommerce::params::SaveProduct;
use mailerlite_rs::{Decimal, MailerLite};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let product_id = std::env::var("MAILERLITE_PRODUCT_ID")?;

    let params = SaveProduct::new("Example product").price(Decimal::new(1999, 2));

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .products()
        .update(product_id, params)
        .await?;

    println!("{}", response.data.id);

    Ok(())
}
