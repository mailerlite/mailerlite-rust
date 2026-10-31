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
    ///
    /// `None` means no continuation was supplied, even if `links.next` is set.
    /// Pass the cursor unchanged to the same resource's list builder, preserving
    /// its filters. This accessor makes no request and does not detect cycles;
    /// automatic streams perform cycle detection while fetching pages.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::pagination::CursorPage;
    ///
    /// # fn main() -> Result<(), serde_json::Error> {
    /// let page: CursorPage<u64> = serde_json::from_str(
    ///     r#"{"data":[1],"meta":{"next_cursor":"opaque+/="}}"#,
    /// )?;
    ///
    /// assert_eq!(page.next_cursor(), Some("opaque+/="));
    /// # Ok(())
    /// # }
    /// ```
    pub fn next_cursor(&self) -> Option<&str> {
        self.meta.next_cursor.as_deref()
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
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
