//! Assign an existing subscriber to a group.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_GROUP_ID`, `MAILERLITE_SUBSCRIBER_ID`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let group_id = std::env::var("MAILERLITE_GROUP_ID")?;
    let subscriber_id = std::env::var("MAILERLITE_SUBSCRIBER_ID")?;
    let group = mailerlite.groups().assign(group_id, subscriber_id).await?;
    println!("Assigned subscriber to group: {}", group.data.id);
    Ok(())
}
