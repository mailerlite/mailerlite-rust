//! Group response models.
use crate::models::GroupId;
use serde::Deserialize;

pub use crate::models::common::Rate;

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
    /// Email open rate in the server-provided rate representation.
    pub open_rate: Option<Rate>,
    /// Clicks.
    pub clicks_count: Option<u64>,
    /// Email click rate in the server-provided rate representation.
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
