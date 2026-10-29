//! The shared pagination types of the new list methods.

use serde::Deserialize;

/// A page of a list: the items, the page links and the page data.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Paginated<T> {
    #[serde(default = "Vec::new")]
    pub data: Vec<T>,
    pub links: Links,
    pub meta: Meta,
}

/// The links to the first, last, previous and next pages. `prev` and `next` are `None` when there is no
/// such page.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Links {
    pub first: Option<String>,
    pub last: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
}

/// The page data. `from` and `to` are `None` when the page is empty.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Meta {
    pub current_page: u64,
    pub from: Option<u64>,
    pub last_page: u64,
    #[serde(default)]
    pub links: Vec<MetaLink>,
    pub path: String,
    pub per_page: u64,
    pub to: Option<u64>,
    pub total: u64,
}

/// A link of the page list. `url` and `page` are `None` for a link that is not active, for example
/// "Previous" on the first page.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MetaLink {
    pub url: Option<String>,
    pub label: String,
    pub page: Option<u64>,
    pub active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_null_links_and_an_empty_page() {
        let page: Paginated<String> = serde_json::from_str(
            r#"{
                "data": [],
                "links": {
                    "first": "https://example.com/api/files?page=1",
                    "last": "https://example.com/api/files?page=1",
                    "prev": null,
                    "next": null
                },
                "meta": {
                    "current_page": 1,
                    "from": null,
                    "last_page": 1,
                    "links": [
                        {"url": null, "label": "&laquo; Previous", "page": null, "active": false},
                        {"url": "https://example.com/api/files?page=1", "label": "1", "page": 1, "active": true}
                    ],
                    "path": "https://example.com/api/files",
                    "per_page": 25,
                    "to": null,
                    "total": 0
                }
            }"#,
        )
        .unwrap();

        assert!(page.data.is_empty());
        assert_eq!(page.links.prev, None);
        assert_eq!(page.links.next, None);
        assert_eq!(page.meta.from, None);
        assert_eq!(page.meta.to, None);
        assert_eq!(page.meta.total, 0);
        assert_eq!(page.meta.links[0].url, None);
        assert_eq!(page.meta.links[0].page, None);
        assert_eq!(page.meta.links[1].page, Some(1));
        assert!(page.meta.links[1].active);
    }

    #[test]
    fn decodes_the_next_page_link() {
        let links: Links = serde_json::from_str(
            r#"{"first": "a", "last": "c", "prev": null, "next": "https://example.com/api/files?page=2"}"#,
        )
        .unwrap();

        assert_eq!(
            links.next.as_deref(),
            Some("https://example.com/api/files?page=2")
        );
    }
}
