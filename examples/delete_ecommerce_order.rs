//! Delete an order (and its cart): `DELETE /ecommerce/shops/{shop_id}/orders/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_ecommerce_order
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let order = shop.orders().delete("Your Order ID").await?;

    println!("deleted {order:#?}");

    Ok(())
}
