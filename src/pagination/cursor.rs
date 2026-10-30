//! Cursor-based page models and continuation metadata.
use super::PageLinks;
use serde::Deserialize;

/// Metadata for a cursor-paginated endpoint.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CursorMeta {
    /// Requested page size, when reported.
    pub per_page: Option<u64>,
    /// Opaque cursor for the following page.
    pub next_cursor: Option<String>,
    /// Opaque cursor for the preceding page.
    pub prev_cursor: Option<String>,
}

/// A page from an endpoint that uses opaque cursors.
#[derive(Debug, Clone, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct CursorPage<T> {
    /// Resources in this page.
    pub data: Vec<T>,
    /// Navigation links, retained but never fetched directly.
    #[serde(default)]
    pub links: PageLinks,
    /// Cursor metadata. Missing metadata means no known continuation.
    #[serde(default)]
    pub meta: CursorMeta,
}

impl<T> CursorPage<T> {
    /// Return the next cursor without interpreting its contents.
    pub fn next_cursor(&self) -> Option<&str> {
        self.meta.next_cursor.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_metadata_preserves_opaque_values() {
        let page: CursorPage<u32> = serde_json::from_str(
            r#"{"data":[],"meta":{"next_cursor":"a+/=%?"},"links":{"next":"https://unexpected.example/"}}"#,
        ).unwrap();
        assert_eq!(page.next_cursor(), Some("a+/=%?"));
    }
}
