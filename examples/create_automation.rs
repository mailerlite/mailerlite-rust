//! Create a draft automation: `POST /automations`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_automation
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let automation = client.automations().create("New automation").await?;

    println!("{automation:#?}");

    Ok(())
}
