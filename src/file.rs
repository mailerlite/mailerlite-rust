//! The files of the media library of the account.

use crate::{
    api_response::ApiResponse,
    error::Error,
    http::{self, Method, NO_BODY},
    pagination::Paginated,
    validate, MailerLite,
};
use serde::Deserialize;

const END_POINT: &str = "files";

open_enum! {
    /// The type of a file.
    pub enum FileType {
        Image => "image",
        Pdf => "pdf",
        Epub => "epub",
    }
}

open_enum! {
    /// The sort order of a file list. The default of the API is `CreatedAtDesc`, newest first.
    pub enum FileSort {
        CreatedAt => "created_at",
        CreatedAtDesc => "-created_at",
        Name => "name",
        NameDesc => "-name",
        Size => "size",
        SizeDesc => "-size",
    }
}

/// A file of the media library.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MediaFile {
    pub id: String,
    #[serde(rename = "type")]
    pub file_type: FileType,
    pub name: String,
    pub url: String,
    /// The size in bytes.
    pub size: u64,
    /// The width in pixels. `None` for a file that is not an image.
    pub width: Option<u32>,
    /// The height in pixels. `None` for a file that is not an image.
    pub height: Option<u32>,
    pub created_at: String,
}

/// The filters and the page options of `File::get`. The SDK does not send `None` values. It sends an
/// empty string as an empty value.
#[derive(Debug, Clone, Default)]
pub struct ListFilesOptions {
    /// Gets only the files with a name that contains this text.
    pub name: Option<String>,
    pub file_type: Option<FileType>,
    pub sort: Option<FileSort>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// The file methods. Get it with `MailerLite::file`.
#[derive(Debug)]
pub struct File {
    mailerlite: MailerLite,
}

impl File {
    pub fn new(mailerlite: MailerLite) -> Self {
        Self { mailerlite }
    }

    /// Gets a list of the files of the media library.
    pub async fn get(
        &self,
        options: ListFilesOptions,
    ) -> Result<ApiResponse<Paginated<MediaFile>>, Error> {
        let query: Vec<(&str, String)> = validate::query_pairs([
            ("filter[name]", options.name),
            ("filter[type]", options.file_type.map(String::from)),
            ("sort", options.sort.map(String::from)),
            ("page", options.page.map(|page| page.to_string())),
            ("limit", options.limit.map(|limit| limit.to_string())),
        ]);

        http::send(
            &self.mailerlite,
            Method::GET,
            END_POINT,
            &query,
            NO_BODY,
            None,
        )
        .await
    }
}
