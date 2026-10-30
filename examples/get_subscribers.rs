//! List subscribers: `GET /subscribers`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_subscribers
//! ```

use mailerlite_rs::{MailerLite, subscribers::ListSubscribers};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client
        .subscribers()
        .list(&ListSubscribers::new().limit(25))
        .await?;

    println!("{page:#?}");

    Ok(())
}
