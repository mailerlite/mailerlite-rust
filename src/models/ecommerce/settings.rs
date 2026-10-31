//! E-commerce tracking and attribution settings.
use serde::Deserialize;

/// Settings returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Settings {
    /// Ecommerce tracking in campaigns.
    pub ecommerce_tracking_in_campaigns: bool,
    /// Ecommerce tracking in automations.
    pub ecommerce_tracking_in_automations: bool,
    /// Ecommerce attribution window.
    pub ecommerce_attribution_window: u64,
}
