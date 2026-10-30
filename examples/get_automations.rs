//! List automations: `GET /automations`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_automations
//! ```

use mailerlite_rs::{MailerLite, automations::ListAutomations};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client
        .automations()
        .list(&ListAutomations::new().limit(10))
        .await?;

    println!("{page:#?}");

    Ok(())
}
