//! Replace an automation delay step configuration.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_STEP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::models::automation_configuration::{
    DelayConfiguration, DelayUnit, StepConfiguration,
};
use mailerlite_rs::resources::automation_steps::UpdateStep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let automation_id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let step_id = std::env::var("MAILERLITE_STEP_ID")?;
    let response = mailerlite
        .automation_steps()
        .update(
            automation_id,
            step_id,
            UpdateStep::new()
                .configuration(StepConfiguration::Delay(DelayConfiguration::new(
                    DelayUnit::Days,
                    2,
                )))?
                .reschedule_waiting_subscribers(true),
        )
        .await?;
    println!("Updated step {}", response.data.id);
    Ok(())
}
