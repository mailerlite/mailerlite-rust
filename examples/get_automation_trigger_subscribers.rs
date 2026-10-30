//! Get automation trigger subscribers.

use mailerlite_rs::MailerLite;
use mailerlite_rs::Sort;
use mailerlite_rs::automations::{
    ListWorkflowSubscribers, WorkflowSubscriberSort, WorkflowSubscriberStatus,
};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;
    let response = client
        .automation_triggers()
        .subscribers(
            "Your Automation ID",
            "Your Step or Email ID",
            &ListWorkflowSubscribers::new(WorkflowSubscriberStatus::Completed)
                .sort(Sort::asc(WorkflowSubscriberSort::Date))
                .search("ann@example.com")
                .page(2)
                .limit(10),
        )
        .await?;
    println!("{response:#?}");
    Ok(())
}
