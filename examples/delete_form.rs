//! Delete a form: `DELETE /forms/{id}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example delete_form
//! ```

use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    client.forms().delete("Your Form ID").await?;

    println!("deleted");

    Ok(())
}
