//! Forms: list by type with aggregations, fetch one and read its signups.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example forms
//! ```

use mailerlite_rs::{
    MailerLite,
    forms::{FormType, ListForms},
    subscribers::ListSubscribers,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;
    let forms = client.forms();

    let page = forms
        .list(&FormType::Embedded, &ListForms::new().limit(10))
        .await?;
    if let Some(counts) = &page.aggregations {
        println!(
            "popup: {}, embedded: {}, promotion: {}",
            counts.popup, counts.embedded, counts.promotion
        );
    }

    if let Some(form) = page.data.first() {
        let form = forms.get(&form.id).await?;
        println!(
            "{}: {} conversions ({})",
            form.name, form.conversions_count, form.conversion_rate.string
        );

        let signups = forms
            .subscribers(&form.id, &ListSubscribers::new().limit(5))
            .await?;
        for subscriber in &signups.data {
            println!("  {}", subscriber.email);
        }
    }

    Ok(())
}
