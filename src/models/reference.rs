//! Reference data for scheduling and localization.
/// Timezone returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct Timezone {
    /// Id.
    pub id: String,
    /// Name.
    pub name: String,
    /// Name for humans.
    pub name_for_humans: String,
    /// Offset name.
    pub offset_name: String,
    /// Offset.
    pub offset: i32,
}
/// CampaignLanguage returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct CampaignLanguage {
    /// Id.
    pub id: String,
    /// Shortcode.
    pub shortcode: String,
    /// Iso639.
    pub iso639: String,
    /// Name.
    pub name: String,
    /// Direction.
    pub direction: String,
}
