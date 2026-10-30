//! Update an order's status or total: `PUT /ecommerce/shops/{shop_id}/orders/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_ecommerce_order
//! ```

use mailerlite_rs::{
    MailerLite,
    ecommerce::{OrderStatus, UpdateOrder},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let order = shop
        .orders()
        .update(
            "Your Order ID",
            &UpdateOrder::new().status(OrderStatus::Complete),
        )
        .await?;

    println!("{order:#?}");

    Ok(())
}
