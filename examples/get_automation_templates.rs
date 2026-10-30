//! Get automation templates.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::ListAutomationTemplates;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automation_templates()
        .list(
            &ListAutomationTemplates::new()
                .template_type("welcome")
                .category("onboarding")
                .page(2)
                .limit(10),
        )
        .await?;
    println!("{response:#?}");
    Ok(())
}
