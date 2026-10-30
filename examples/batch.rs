//! Batch: up to 50 API calls in one request; each item succeeds or fails on its own.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example batch
//! ```

use mailerlite_rs::{
    MailerLite, batch::BatchRequest, groups::Group, subscribers::CreateSubscriber,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;

    let request = BatchRequest::new()
        .get("api/groups?limit=1")
        .get("api/subscribers/does-not-exist@example.com")
        .post(
            "api/subscribers",
            &CreateSubscriber::new("rust-sdk-batch@example.com"),
        )?;

    let response = client.batch().send(&request).await?;
    println!("{} ok, {} failed", response.successful, response.failed);

    for (index, result) in response.responses.iter().enumerate() {
        if result.is_success() {
            println!("#{index}: {}", result.code);
        } else {
            println!(
                "#{index}: {} {}",
                result.code,
                result.message().unwrap_or_default()
            );
        }
    }

    if let Some(first) = response.responses.first() {
        let groups: Vec<Group> = first.decode()?;
        println!("first group: {:?}", groups.first().map(|group| &group.name));
    }

    Ok(())
}
