//! Get the progress of a subscriber import: `GET /subscribers/import/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_subscriber_import_status
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let status = client.subscribers().import_status("Your Import ID").await?;

    println!("{status:#?}");

    Ok(())
}
