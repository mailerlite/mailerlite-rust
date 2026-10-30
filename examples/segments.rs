//! Segments: list, rename and read subscribers. Segments are created in the MailerLite app.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example segments
//! ```

use mailerlite_rs::{MailerLite, segments::ListSegments, subscribers::ListSubscribers};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;
    let segments = client.segments();

    let page = segments.list(&ListSegments::new().limit(10)).await?;
    for segment in &page.data {
        println!("{}: {} subscribers", segment.name, segment.total);
    }

    if let Some(segment) = page.data.first() {
        let members = segments
            .subscribers(&segment.id, &ListSubscribers::new().limit(5))
            .await?;
        for subscriber in &members.data {
            println!("  {}", subscriber.email);
        }
        // Renaming is the only write besides delete: `segments.update(&segment.id, "New name")`.
    }

    Ok(())
}
