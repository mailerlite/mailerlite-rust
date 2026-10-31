//! Media-library file types and metadata.
use crate::models::open_enum::open_enum;
use serde::Deserialize;
open_enum! {
    pub enum FileType {
        Image => "image",
        Pdf => "pdf",
        Epub => "epub",
    }
}

open_enum! {
    /// The API default is `CreatedAtDesc`, newest first.
    pub enum FileSort {
        CreatedAt => "created_at",
        CreatedAtDesc => "-created_at",
        Name => "name",
        NameDesc => "-name",
        Size => "size",
        SizeDesc => "-size",
    }
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
/// Stored media metadata returned by the account file library.
pub struct MediaFile {
    /// Opaque identifier, preserved as returned by the server.
    pub id: String,
    #[serde(rename = "type")]
    /// Media-library file format discriminator.
    pub file_type: FileType,
    /// Display name assigned by the API.
    pub name: String,
    /// URL returned by or submitted to the API.
    pub url: String,
    /// The size in bytes.
    pub size: u64,
    /// The width in pixels. `None` for a file that is not an image.
    pub width: Option<u32>,
    /// The height in pixels. `None` for a file that is not an image.
    pub height: Option<u32>,
    /// Server timestamp retained without assuming a timezone.
    pub created_at: String,
}
