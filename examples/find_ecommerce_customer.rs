//! Get a customer: `GET /ecommerce/shops/{shop_id}/customers/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example find_ecommerce_customer
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let customer = shop.customers().get("Your Customer ID").await?;

    println!("{customer:#?}");

    Ok(())
}
