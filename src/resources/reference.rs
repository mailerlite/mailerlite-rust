//! Reference data used by campaign configuration.
use crate::{
    MailerLite, Response, Result,
    http::OperationPolicy,
    models::reference::{CampaignLanguage, Timezone},
};
use reqwest::Method;
/// Reference-data resource handle.
#[derive(Debug, Clone, Copy)]
pub struct Reference<'a> {
    client: &'a MailerLite,
}
impl<'a> Reference<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List available timezones.
    pub async fn timezones(&self) -> Result<Response<Vec<Timezone>>> {
        self.client
            .data(
                Method::GET,
                &["timezones"],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// List campaign localization languages.
    pub async fn campaign_languages(&self) -> Result<Response<Vec<CampaignLanguage>>> {
        self.client
            .data(
                Method::GET,
                &["campaigns", "languages"],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
}
