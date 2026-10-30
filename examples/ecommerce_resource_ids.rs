//! E-commerce with your platform's own ids (`with_resource_id`) and bulk imports.
//!
//! ```sh
//! MAILERLITE_API_KEY=... MAILERLITE_SHOP_ID=... cargo run --example ecommerce_resource_ids
//! ```
//!
//! With `with_resource_id()`, every id below the shop, in paths *and* bodies, is your own
//! `resource_id`, so you never have to store MailerLite's ids. Runs against your real account.

use mailerlite_rs::{
    MailerLite,
    ecommerce::{CreateCategory, CreateCustomer, CreateProduct},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;
    // The shop is addressed by its MailerLite id; everything below it by our own ids.
    let shop = client
        .ecommerce()
        .shop(std::env::var("MAILERLITE_SHOP_ID")?)
        .with_resource_id();

    // Imports return immediately; customers are processed in the background.
    let categories = shop
        .imports()
        .categories(&[CreateCategory::new("Mugs").resource_id("cat-mugs")])
        .await?;
    let products = shop
        .imports()
        .products(&[CreateProduct::new("Ferris mug")
            .price(12.5)
            .resource_id("sku-ferris-mug")])
        .await?;
    shop.imports()
        .customers(&[CreateCustomer::new("rust-sdk-import@example.com").resource_id("cust-42")])
        .await?;
    println!(
        "imported {} categories and {} products",
        categories.len(),
        products.len()
    );

    let product = shop.products().get("sku-ferris-mug").await?;
    println!("{} costs {}", product.name, product.price);

    shop.categories()
        .assign_product("cat-mugs", "sku-ferris-mug")
        .await?;
    shop.products().delete("sku-ferris-mug").await?;
    shop.categories().delete("cat-mugs").await?;

    Ok(())
}
