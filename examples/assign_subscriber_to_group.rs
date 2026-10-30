//! Add a subscriber to a group: `POST /subscribers/{id}/groups/{group_id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example assign_subscriber_to_group
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let group = client
        .groups()
        .assign_subscriber("Your Group ID", "Your Subscriber ID")
        .await?;

    println!("{group:#?}");

    Ok(())
}
