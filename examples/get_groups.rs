//! List groups: `GET /groups`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_groups
//! ```

use mailerlite_rs::{MailerLite, groups::ListGroups};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client.groups().list(&ListGroups::new().limit(25)).await?;

    println!("{page:#?}");

    Ok(())
}
