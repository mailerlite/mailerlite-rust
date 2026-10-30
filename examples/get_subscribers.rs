//! Fetch the first page of active subscribers.
//!
//! Requires `MAILERLITE_API_TOKEN`.

use mailerlite_rs::MailerLite;
use mailerlite_rs::models::subscribers::SubscriberStatus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let page = mailerlite
        .subscribers()
        .list()
        .status(SubscriberStatus::Active)
        .limit(25)
        .send()
        .await?;
    for subscriber in &page.data.data {
        println!("{}: {}", subscriber.id, subscriber.email);
    }
    println!("Next cursor: {:?}", page.data.next_cursor());
    Ok(())
}
