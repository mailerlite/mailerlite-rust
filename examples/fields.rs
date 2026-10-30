//! Custom fields: list, create, rename and delete.
//!
//! ```sh
//! MAILERLITE_API_KEY=... cargo run --example fields
//! ```

use mailerlite_rs::{
    MailerLite, Sort,
    fields::{FieldSort, FieldType, ListFields},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY")?)?;
    let fields = client.fields();

    for field in fields
        .list(&ListFields::new().sort(Sort::asc(FieldSort::Name)))
        .await?
    {
        println!(
            "{} ({}) -> key `{}`",
            field.name, field.field_type, field.key
        );
    }

    let field = fields.create("Rust SDK score", FieldType::Number).await?;
    let field = fields.update(&field.id, "Rust SDK score (renamed)").await?;
    println!("created field `{}`", field.key);
    fields.delete(&field.id).await?;

    Ok(())
}
