//! Replace an automation delay step configuration with a delay measured in days.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_STEP_ID`.
//! Also requires `MAILERLITE_DELAY_DAYS` (unsigned integer) and
//! `MAILERLITE_RESCHEDULE_WAITING_SUBSCRIBERS` (`true` or `false`).
//! Replaces the complete delay configuration; the flag controls rescheduling waiting subscribers.
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
    let days: u64 = std::env::var("MAILERLITE_DELAY_DAYS")?.parse()?;
    let reschedule: bool = std::env::var("MAILERLITE_RESCHEDULE_WAITING_SUBSCRIBERS")?.parse()?;

    let response = mailerlite
        .automation_steps()
        .update(
            automation_id,
            step_id,
            UpdateStep::new()
                .configuration(StepConfiguration::Delay(DelayConfiguration::new(
                    DelayUnit::Days,
                    days,
                )))?
                .reschedule_waiting_subscribers(reschedule),
        )
        .await?;

    println!("Updated step {}", response.data.id);

    Ok(())
}
