//! Bulk import orders: `POST /ecommerce/shops/{shop_id}/orders/import`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example import_ecommerce_orders
//! ```

use mailerlite_rs::{
    MailerLite,
    ecommerce::{CreateCartItem, CreateCustomer, CreateOrder, OrderCart},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let orders = shop
        .imports()
        .orders(&[CreateOrder::new(
            CreateCustomer::new("john@example.com"),
            OrderCart::new([CreateCartItem::new("Your Product ID").quantity(1)]),
        )])
        .await?;

    println!("{orders:#?}");

    Ok(())
}
