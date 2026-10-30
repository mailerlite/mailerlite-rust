//! Create an automation from a template.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_TEMPLATE_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let template_id = std::env::var("MAILERLITE_TEMPLATE_ID")?;

    let response = mailerlite
        .automation_templates()
        .select(template_id)
        .await?;

    println!("HTTP {}", response.metadata.status);
    println!(
        "Automation {}: building = {:?}",
        response.data.id, response.data.building_in_progress
    );

    // Workflow creation is asynchronous. Poll mailerlite.automations().get(id)
    // until building_in_progress clears before configuring or enabling it.
    Ok(())
}
