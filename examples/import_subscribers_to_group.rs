//! Bulk import subscribers into a group: `POST /groups/{id}/import-subscribers`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example import_subscribers_to_group
//! ```

use mailerlite_rs::{MailerLite, groups::ImportSubscriber};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let started = client
        .groups()
        .import_subscribers(
            "Your Group ID",
            &[
                ImportSubscriber::new("john@example.com").field("name", "John"),
                ImportSubscriber::new("jane@example.com"),
            ],
        )
        .await?;

    println!("import id: {:?}", started.import_id());

    Ok(())
}
