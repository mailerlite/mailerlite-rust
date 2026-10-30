//! List a shop's customers: `GET /ecommerce/shops/{shop_id}/customers`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_ecommerce_customers
//! ```

use mailerlite_rs::{MailerLite, ecommerce::ListCustomers};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let page = shop.customers().list(&ListCustomers::new()).await?;

    println!("{page:#?}");

    Ok(())
}
