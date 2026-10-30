//! Delete an automation: `DELETE /automations/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_automation
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    client.automations().delete("Your Automation ID").await?;

    println!("deleted");

    Ok(())
}
