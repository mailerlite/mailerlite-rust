//! Start a group import and explicitly wait for its completion.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_GROUP_ID`, `MAILERLITE_SUBSCRIBER_EMAIL`.
use mailerlite_rs::{MailerLite, resources::imports::ImportSubscriber};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let group = std::env::var("MAILERLITE_GROUP_ID")?;
    let email = std::env::var("MAILERLITE_SUBSCRIBER_EMAIL")?;

    let imports = mailerlite.imports();
    let receipt = imports
        .import_to_group(group, vec![ImportSubscriber::new(email)])
        .await?;

    let report = imports
        .wait(
            &receipt.data,
            Duration::from_secs(5),
            Duration::from_secs(300),
        )
        .await?;

    println!("Import completed: {}", report.data.done);

    Ok(())
}
