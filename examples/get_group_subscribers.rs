//! List a group's subscribers: `GET /groups/{id}/subscribers`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_group_subscribers
//! ```

use mailerlite_rs::{MailerLite, subscribers::ListSubscribers};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client
        .groups()
        .subscribers("Your Group ID", &ListSubscribers::new().limit(50))
        .await?;

    println!("{page:#?}");

    Ok(())
}
