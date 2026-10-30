//! Create a group: `POST /groups`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_group
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let group = client.groups().create("New group").await?;

    println!("{group:#?}");

    Ok(())
}
