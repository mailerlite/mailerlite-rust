//! Create an order with its customer and cart: `POST /ecommerce/shops/{shop_id}/orders`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_ecommerce_order
//! ```

use mailerlite_rs::{
    MailerLite,
    ecommerce::{CreateCartItem, CreateCustomer, CreateOrder, OrderCart, OrderStatus},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let order = shop
        .orders()
        .create(
            &CreateOrder::new(
                CreateCustomer::new("john@example.com"),
                OrderCart::new([CreateCartItem::new("Your Product ID")
                    .quantity(2)
                    .price(19.99)]),
            )
            .status(OrderStatus::Complete)
            .total_price(39.98),
        )
        .await?;

    println!("{order:#?}");

    Ok(())
}
