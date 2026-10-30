//! Rename a group: `PUT /groups/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_group
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let group = client
        .groups()
        .update("Your Group ID", "Renamed group")
        .await?;

    println!("{group:#?}");

    Ok(())
}
