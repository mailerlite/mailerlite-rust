//! Create a custom field: `POST /fields`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example create_field
//! ```

use mailerlite_rs::{MailerLite, fields::FieldType};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let field = client.fields().create("Score", FieldType::Number).await?;

    println!("{field:#?}");

    Ok(())
}
