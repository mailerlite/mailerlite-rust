//! Update a subscriber: `PUT /subscribers/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_subscriber
//! ```

use mailerlite_rs::{MailerLite, subscribers::UpdateSubscriber};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let subscriber = client
        .subscribers()
        .update(
            "Your Subscriber ID",
            &UpdateSubscriber::new().field("name", "John"),
        )
        .await?;

    println!("{subscriber:#?}");

    Ok(())
}
