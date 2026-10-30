//! E-commerce: shop, catalogue, customer, order, cart and reports, then clean up.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example ecommerce
//! ```
//!
//! Runs against your real account: it creates a temporary shop with one category, product,
//! customer and order, prints the shop stats and deletes the shop again.

use mailerlite_rs::{
    MailerLite,
    ecommerce::{
        CreateCartItem, CreateCategory, CreateCustomer, CreateOrder, CreateProduct, CreateShop,
        ListOrders, OrderCart, OrderStatus, UpdateOrder,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;
    let ecommerce = client.ecommerce();

    println!(
        "attribution window: {} days",
        ecommerce
            .settings()
            .get()
            .await?
            .ecommerce_attribution_window
    );

    let shop = ecommerce
        .shops()
        .create(&CreateShop::new(
            "Rust SDK shop",
            "https://shop.example.com",
            "EUR",
        ))
        .await?;
    println!("created shop {}", shop.id);

    // Everything below is scoped to the shop, so its id is not repeated on every call.
    let scope = ecommerce.shop(shop.id.clone());

    let category = scope
        .categories()
        .create(&CreateCategory::new("Mugs"))
        .await?;
    let product = scope
        .products()
        .create(
            &CreateProduct::new("Ferris mug")
                .price(12.5)
                .url("https://shop.example.com/mug"),
        )
        .await?;
    scope
        .categories()
        .assign_product(&category.id, &product.id)
        .await?;

    let customer = CreateCustomer::new("rust-sdk-buyer@example.com").accepts_marketing(true);
    let cart = OrderCart::new([CreateCartItem::new(product.id.clone())
        .quantity(2)
        .price(12.5)]);
    let order = scope
        .orders()
        .create(&CreateOrder::new(customer, cart).total_price(25.0))
        .await?;
    println!("order {} is {:?}", order.id, order.status);

    // Pending orders trigger abandoned-cart automations; completing one triggers purchase flows.
    scope
        .orders()
        .update(&order.id, &UpdateOrder::new().status(OrderStatus::Complete))
        .await?;

    let cart = scope.carts().get(&order.cart.id).await?;
    println!("cart {} holds {} item(s)", cart.id, cart.items.len());

    let completed = scope
        .orders()
        .list(&ListOrders::new().status(OrderStatus::Complete))
        .await?;
    println!("{} completed order(s)", completed.meta.total);

    let stats = ecommerce
        .shops()
        .stats(&shop.id, "2026-01-01", "2026-12-31")
        .await?;
    println!(
        "revenue: {} from {} orders",
        stats.ecommerce.revenue, stats.ecommerce.orders_count
    );

    scope.orders().delete(&order.id).await?;
    ecommerce.shops().delete(&shop.id).await?;
    println!("cleaned up");

    Ok(())
}
