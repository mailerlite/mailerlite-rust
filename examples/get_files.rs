use mailerlite_rs::{
    api_response::ApiResponse,
    file::{FileSort, FileType, ListFilesOptions, MediaFile},
    pagination::Paginated,
    MailerLite,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let options: ListFilesOptions = ListFilesOptions {
        name: Some(String::from("summer sale")),
        file_type: Some(FileType::Image),
        sort: Some(FileSort::SizeDesc),
        page: Some(1),
        limit: Some(25),
    };

    let response: ApiResponse<Paginated<MediaFile>> = mailerlite.file().get(options).await?;

    for file in &response.data {
        println!("{}: {}", file.name, file.url);
    }

    Ok(())
}
