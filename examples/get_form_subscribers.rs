//! List subscribers who signed up using a form.
//!
//! Required environment variables: `MAILERLITE_API_TOKEN`, `MAILERLITE_FORM_ID`.
use mailerlite_rs::MailerLite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let id = std::env::var("MAILERLITE_FORM_ID")?;
    let response = mailerlite.forms().subscribers(id).limit(25).send().await?;
    println!("Next cursor: {:?}", response.data.next_cursor());
    for subscriber in response.data.data {
        println!("{}: {}", subscriber.id, subscriber.email);
    }
    Ok(())
}
