//! Read email report statistics.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_AUTOMATION_ID`, `MAILERLITE_EMAIL_ID`, `MAILERLITE_START_DATE`, `MAILERLITE_END_DATE`.
use mailerlite_rs::MailerLite;
use mailerlite_rs::resources::automation_actions::AutomationReportQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_AUTOMATION_ID")?;
    let email_id = std::env::var("MAILERLITE_EMAIL_ID")?;
    let params = AutomationReportQuery::new()
        .start_date(std::env::var("MAILERLITE_START_DATE")?)
        .end_date(std::env::var("MAILERLITE_END_DATE")?);
    let response = mailerlite
        .automations()
        .email_report_stats(id, email_id, params)
        .await?;
    println!(
        "Sent: {}; open rate: {}",
        response.data.engagement.sent, response.data.engagement.open_rate.string
    );
    Ok(())
}
