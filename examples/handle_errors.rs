//! Inspect errors without logging raw response bodies.
//!
//! Requires `MAILERLITE_API_TOKEN`, `MAILERLITE_SUBSCRIBER_ID`.
use mailerlite_rs::{Error, MailerLite};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_SUBSCRIBER_ID")?;
    match mailerlite.subscribers().get(id).await {
        Ok(response) => println!("{}", response.data.id),
        Err(Error::Api(error)) => {
            eprintln!("HTTP {}", error.status());
            if error.is_validation() {
                for field in error.validation_errors().keys() {
                    eprintln!("Invalid field: {field}");
                }
            } else if error.is_unauthorized() {
                eprintln!("Check the configured API token.");
            } else if error.is_rate_limited() {
                eprintln!("Retry delay: {:?}", error.metadata().retry_after());
            } else if error.is_server_error() {
                eprintln!("MailerLite reported a server error.");
            }
        }
        Err(Error::Decode(error)) => eprintln!("Unexpected response at HTTP {}", error.status()),
        Err(error) => return Err(error.into()),
    }
    Ok(())
}
