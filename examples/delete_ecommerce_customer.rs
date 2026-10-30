//! Delete a customer: `DELETE /ecommerce/shops/{shop_id}/customers/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_ecommerce_customer
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let customer = shop.customers().delete("Your Customer ID").await?;

    println!("deleted {customer:#?}");

    Ok(())
}
