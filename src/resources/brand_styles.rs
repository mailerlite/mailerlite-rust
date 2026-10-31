//! Account brand-style retrieval.
use crate::{
    MailerLite, Response, Result, http::OperationPolicy, models::brand_styles::BrandStyles,
};
/// Borrowed account brand-style operations.
#[derive(Debug, Clone, Copy)]
pub struct BrandStylesResource<'a> {
    client: &'a MailerLite,
}
impl<'a> BrandStylesResource<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// Retrieve account, default, and preset palettes.
    ///
    /// # Errors
    /// Returns shared configuration, transport, API, decoding, or timeout errors.
    pub async fn get(&self) -> Result<Response<BrandStyles>> {
        self.client
            .data(
                reqwest::Method::GET,
                &["brand-styles"],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
}
