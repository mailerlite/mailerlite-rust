//! Internal operation classification and request serialization.
use crate::{MailerLite, Result};

/// Retry and local rate-limit policy attached to an operation.
#[derive(Debug, Clone, Copy)]
pub(crate) struct OperationPolicy {
    pub read_only: bool,
    pub import: bool,
    pub success_statuses: &'static [u16],
}
impl OperationPolicy {
    /// Refine accepted statuses for an endpoint with a distinct response contract.
    pub const fn with_success_statuses(mut self, statuses: &'static [u16]) -> Self {
        self.success_statuses = statuses;
        self
    }
    /// Explicitly reviewed read operation.
    pub const READ: Self = Self {
        read_only: true,
        import: false,
        success_statuses: &[200],
    };
    /// Write operation that must never be replayed automatically.
    pub const WRITE: Self = Self {
        read_only: false,
        import: false,
        success_statuses: &[200, 201, 204],
    };
    /// Subscriber import operation, subject to both local pacing buckets.
    pub const IMPORT: Self = Self {
        read_only: false,
        import: true,
        success_statuses: &[200, 202],
    };
}

/// Serialize a typed request before dispatch.
pub(crate) fn body<T: serde::Serialize>(value: &T) -> Result<serde_json::Value> {
    MailerLite::body(value)
}
