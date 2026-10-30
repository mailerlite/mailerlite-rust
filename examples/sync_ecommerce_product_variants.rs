//! Create product variants. Adding resource IDs to the request also removes existing variants whose resource IDs are absent.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_PRODUCT_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::ecommerce::params::{PriceType, SaveVariant};
use rust_decimal::Decimal;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let product_id = std::env::var("MAILERLITE_PRODUCT_ID")?;

    let variant = SaveVariant::new(
        "Black / Medium",
        BTreeMap::from([
            ("color".into(), "Black".into()),
            ("size".into(), "Medium".into()),
        ]),
    )
    .price_type(PriceType::Fixed)
    .price(Decimal::new(1999, 2));
    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .products()
        .variants(product_id)
        .create(vec![variant])
        .await?;
    println!("{}", response.data.id);
    Ok(())
}
