//! Create an e-commerce order.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_CUSTOMER_EMAIL`, `MAILERLITE_PRODUCT_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::{
    CreateOrder, OrderCart, OrderStatus, SaveCartItem, SaveCustomer,
};
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let customer_email = std::env::var("MAILERLITE_CUSTOMER_EMAIL")?;
    let product_id = std::env::var("MAILERLITE_PRODUCT_ID")?;

    let params = CreateOrder::new(
        SaveCustomer::new(customer_email),
        OrderCart::new(vec![
            SaveCartItem::new(product_id)
                .quantity(1)
                .price(Decimal::new(1999, 2)),
        ]),
    )
    .status(OrderStatus::Pending)
    .total_price(Decimal::new(1999, 2));

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .orders()
        .create(params)
        .await?;

    println!("{}", response.data.id);

    Ok(())
}
