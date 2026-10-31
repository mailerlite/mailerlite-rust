//! Reference data for scheduling and localization.
/// Timezone returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct Timezone {
    /// MailerLite identifier, preserved as text.
    pub id: String,
    /// Timezone name used for scheduling, such as an IANA identifier.
    pub name: String,
    /// Human-readable timezone label supplied by the server.
    pub name_for_humans: String,
    /// Display label for the UTC offset.
    pub offset_name: String,
    /// UTC offset supplied by the server, in minutes.
    pub offset: i32,
}
/// CampaignLanguage returned by MailerLite.
#[derive(Debug, Clone, serde::Deserialize)]
#[non_exhaustive]
pub struct CampaignLanguage {
    /// MailerLite identifier, preserved as text.
    pub id: String,
    /// MailerLite language shortcode used by campaign settings.
    pub shortcode: String,
    /// ISO 639 language code supplied by the server.
    pub iso639: String,
    /// Human-readable language name.
    pub name: String,
    /// Text direction reported for the language, such as `ltr` or `rtl`.
    pub direction: String,
}
