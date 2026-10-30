//! List fields: `GET /fields`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_fields
//! ```

use mailerlite_rs::{MailerLite, fields::ListFields};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client.fields().list(&ListFields::new()).await?;

    println!("{page:#?}");

    Ok(())
}
