//! Sync a product's variants: `POST /ecommerce/shops/{shop_id}/products/{id}/variants`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example sync_ecommerce_product_variants
//! ```

use mailerlite_rs::{
    MailerLite,
    ecommerce::{CreateVariant, PriceType},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let shop = client.ecommerce().shop("Your Shop ID");
    let product = shop
        .variants("Your Product ID")
        .create(&[
            CreateVariant::new("Small").attribute("size", "S"),
            CreateVariant::new("Large")
                .attribute("size", "L")
                .price(PriceType::Increment, 5.0),
        ])
        .await?;

    println!("{product:#?}");

    Ok(())
}
