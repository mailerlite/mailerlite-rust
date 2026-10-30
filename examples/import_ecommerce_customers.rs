//! Bulk import customers: `POST /ecommerce/shops/{shop_id}/customers/import`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example import_ecommerce_customers
//! ```

use mailerlite_rs::{MailerLite, ecommerce::CreateCustomer};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let customers = shop
        .imports()
        .customers(&[
            CreateCustomer::new("john@example.com"),
            CreateCustomer::new("jane@example.com").accepts_marketing(true),
        ])
        .await?;

    println!("{customers:#?}");

    Ok(())
}
