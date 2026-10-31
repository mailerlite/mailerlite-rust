//! Product category responses.
use super::wire;
use crate::models::CategoryId;
use serde::Deserialize;

/// Category returned by the e-commerce API.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Category {
    /// MailerLite identifier, preserved as text.
    #[serde(deserialize_with = "wire::id")]
    pub id: CategoryId,
    /// Display name assigned to this resource.
    pub name: String,
    /// Identifier assigned by the connected shop, distinct from the MailerLite ID.
    pub resource_id: Option<String>,
    /// Whether this resource is excluded from automation processing.
    pub exclude_from_automations: Option<bool>,
    /// Creation timestamp as returned by the server; no timezone is inferred.
    pub created_at: Option<String>,
    /// Last modification timestamp as returned by the server.
    pub updated_at: Option<String>,
}
