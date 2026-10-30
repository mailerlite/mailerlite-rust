//! Rename a segment: `PUT /segments/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_segment
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let segment = client
        .segments()
        .update("Your Segment ID", "Renamed segment")
        .await?;

    println!("{segment:#?}");

    Ok(())
}
