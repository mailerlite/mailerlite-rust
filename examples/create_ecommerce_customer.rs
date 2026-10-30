//! Create a customer: `POST /ecommerce/shops/{shop_id}/customers`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_ecommerce_customer
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateCustomer};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let customer = shop
        .customers()
        .create(&CreateCustomer::new("john@example.com").accepts_marketing(true))
        .await?;

    println!("{customer:#?}");

    Ok(())
}
