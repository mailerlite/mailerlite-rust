//! Timezones, e.g. to pick a `timezone_id` for scheduling campaigns.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example timezones
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;

    let timezones = client.timezones().list().await?;
    for timezone in timezones
        .iter()
        .filter(|timezone| timezone.name.starts_with("Europe/"))
    {
        println!(
            "{:>4} {} ({})",
            timezone.id, timezone.name, timezone.offset_name
        );
    }

    Ok(())
}
