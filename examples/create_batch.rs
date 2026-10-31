//! Submit a batch and inspect individual results. Requires MAILERLITE_API_TOKEN.
use mailerlite_rs::{
    MailerLite,
    resources::batch::{BatchMethod, BatchRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let response = mailerlite
        .batch()
        .send(vec![
            BatchRequest::new(BatchMethod::Get, "api/groups?limit=25")?,
            BatchRequest::new(BatchMethod::Get, "api/fields?limit=25")?,
        ])
        .await?;

    for (index, item) in response.data.responses.iter().enumerate() {
        println!(
            "Operation {index}: HTTP {}, success={}",
            item.code,
            item.is_success()
        );
    }

    Ok(())
}
