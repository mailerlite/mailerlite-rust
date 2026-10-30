//! Lazily consume subscribers using the `stream` feature.
//!
//! Requires `MAILERLITE_API_TOKEN` and the Cargo feature `stream`.
use futures_util::TryStreamExt;
use mailerlite_rs::{MailerLite, models::subscribers::SubscriberStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let subscribers = mailerlite.subscribers();
    let stream = subscribers
        .list()
        .status(SubscriberStatus::Active)
        .limit(100)
        .into_stream();
    tokio::pin!(stream);

    while let Some(subscriber) = stream.try_next().await? {
        println!("{}", subscriber.id);
    }

    Ok(())
}
