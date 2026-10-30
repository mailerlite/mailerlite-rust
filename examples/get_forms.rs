//! List forms of a type: `GET /forms/{type}`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_forms
//! ```

use mailerlite_rs::{
    MailerLite,
    forms::{FormType, ListForms},
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client
        .forms()
        .list(&FormType::Embedded, &ListForms::new().limit(25))
        .await?;

    println!("{page:#?}");

    Ok(())
}
