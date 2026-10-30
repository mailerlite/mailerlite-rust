//! Delete a webhook: `DELETE /webhooks/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_webhook
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    client.webhooks().delete("Your Webhook ID").await?;

    println!("deleted");

    Ok(())
}
