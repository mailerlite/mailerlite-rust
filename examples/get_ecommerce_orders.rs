//! List a shop's orders: `GET /ecommerce/shops/{shop_id}/orders`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_ecommerce_orders
//! ```

use mailerlite_rs::{MailerLite, ecommerce::ListOrders};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let page = shop.orders().list(&ListOrders::new()).await?;

    println!("{page:#?}");

    Ok(())
}
