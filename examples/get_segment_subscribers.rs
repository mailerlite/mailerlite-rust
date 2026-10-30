//! List a segment's subscribers: `GET /segments/{id}/subscribers`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_segment_subscribers
//! ```

use mailerlite_rs::{MailerLite, subscribers::ListSubscribers};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client
        .segments()
        .subscribers("Your Segment ID", &ListSubscribers::new().limit(50))
        .await?;

    println!("{page:#?}");

    Ok(())
}
