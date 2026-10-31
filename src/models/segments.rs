//! Segment response models.
use crate::models::{SegmentId, groups::Rate};
use serde::Deserialize;
/// A dynamic subscriber segment.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Segment {
    /// Segment identifier.
    pub id: SegmentId,
    /// Segment name.
    pub name: String,
    /// Matching subscribers.
    pub total: Option<u64>,
    /// Open rate.
    pub open_rate: Option<Rate>,
    /// Click rate.
    pub click_rate: Option<Rate>,
    /// Creation timestamp without an assumed timezone.
    pub created_at: Option<String>,
}
