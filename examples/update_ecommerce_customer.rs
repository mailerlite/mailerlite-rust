//! Update a customer: `PUT /ecommerce/shops/{shop_id}/customers/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_ecommerce_customer
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateCustomer};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let customer = shop
        .customers()
        .update(
            "Your Customer ID",
            &CreateCustomer::new("john@example.com").name("John"),
        )
        .await?;

    println!("{customer:#?}");

    Ok(())
}
