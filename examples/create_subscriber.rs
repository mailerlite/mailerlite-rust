//! Create or update (upsert) a subscriber: `POST /subscribers`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_subscriber
//! ```

use mailerlite_rs::{MailerLite, subscribers::CreateSubscriber};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let upserted = client
        .subscribers()
        .create(&CreateSubscriber::new("john@example.com").field("name", "John"))
        .await?;

    println!("created: {}\n{:#?}", upserted.created, upserted.data);

    Ok(())
}
