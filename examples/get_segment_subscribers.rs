//! Fetch one page of subscribers matching a segment.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_SEGMENT_ID`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_SEGMENT_ID")?;

    let page = mailerlite
        .segments()
        .subscribers(id)
        .limit(25)
        .send()
        .await?;

    for subscriber in &page.data.data {
        println!("{}: {}", subscriber.id, subscriber.email);
    }

    println!("Next cursor: {:?}", page.data.next_cursor());

    Ok(())
}
