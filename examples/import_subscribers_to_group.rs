//! Start a group import and retain its receipt for later status checks.
//! Requires MAILERLITE_API_TOKEN, MAILERLITE_GROUP_ID, and MAILERLITE_SUBSCRIBER_EMAIL.
use mailerlite_rs::{MailerLite, resources::imports::ImportSubscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let group = std::env::var("MAILERLITE_GROUP_ID")?;
    let email = std::env::var("MAILERLITE_SUBSCRIBER_EMAIL")?;

    let response = mailerlite
        .imports()
        .import_to_group(
            group,
            vec![ImportSubscriber::new(email).field("name", "Jane")],
        )
        .await?;

    println!("Import progress URL: {}", response.data.import_progress_url);

    // Acceptance is not completion. See group_import.rs for explicit polling.
    Ok(())
}
