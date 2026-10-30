//! Settings: <https://developers.mailerlite.com/ecommerce/settings>

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{MailerLite, Result, types::de};

/// Account defaults for e-commerce tracking, copied to each campaign or automation when it
/// is created.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EcommerceSettings {
    /// Whether new campaigns track e-commerce.
    #[serde(default, deserialize_with = "de::null_default")]
    pub ecommerce_tracking_in_campaigns: bool,
    /// Whether new automations track e-commerce. Always `false` while no shop is enabled.
    #[serde(default, deserialize_with = "de::null_default")]
    pub ecommerce_tracking_in_automations: bool,
    /// Days after a click during which a purchase is attributed to the email.
    #[serde(default)]
    pub ecommerce_attribution_window: u32,
}

/// E-commerce settings endpoint. Obtained with
/// [`Ecommerce::settings`](super::Ecommerce::settings).
#[derive(Clone, Copy, Debug)]
pub struct Settings<'a> {
    pub(super) client: &'a MailerLite,
}

impl Settings<'_> {
    /// `GET /ecommerce/settings`: the account's e-commerce tracking defaults (read-only).
    pub async fn get(&self) -> Result<EcommerceSettings> {
        self.client
            .call(Method::GET, &["ecommerce", "settings"])
            .data()
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;

    use crate::http::mock::{client, fixture};

    #[tokio::test]
    async fn get() {
        let (client, mock) = client();
        mock.respond(200, fixture("ecommerce_settings"));

        let settings = client.ecommerce().settings().get().await.unwrap();

        mock.request()
            .assert(Method::GET, "ecommerce/settings")
            .no_body();
        assert!(!settings.ecommerce_tracking_in_campaigns);
        assert!(settings.ecommerce_tracking_in_automations);
        assert_eq!(settings.ecommerce_attribution_window, 30);
    }
}
