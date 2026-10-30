//! Get a subscriber's activity log: `GET /subscribers/{id}/activity-log`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_subscriber_activity
//! ```

use mailerlite_rs::{MailerLite, subscribers::ListActivity};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let activity = client
        .subscribers()
        .activity("Your Subscriber ID", &ListActivity::new().limit(100))
        .await?;

    println!("{activity:#?}");

    Ok(())
}
