//! Automations: list, inspect steps and activity, create and delete a draft.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example automations
//! ```

use mailerlite_rs::{
    MailerLite,
    automations::{AutomationActivityStatus, ListAutomationActivity, ListAutomations},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;
    let automations = client.automations();

    let enabled = automations
        .list(&ListAutomations::new().enabled(true).limit(5))
        .await?;
    for automation in &enabled.data {
        println!("{} ({} steps)", automation.name, automation.steps.len());
        for step in &automation.steps {
            println!("  - {:?}: {}", step.step_type, step.description);
        }
    }

    if let Some(first) = enabled.data.first() {
        let completed = automations
            .activity(
                &first.id,
                &ListAutomationActivity::new(AutomationActivityStatus::Completed).limit(10),
            )
            .await?;
        println!("{} completed runs on the first page", completed.data.len());
    }

    let draft = automations.create("Rust SDK example").await?;
    println!("created draft automation {}", draft.id);
    automations.delete(&draft.id).await?;
    println!("deleted {}", draft.id);

    Ok(())
}
