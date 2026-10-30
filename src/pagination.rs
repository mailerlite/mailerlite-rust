//! Page- and cursor-based list responses.
//!
//! MailerLite uses two pagination styles:
//!
//! * **cursor** ([`CursorPage`]): subscriber listings (`/subscribers`,
//!   `/groups/{id}/subscribers`, `/segments/{id}/subscribers`, `/forms/{id}/subscribers`).
//! * **page number** ([`Page`]): everything else.
//!
//! With the `stream` feature (on by default) list handles also expose `list_all`, which walks
//! every page lazily and yields items as a [`futures_util::Stream`].

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// One page of a page-number paginated list.
///
/// `A` is the type of the optional `aggregations` object some endpoints attach (e.g. campaign
/// counts per status). It defaults to raw JSON.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(
    from = "RawPage<T, A>",
    bound(deserialize = "T: Deserialize<'de>, A: Deserialize<'de>")
)]
#[non_exhaustive]
pub struct Page<T, A = Value> {
    /// Items on this page.
    pub data: Vec<T>,
    /// Pagination metadata.
    pub meta: PageMeta,
    /// Navigation links.
    pub links: Links,
    /// Endpoint-specific aggregate counters, if the endpoint returns any.
    pub aggregations: Option<A>,
}

impl<T, A> Page<T, A> {
    /// Next page number, or `None` on the last page.
    #[must_use]
    pub fn next_page(&self) -> Option<u32> {
        if self.data.is_empty() || self.meta.current_page >= self.meta.last_page {
            None
        } else {
            Some(self.meta.current_page + 1)
        }
    }

    /// `true` if there is a page after this one.
    #[must_use]
    pub fn has_more(&self) -> bool {
        self.next_page().is_some()
    }
}

impl<T, A> IntoIterator for Page<T, A> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/// Metadata of a page-number paginated list.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PageMeta {
    /// 1-based index of this page.
    #[serde(default = "one")]
    pub current_page: u32,
    /// Index of the last page.
    #[serde(default = "one")]
    pub last_page: u32,
    /// Page size.
    #[serde(default)]
    pub per_page: u32,
    /// Total number of items across all pages.
    #[serde(default)]
    pub total: u64,
    /// 1-based position of the first item on this page.
    #[serde(default)]
    pub from: Option<u64>,
    /// 1-based position of the last item on this page.
    #[serde(default)]
    pub to: Option<u64>,
    /// Endpoint path.
    #[serde(default)]
    pub path: Option<String>,
}

const fn one() -> u32 {
    1
}

/// `first` / `last` / `prev` / `next` URLs of a list.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Links {
    /// First page.
    #[serde(default)]
    pub first: Option<String>,
    /// Last page.
    #[serde(default)]
    pub last: Option<String>,
    /// Previous page.
    #[serde(default)]
    pub prev: Option<String>,
    /// Next page.
    #[serde(default)]
    pub next: Option<String>,
}

/// One page of a cursor paginated list.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CursorPage<T> {
    /// Items on this page.
    pub data: Vec<T>,
    /// Cursor metadata.
    #[serde(default)]
    pub meta: CursorMeta,
    /// Navigation links.
    #[serde(default)]
    pub links: Links,
}

impl<T> CursorPage<T> {
    /// Cursor for the next page, or `None` on the last page.
    #[must_use]
    pub fn next_cursor(&self) -> Option<&str> {
        if self.data.is_empty() {
            return None;
        }
        self.meta
            .next_cursor
            .as_deref()
            .filter(|cursor| !cursor.is_empty())
    }

    /// `true` if there is a page after this one.
    #[must_use]
    pub fn has_more(&self) -> bool {
        self.next_cursor().is_some()
    }
}

impl<T> IntoIterator for CursorPage<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/// Metadata of a cursor paginated list.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CursorMeta {
    /// Page size.
    #[serde(default)]
    pub per_page: u32,
    /// Pass as `cursor` to fetch the next page.
    #[serde(default)]
    pub next_cursor: Option<String>,
    /// Pass as `cursor` to fetch the previous page.
    #[serde(default)]
    pub prev_cursor: Option<String>,
    /// Endpoint path.
    #[serde(default)]
    pub path: Option<String>,
}

/// Wire shape of [`Page`]: `aggregations` lives in `meta` for most endpoints but at the top
/// level for automations, and `meta`/`links` are missing on a few lists.
#[derive(Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>, A: Deserialize<'de>"))]
struct RawPage<T, A> {
    data: Vec<T>,
    #[serde(default = "RawMeta::empty")]
    meta: RawMeta<A>,
    #[serde(default)]
    links: Links,
    #[serde(default)]
    aggregations: Option<A>,
}

#[derive(Deserialize)]
#[serde(bound(deserialize = "A: Deserialize<'de>"))]
struct RawMeta<A> {
    #[serde(default = "one")]
    current_page: u32,
    #[serde(default = "one")]
    last_page: u32,
    #[serde(default)]
    per_page: u32,
    #[serde(default)]
    total: u64,
    #[serde(default)]
    from: Option<u64>,
    #[serde(default)]
    to: Option<u64>,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    aggregations: Option<A>,
}

impl<A> RawMeta<A> {
    fn empty() -> Self {
        Self {
            current_page: 1,
            last_page: 1,
            per_page: 0,
            total: 0,
            from: None,
            to: None,
            path: None,
            aggregations: None,
        }
    }
}

