//! Get automation email stats.

use mailerlite_rs::MailerLite;
use mailerlite_rs::automations::AutomationEmailStatsQuery;

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automations()
        .email_stats(
            "Your Automation ID",
            &AutomationEmailStatsQuery::new()
                .start_date("2026-01-01")
                .end_date("2026-01-31"),
        )
        .await?;
    println!("{response:#?}");
    Ok(())
}
