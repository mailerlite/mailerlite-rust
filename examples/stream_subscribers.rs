//! Stream every subscriber, fetching pages lazily (`stream` feature).
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example stream_subscribers
//! ```

use futures_util::TryStreamExt;
use mailerlite_rs::{MailerLite, subscribers::ListSubscribers};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let subscribers = client.subscribers();
    let mut all = std::pin::pin!(subscribers.list_all(ListSubscribers::new().limit(100)));
    while let Some(subscriber) = all.try_next().await? {
        println!("{} ({})", subscriber.email, subscriber.status);
    }

    Ok(())
}
