//! List webhooks: `GET /webhooks`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_webhooks
//! ```

use mailerlite_rs::{MailerLite, webhooks::ListWebhooks};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client.webhooks().list(&ListWebhooks::new()).await?;

    println!("{page:#?}");

    Ok(())
}
