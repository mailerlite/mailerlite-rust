//! Delete a field: `DELETE /fields/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_field
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    client.fields().delete("Your Field ID").await?;

    println!("deleted");

    Ok(())
}
