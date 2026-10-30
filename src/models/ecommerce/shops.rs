//! E-commerce shop responses.
use super::wire;
use crate::models::ShopId;
use serde::Deserialize;

/// Shop returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct Shop {
    /// Id.
    #[serde(deserialize_with = "wire::id")]
    pub id: ShopId,
    /// Name.
    pub name: String,
    /// Url.
    pub url: Option<String>,
    /// Currency.
    pub currency: Option<String>,
    /// Enabled.
    pub enabled: Option<bool>,
    /// Enable popups.
    pub enable_popups: Option<bool>,
    /// Platform.
    pub platform: Option<String>,
    /// Internal.
    pub internal: Option<bool>,
    /// Group.
    pub group: Option<crate::models::groups::Group>,
    /// Created at.
    pub created_at: Option<String>,
    /// Updated at.
    pub updated_at: Option<String>,
    /// Currency sign.
    pub currency_sign: Option<String>,
}
