//! Rename an existing subscriber segment.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_SEGMENT_ID`, `MAILERLITE_SEGMENT_NAME`.

use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::segments::UpdateSegment;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_SEGMENT_ID")?;
    let name = std::env::var("MAILERLITE_SEGMENT_NAME")?;
    let segment = mailerlite
        .segments()
        .update(id, UpdateSegment::new(name))
        .await?;
    println!(
        "Updated segment: {} ({})",
        segment.data.id, segment.data.name
    );
    Ok(())
}
