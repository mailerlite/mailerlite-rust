//! Group response models.
use crate::models::GroupId;
use serde::Deserialize;
/// Numeric and human-readable percentage supplied together by MailerLite.
#[derive(Debug, Clone, Deserialize)]
pub struct Rate {
    /// Numeric percentage.
    pub float: f64,
    /// Formatted percentage.
    pub string: String,
}
/// A subscriber group.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Group {
    /// Group identifier.
    pub id: GroupId,
    /// Group name.
    pub name: String,
    /// Active subscribers.
    pub active_count: Option<u64>,
    /// Number sent.
    pub sent_count: Option<u64>,
    /// Opens.
    pub opens_count: Option<u64>,
    /// Open rate.
    pub open_rate: Option<Rate>,
    /// Clicks.
    pub clicks_count: Option<u64>,
    /// Click rate.
    pub click_rate: Option<Rate>,
    /// Unsubscribed members.
    pub unsubscribed_count: Option<u64>,
    /// Unconfirmed members.
    pub unconfirmed_count: Option<u64>,
    /// Bounced members.
    pub bounced_count: Option<u64>,
    /// Junk members.
    pub junk_count: Option<u64>,
    /// Creation timestamp without an assumed timezone.
    pub created_at: Option<String>,
    /// Whether the group is processing.
    pub processing: Option<bool>,
    /// Whether referenced by forms.
    pub used_in_forms: Option<bool>,
    /// Whether referenced by sites.
    pub used_in_sites: Option<bool>,
}
