//! Delete a segment: `DELETE /segments/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_segment
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    client.segments().delete("Your Segment ID").await?;

    println!("deleted");

    Ok(())
}
