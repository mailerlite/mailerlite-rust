//! Groups: create, rename, manage membership, bulk import and clean up.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example groups
//! ```
//!
//! Runs against your real account: it creates a temporary group and subscriber and deletes both.

use mailerlite_rs::{
    MailerLite, Sort,
    groups::{GroupSort, ImportSubscriber, ListGroups},
    subscribers::{CreateSubscriber, ListSubscribers},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;
    let groups = client.groups();

    let largest = groups
        .list(
            &ListGroups::new()
                .sort(Sort::desc(GroupSort::ActiveCount))
                .limit(3),
        )
        .await?;
    for group in &largest.data {
        println!(
            "{}: {} active, open rate {}",
            group.name, group.active_count, group.open_rate.string
        );
    }

    let group = groups.create("Rust SDK example").await?;
    let group = groups
        .update(&group.id, "Rust SDK example (renamed)")
        .await?;
    println!("created group {} ({})", group.name, group.id);

    let subscriber = client
        .subscribers()
        .create(&CreateSubscriber::new("rust-sdk-group@example.com"))
        .await?
        .data;
    groups.assign_subscriber(&group.id, &subscriber.id).await?;

    let members = groups
        .subscribers(&group.id, &ListSubscribers::new().limit(10))
        .await?;
    println!(
        "group has {} member(s) on the first page",
        members.data.len()
    );

    // Bulk import runs in the background (5 imports per minute per account).
    let started = groups
        .import_subscribers(
            &group.id,
            &[ImportSubscriber::new("rust-sdk-import@example.com").field("name", "Imported")],
        )
        .await?;
    if let Some(import_id) = started.import_id() {
        let status = client.subscribers().import_status(import_id).await?;
        println!("import {import_id}: {}% done", status.percent);
    }

    groups
        .unassign_subscriber(&group.id, &subscriber.id)
        .await?;
    client.subscribers().delete(&subscriber.id).await?;
    groups.delete(&group.id).await?;
    println!("cleaned up");

    Ok(())
}
