//! Fetch one page of subscribers belonging to a group.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_GROUP_ID`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_GROUP_ID")?;

    let page = mailerlite.groups().subscribers(id).limit(25).send().await?;

    for subscriber in &page.data.data {
        println!("{}: {}", subscriber.id, subscriber.email);
    }

    println!("Next cursor: {:?}", page.data.next_cursor());

    Ok(())
}
