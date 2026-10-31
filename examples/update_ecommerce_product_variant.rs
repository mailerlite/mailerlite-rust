//! Update a product variant and return its containing product.
//!
//! Required environment: `MAILERLITE_API_TOKEN`, `MAILERLITE_SHOP_ID`, `MAILERLITE_PRODUCT_ID`, `MAILERLITE_VARIANT_ID`.
//! Also required: `MAILERLITE_VARIANT_NAME`, `MAILERLITE_VARIANT_ATTRIBUTES` (JSON string-to-string object),
//! `MAILERLITE_VARIANT_PRICE` (decimal text), and `MAILERLITE_VARIANT_PRICE_TYPE` (`fixed` or `increment`).
//! Replaces the variant's name, complete attribute map, price, and price type.
use mailerlite_rs::resources::ecommerce::params::{PriceType, SaveVariant};
use mailerlite_rs::{Decimal, MailerLite};
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let shop_id = std::env::var("MAILERLITE_SHOP_ID")?;
    let product_id = std::env::var("MAILERLITE_PRODUCT_ID")?;
    let variant_id = std::env::var("MAILERLITE_VARIANT_ID")?;
    let name = std::env::var("MAILERLITE_VARIANT_NAME")?;
    let attributes: BTreeMap<String, String> =
        serde_json::from_str(&std::env::var("MAILERLITE_VARIANT_ATTRIBUTES")?)?;
    let price: Decimal = std::env::var("MAILERLITE_VARIANT_PRICE")?.parse()?;
    let price_type = match std::env::var("MAILERLITE_VARIANT_PRICE_TYPE")?.as_str() {
        "fixed" => PriceType::Fixed,
        "increment" => PriceType::Increment,
        _ => return Err("MAILERLITE_VARIANT_PRICE_TYPE must be fixed or increment".into()),
    };

    let variant = SaveVariant::new(name, attributes)
        .price_type(price_type)
        .price(price);

    let response = mailerlite
        .ecommerce()
        .shop(shop_id)
        .products()
        .variants(product_id)
        .update(variant_id, variant)
        .await?;

    println!("{}", response.data.id);

    Ok(())
}
