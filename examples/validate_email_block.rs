//! Validate email block.
//!
//! Requires `MAILERLITE_API_TOKEN`.
use mailerlite_rs::{
    MailerLite, Response,
    models::common::Patch,
    models::emails::{
        BlockContent, BlockData, ButtonAttributes, EmailResponseMetadata, Validation,
    },
    resources::emails::NewBlock,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = std::env::var("MAILERLITE_API_TOKEN")?;

    let mailerlite: MailerLite = MailerLite::new(api_key)?;

    let content = BlockData {
        attrs: Patch::Value(ButtonAttributes {
            url: Patch::Value("https://example.com".into()),
            ..Default::default()
        }),
        ..BlockData::text("Buy now")
    };
    let block = NewBlock::new(BlockContent::Button(content));

    let response: Response<Validation, EmailResponseMetadata> =
        mailerlite.emails().validate_block(&block).await?;

    if !response.data.valid {
        println!("{:#?}", response.data.errors);
    }

    Ok(())
}
