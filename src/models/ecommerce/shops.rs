//! E-commerce shop responses.
use super::wire;
use crate::models::ShopId;
use serde::Deserialize;

/// Shop returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Shop {
    /// MailerLite identifier, preserved as text.
    #[serde(deserialize_with = "wire::id")]
    pub id: ShopId,
    /// Display name assigned to this resource.
    pub name: String,
    /// URL of the connected shop.
    pub url: Option<String>,
    /// Currency identifier reported for this resource.
    pub currency: Option<String>,
    /// Whether this resource is enabled on the server.
    pub enabled: Option<bool>,
    /// Whether MailerLite popups are enabled for this shop.
    pub enable_popups: Option<bool>,
    /// Connected commerce platform identifier, preserved as text.
    pub platform: Option<String>,
    /// Whether this is the internal MailerLite shop rather than an external shop.
    pub internal: Option<bool>,
    /// Subscriber group associated with the shop, when included.
    pub group: Option<crate::models::groups::Group>,
    /// Creation timestamp as returned by the server; no timezone is inferred.
    pub created_at: Option<String>,
    /// Last modification timestamp as returned by the server.
    pub updated_at: Option<String>,
    /// Display symbol for the shop currency.
    pub currency_sign: Option<String>,
}
