//! Delete a subscriber group.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_GROUP_ID`.

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_GROUP_ID")?;

    mailerlite.groups().delete(&id).await?;

    println!("Deleted group: {id}");

    Ok(())
}
