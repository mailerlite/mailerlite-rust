//! Media-library listing with typed filters and numbered pages.
use crate::{
    Error, MailerLite, Response,
    http::OperationPolicy,
    models::files::{FileSort, FileType, MediaFile},
    pagination::NumberedPage,
};
use reqwest::Method;
/// The filters and the page options of `Files::list`. The SDK does not send `None` values. It sends an
/// empty string as an empty value.
#[derive(Debug, Clone, Default)]
pub struct ListFilesOptions {
    /// Gets only the files with a name that contains this text.
    pub name: Option<String>,
    /// Media-library file format discriminator.
    pub file_type: Option<FileType>,
    /// Ordering requested from the media library.
    pub sort: Option<FileSort>,
    /// One-based page number; omission uses the server default.
    pub page: Option<u32>,
    /// Requested page size; omission uses the server default.
    pub limit: Option<u32>,
}

#[derive(Debug)]
/// Borrowed media-library operations sharing the client transport.
pub struct Files<'a> {
    mailerlite: &'a MailerLite,
}

impl<'a> Files<'a> {
    pub(crate) fn new(mailerlite: &'a MailerLite) -> Self {
        Self { mailerlite }
    }

    /// Construct a lazy media-library listing request.
    pub fn list(&self) -> ListFiles<'a> {
        ListFiles {
            mailerlite: self.mailerlite,
            options: ListFilesOptions::default(),
        }
    }
}
/// Lazy file listing; filters are encoded only when the request is sent.
#[derive(Debug)]
#[must_use = "request builders do nothing until sent; call .send().await"]
pub struct ListFiles<'a> {
    mailerlite: &'a MailerLite,
    options: ListFilesOptions,
}
impl ListFiles<'_> {
    /// Replace all filters and page options.
    pub fn options(mut self, options: ListFilesOptions) -> Self {
        self.options = options;
        self
    }
    /// Filter by a substring of the file name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.options.name = Some(name.into());
        self
    }
    /// Filter by media format.
    pub fn kind(mut self, kind: FileType) -> Self {
        self.options.file_type = Some(kind);
        self
    }
    /// Select the listing order.
    pub fn sort(mut self, sort: FileSort) -> Self {
        self.options.sort = Some(sort);
        self
    }
    /// Select a one-based page.
    pub fn page(mut self, page: u32) -> Self {
        self.options.page = Some(page);
        self
    }
    /// Set the requested page size.
    pub fn limit(mut self, limit: u32) -> Self {
        self.options.limit = Some(limit);
        self
    }
    /// Fetch one numbered page of media-library files.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors.
    pub async fn send(self) -> Result<Response<NumberedPage<MediaFile>>, Error> {
        let options = self.options;
        let query: Vec<(&str, String)> = query_pairs([
            ("filter[name]", options.name),
            ("filter[type]", options.file_type.map(String::from)),
            ("sort", options.sort.map(String::from)),
            ("page", options.page.map(|page| page.to_string())),
            ("limit", options.limit.map(|limit| limit.to_string())),
        ]);

        self.mailerlite
            .request(
                Method::GET,
                &["files"],
                &query
                    .into_iter()
                    .map(|(k, v)| (k.to_owned(), v))
                    .collect::<Vec<_>>(),
                None,
                OperationPolicy::READ,
            )
            .await
    }
}

/// An empty string stays, and the SDK sends it as `key=`.
pub(crate) fn query_pairs<'a>(
    pairs: impl IntoIterator<Item = (&'a str, Option<String>)>,
) -> Vec<(&'a str, String)> {
    pairs
        .into_iter()
        .filter_map(|(key, value)| value.map(|value| (key, value)))
        .collect()
}
