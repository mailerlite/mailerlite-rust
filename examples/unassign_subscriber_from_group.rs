//! Remove a subscriber from a group: `DELETE /subscribers/{id}/groups/{group_id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example unassign_subscriber_from_group
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    client
        .groups()
        .unassign_subscriber("Your Group ID", "Your Subscriber ID")
        .await?;

    println!("removed");

    Ok(())
}
