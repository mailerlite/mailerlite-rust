//! Get an automation: `GET /automations/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example find_automation
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let automation = client.automations().get("Your Automation ID").await?;

    println!("{automation:#?}");

    Ok(())
}
