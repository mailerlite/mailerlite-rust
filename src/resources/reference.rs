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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.reference().timezones().await?;
    ///
    /// println!("{} timezones", response.data.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Server rejection returns [`crate::Error::Api`]. HTTP execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.reference().campaign_languages().await?;
    ///
    /// println!("{} languages", response.data.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Server rejection returns [`crate::Error::Api`]. HTTP execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
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
