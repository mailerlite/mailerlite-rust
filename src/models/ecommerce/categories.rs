//! Product category responses.
use super::wire;
use crate::models::CategoryId;
use serde::Deserialize;

/// Category returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
pub struct Category {
    /// Id.
    #[serde(deserialize_with = "wire::id")]
    pub id: CategoryId,
    /// Name.
    pub name: String,
    /// Resource id.
    pub resource_id: Option<String>,
    /// Exclude from automations.
    pub exclude_from_automations: Option<bool>,
    /// Created at.
    pub created_at: Option<String>,
    /// Updated at.
    pub updated_at: Option<String>,
}
