//! Update an existing e-commerce customer.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_CUSTOMER_ID`, `MAILERLITE_CUSTOMER_EMAIL`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::SaveCustomer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let customer_id = std::env::var("MAILERLITE_CUSTOMER_ID")?;
    let customer_email = std::env::var("MAILERLITE_CUSTOMER_EMAIL")?;

    let params = SaveCustomer::new(customer_email);
    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .customers()
        .update(customer_id, params)
        .await?;
    println!("{}", response.data.id);
    Ok(())
}
