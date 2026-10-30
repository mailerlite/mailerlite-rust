//! Rename a form: `PUT /forms/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example update_form
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let form = client
        .forms()
        .update("Your Form ID", "Renamed form")
        .await?;

    println!("{form:#?}");

    Ok(())
}
