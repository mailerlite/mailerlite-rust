//! Select an automation A/B test winner.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_STEP_ID`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_steps::AbWinner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let automation_id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let step_id = std::env::var("MAILERLITE_STEP_ID")?;

    let response = mailerlite
        .automation_steps()
        .select_ab_test_winner(automation_id, step_id, AbWinner::APath)
        .await?;

    println!(
        "Step {} winner: {:?}",
        response.data.step_id, response.data.ab_test_winner
    );

    Ok(())
}
