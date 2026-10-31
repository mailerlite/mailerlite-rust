//! Create an automation delay step.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_PARENT_STEP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::models::automation_configuration::{
    DelayConfiguration, DelayUnit, StepConfiguration,
};
use mailerlite_rs::resources::automation_steps::{CreateStep, StepType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let automation_id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let parent_step_id = std::env::var("MAILERLITE_PARENT_STEP_ID")?;

    let response = mailerlite
        .automation_steps()
        .create(
            automation_id,
            CreateStep::new(StepType::Delay)
                .parent_id(parent_step_id)
                .configuration(StepConfiguration::Delay(DelayConfiguration::new(
                    DelayUnit::Days,
                    1,
                )))?,
        )
        .await?;

    println!("Created step {}", response.data.id);

    Ok(())
}