impl<T, A> From<RawPage<T, A>> for Page<T, A> {
    fn from(raw: RawPage<T, A>) -> Self {
        let RawMeta {
            current_page,
            last_page,
            per_page,
            total,
            from,
            to,
            path,
            aggregations,
        } = raw.meta;

        Self {
            data: raw.data,
            meta: PageMeta {
                current_page,
                last_page,
                per_page,
                total,
                from,
                to,
                path,
            },
            links: raw.links,
            aggregations: raw.aggregations.or(aggregations),
        }
    }
}

#[cfg(feature = "stream")]
pub(crate) mod stream {
    use std::future::Future;

    use futures_util::{Stream, TryStreamExt, stream};

    use super::{CursorPage, Page};
    use crate::Result;

    /// Lazily walks a page-number paginated list, starting at `first_page`.
    pub(crate) fn pages<T, A, F, Fut>(
        first_page: u32,
        mut fetch: F,
    ) -> impl Stream<Item = Result<T>>
    where
        F: FnMut(u32) -> Fut,
        Fut: Future<Output = Result<Page<T, A>>>,
    {
        stream::try_unfold(Some(first_page), move |next| {
            let request = next.map(&mut fetch);
            async move {
                let Some(request) = request else {
                    return Ok::<_, crate::Error>(None);
                };
                let page = request.await?;
                let next = page.next_page();
                Ok(Some((stream::iter(page.data.into_iter().map(Ok)), next)))
            }
        })
        .try_flatten()
    }

    /// Lazily walks a cursor paginated list.
    pub(crate) fn cursor<T, F, Fut>(
        first_cursor: Option<String>,
        mut fetch: F,
    ) -> impl Stream<Item = Result<T>>
    where
        F: FnMut(Option<String>) -> Fut,
        Fut: Future<Output = Result<CursorPage<T>>>,
    {
        // `None` = done; `Some(cursor)` = fetch with that cursor.
        stream::try_unfold(Some(first_cursor), move |next| {
            let request = next.map(&mut fetch);
            async move {
                let Some(request) = request else {
                    return Ok::<_, crate::Error>(None);
                };
                let page = request.await?;
                let next = page.next_cursor().map(|cursor| Some(cursor.to_owned()));
                Ok(Some((stream::iter(page.data.into_iter().map(Ok)), next)))
            }
        })
        .try_flatten()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn page_reads_meta_aggregations() {
        let page: Page<u32> = serde_json::from_value(json!({
            "data": [1, 2],
            "links": {"first": "f", "last": "l", "prev": null, "next": "n"},
            "meta": {
                "current_page": 1, "from": 1, "last_page": 3, "per_page": 2, "to": 2, "total": 6,
                "path": "https://connect.mailerlite.com/api/campaigns",
                "links": [{"url": null, "label": "&laquo; Previous", "active": false}],
                "aggregations": {"sum": 6}
            }
        }))
        .unwrap();

        assert_eq!(page.next_page(), Some(2));
        assert_eq!(page.meta.total, 6);
        assert_eq!(page.links.next.as_deref(), Some("n"));
        assert_eq!(page.aggregations, Some(json!({"sum": 6})));
    }

    #[test]
    fn page_reads_top_level_aggregations_and_tolerates_missing_meta() {
        let page: Page<u32> = serde_json::from_value(json!({
            "data": [1],
            "meta": {"current_page": 2, "last_page": 2},
            "aggregations": {"sum": 24}
        }))
        .unwrap();
        assert_eq!(page.aggregations, Some(json!({"sum": 24})));
        assert_eq!(page.next_page(), None);

        let bare: Page<u32> = serde_json::from_value(json!({"data": [1, 2]})).unwrap();
        assert_eq!(bare.next_page(), None);
        assert_eq!(bare.into_iter().collect::<Vec<_>>(), [1, 2]);
    }

    #[test]
    fn cursor_page_next_cursor() {
        let page: CursorPage<u32> = serde_json::from_value(json!({
            "data": [1],
            "links": {"first": null, "last": null, "prev": null, "next": "https://x?cursor=abc"},
            "meta": {"path": "p", "per_page": 25, "next_cursor": "abc", "prev_cursor": null}
        }))
        .unwrap();
        assert_eq!(page.next_cursor(), Some("abc"));

        let last: CursorPage<u32> =
            serde_json::from_value(json!({"data": [1], "meta": {"next_cursor": null}})).unwrap();
        assert!(!last.has_more());
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn streams_walk_every_page() {
        use futures_util::TryStreamExt;

        let pages = stream::pages(1, |page| async move {
            Ok::<_, crate::Error>(
                serde_json::from_value::<Page<u32>>(json!({
                    "data": [page * 10, page * 10 + 1],
                    "meta": {"current_page": page, "last_page": 3}
                }))
                .unwrap(),
            )
        });
        let items: Vec<u32> = pages.try_collect().await.unwrap();
        assert_eq!(items, [10, 11, 20, 21, 30, 31]);

        let cursors = stream::cursor(None, |cursor| async move {
            let (data, next) = match cursor.as_deref() {
                None => (vec![1, 2], Some("b")),
                Some("b") => (vec![3], None),
                Some(other) => panic!("unexpected cursor {other}"),
            };
            Ok::<_, crate::Error>(
                serde_json::from_value::<CursorPage<u32>>(
                    json!({"data": data, "meta": {"next_cursor": next}}),
                )
                .unwrap(),
            )
        });
        let items: Vec<u32> = cursors.try_collect().await.unwrap();
        assert_eq!(items, [1, 2, 3]);
    }
}
