//! Create a subscriber group.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_GROUP_NAME`.

use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::groups::CreateGroup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let name = std::env::var("MAILERLITE_GROUP_NAME")?;
    let group = mailerlite.groups().create(CreateGroup::new(name)).await?;
    println!("Created group: {} ({})", group.data.id, group.data.name);
    Ok(())
}
