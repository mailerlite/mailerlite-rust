//! Rename a subscriber group.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_GROUP_ID`, `MAILERLITE_GROUP_NAME`.

use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::groups::UpdateGroup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_GROUP_ID")?;
    let name = std::env::var("MAILERLITE_GROUP_NAME")?;

    let group = mailerlite
        .groups()
        .update(id, UpdateGroup::new(name))
        .await?;

    println!("Updated group: {} ({})", group.data.id, group.data.name);

    Ok(())
}
