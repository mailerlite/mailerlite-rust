//! Subscriber lifecycle: upsert, fetch, list, update, inspect activity and delete.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example subscribers
//! ```
//!
//! Runs against your real account: it creates `rust-sdk-example@example.com` and deletes it again.

use futures_util::{StreamExt, TryStreamExt};
use mailerlite_rs::{
    MailerLite,
    subscribers::{
        CreateSubscriber, ListActivity, ListSubscribers, SubscriberStatus, UpdateSubscriber,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;
    let subscribers = client.subscribers();

    // Create or update (upsert by email).
    let upserted = subscribers
        .create(
            &CreateSubscriber::new("rust-sdk-example@example.com")
                .field("name", "Rusty")
                .field("last_name", "Crab")
                .status(SubscriberStatus::Active),
        )
        .await?;
    let id = upserted.data.id.clone();
    println!(
        "{} subscriber {id}",
        if upserted.created {
            "created"
        } else {
            "updated"
        }
    );

    // Fetch by id or email.
    let subscriber = subscribers.get("rust-sdk-example@example.com").await?;
    println!(
        "{} is {} with {} groups",
        subscriber.email,
        subscriber.status,
        subscriber.groups.len()
    );

    // Update: set one field, clear another.
    subscribers
        .update(
            &id,
            &UpdateSubscriber::new()
                .field("company", "Ferris Inc")
                .field("last_name", None::<&str>),
        )
        .await?;

    // One page, then a lazy walk over the first few active subscribers.
    let page = subscribers.list(&ListSubscribers::new().limit(5)).await?;
    println!(
        "first page: {} subscribers, more: {}",
        page.data.len(),
        page.has_more()
    );

    let active = subscribers.list_all(
        ListSubscribers::new()
            .status(SubscriberStatus::Active)
            .limit(100),
    );
    let first_ten: Vec<_> = active.take(10).try_collect().await?;
    println!("streamed {} active subscribers", first_ten.len());

    println!("total subscribers: {}", subscribers.count().await?);

    for entry in subscribers
        .activity(&id, &ListActivity::new().limit(10))
        .await?
    {
        println!("activity: {} at {:?}", entry.log_name, entry.created_at);
    }

    subscribers.delete(&id).await?;
    println!("deleted {id}");

    Ok(())
}
