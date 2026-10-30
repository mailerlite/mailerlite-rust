//! List subscribers who signed up through a form: `GET /forms/{id}/subscribers`.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example get_form_subscribers
//! ```

use mailerlite_rs::{MailerLite, subscribers::ListSubscribers};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let page = client
        .forms()
        .subscribers("Your Form ID", &ListSubscribers::new().limit(25))
        .await?;

    println!("{page:#?}");

    Ok(())
}
