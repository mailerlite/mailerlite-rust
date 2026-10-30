//! Import typed e-commerce orders; the SDK adds the `orders` request wrapper.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_CUSTOMER_EMAIL`, `MAILERLITE_PRODUCT_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::{
    CreateOrder, OrderCart, SaveCartItem, SaveCustomer,
};
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let customer_email = std::env::var("MAILERLITE_CUSTOMER_EMAIL")?;
    let product_id = std::env::var("MAILERLITE_PRODUCT_ID")?;

    let entry = CreateOrder::new(
        SaveCustomer::new(customer_email),
        OrderCart::new(vec![
            SaveCartItem::new(product_id)
                .quantity(1)
                .price(Decimal::new(1999, 2)),
        ]),
    )
    .total_price(Decimal::new(1999, 2));
    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .orders()
        .import(vec![entry])
        .await?;
    println!("Returned {} entries", response.data.len());
    Ok(())
}
