//! Delete a subscriber: `DELETE /subscribers/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_subscriber
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    client.subscribers().delete("Your Subscriber ID").await?;

    println!("deleted");

    Ok(())
}
