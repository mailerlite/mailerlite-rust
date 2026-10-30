//! Delete a subscriber segment.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_SEGMENT_ID`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_SEGMENT_ID")?;
    mailerlite.segments().delete(&id).await?;
    println!("Deleted segment: {id}");
    Ok(())
}
