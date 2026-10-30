//! Inspect each operation in a batch independently.
//!
//! Requires `MAILERLITE_API_TOKEN`.
use mailerlite_rs::{
    MailerLite,
    models::{fields::Field, groups::Group},
    pagination::NumberedPage,
    resources::batch::{BatchMethod, BatchRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;
    let result = mailerlite
        .batch()
        .send(vec![
            BatchRequest::new(BatchMethod::Get, "api/groups?limit=25")?,
            BatchRequest::new(BatchMethod::Get, "api/fields?limit=25")?,
        ])
        .await?;
    for (index, item) in result.data.responses.iter().enumerate() {
        println!(
            "Operation {index}: HTTP {}, success={}",
            item.code,
            item.is_success()
        );
        if item.is_success() {
            // Batch results retain request order. Decode each body using the
            // endpoint's envelope, rather than assuming one universal schema.
            match index {
                0 => {
                    let groups: NumberedPage<Group> = item.decode()?;
                    println!("Groups on this page: {}", groups.data.len());
                }
                1 => {
                    let fields: NumberedPage<Field> = item.decode()?;
                    println!("Fields on this page: {}", fields.data.len());
                }
                _ => {}
            }
        }
        // Failed bodies remain available for explicit validation-error decoding.
        // Do not log arbitrary bodies: they may contain submitted sensitive data.
    }
    Ok(())
}
