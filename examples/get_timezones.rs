//! List timezones: `GET /timezones`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_timezones
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let timezones = client.timezones().list().await?;

    println!("{timezones:#?}");

    Ok(())
}
