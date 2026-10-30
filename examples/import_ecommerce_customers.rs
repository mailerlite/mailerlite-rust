//! Import e-commerce customers using a typed array payload. Returned entries acknowledge background processing, not completion.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_CUSTOMER_EMAIL`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::{ImportCustomer, SaveCustomer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let customer_email = std::env::var("MAILERLITE_CUSTOMER_EMAIL")?;

    let entry = ImportCustomer::new(SaveCustomer::new(customer_email));
    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .customers()
        .import(vec![entry])
        .await?;
    println!("Returned {} entries", response.data.len());
    Ok(())
}
