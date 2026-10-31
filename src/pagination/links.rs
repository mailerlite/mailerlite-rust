//! Navigation links retained as untrusted response metadata.
use serde::Deserialize;

/// Navigation links returned by the API; these are untrusted response data.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PageLinks {
    /// Link to the first page.
    pub first: Option<String>,
    /// Link to the last page.
    pub last: Option<String>,
    /// Link to the preceding page.
    pub prev: Option<String>,
    /// Link to the following page.
    pub next: Option<String>,
}
