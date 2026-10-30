//! Segments: <https://developers.mailerlite.com/api/segments>
//!
//! Segments are created in the MailerLite app; the API can list, rename and delete them and
//! list their subscribers.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    CursorPage, MailerLite, Page, Result,
    subscribers::{ListSubscribers, Subscriber},
    types::{Id, NameBody, Rate, Timestamp},
};

/// A subscriber segment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Segment {
    /// Segment id.
    pub id: Id,
    /// Segment name.
    pub name: String,
    /// Number of subscribers in the segment.
    #[serde(default)]
    pub total: u64,
    /// Open rate.
    #[serde(default)]
    pub open_rate: Rate,
    /// Click rate.
    #[serde(default)]
    pub click_rate: Rate,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
}

/// Query for [`Segments::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListSegments {
    /// Page size (an account has at most 250 segments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl ListSegments {
    /// Default query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Page size.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// 1-based page number.
    #[must_use]
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

/// Segment endpoints. Obtained with [`MailerLite::segments`].
#[derive(Clone, Copy, Debug)]
pub struct Segments<'a> {
    client: &'a MailerLite,
}

impl MailerLite {
    /// Segment endpoints.
    #[must_use]
    pub fn segments(&self) -> Segments<'_> {
        Segments { client: self }
    }
}

impl<'a> Segments<'a> {
    /// `GET /segments`: one page of segments.
    pub async fn list(self, query: &ListSegments) -> Result<Page<Segment>> {
        self.client
            .call(Method::GET, &["segments"])
            .query(query)
            .send()
            .await
    }

    /// Every segment, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        self,
        query: ListSegments,
    ) -> impl futures_util::Stream<Item = Result<Segment>> + 'a {
        let segments = self;
        crate::pagination::stream::paged(
            query,
            move |query| async move { segments.list(&query).await },
        )
    }

    /// `PUT /segments/{id}`: renames a segment (name at most 255 characters).
    pub async fn update(self, id: impl AsRef<str>, name: &str) -> Result<Segment> {
        self.client
            .call(Method::PUT, &["segments", id.as_ref()])
            .json(&NameBody { name })
            .data()
            .await
    }

    /// `DELETE /segments/{id}`: deletes a segment (its subscribers are kept).
    pub async fn delete(self, id: impl AsRef<str>) -> Result<()> {
        self.client
            .call(Method::DELETE, &["segments", id.as_ref()])
            .send_empty()
            .await
    }

    /// `GET /segments/{id}/subscribers`: one page of the segment's subscribers (active by
    /// default).
    pub async fn subscribers(
        self,
        id: impl AsRef<str>,
        query: &ListSubscribers,
    ) -> Result<CursorPage<Subscriber>> {
        self.client
            .call(Method::GET, &["segments", id.as_ref(), "subscribers"])
            .query(query)
            .send()
            .await
    }

    /// Every subscriber of the segment matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn subscribers_all(
        self,
        id: impl Into<Id>,
        query: ListSubscribers,
    ) -> impl futures_util::Stream<Item = Result<Subscriber>> + 'a {
        let segments = self;
        let id = id.into();
        crate::pagination::stream::cursored(query, move |query| {
            let id = id.clone();
            async move { segments.subscribers(&id, &query).await }
        })
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::{Value, json};

    use super::*;
    use crate::{
        http::mock::{client, fixture},
        subscribers::SubscriberStatus,
    };

    #[tokio::test]
    async fn list() {
        let (client, mock) = client();
        mock.respond(200, fixture("segments_list"));

        let page = client
            .segments()
            .list(&ListSegments::new().limit(10).page(2))
            .await
            .unwrap();

        mock.request()
            .assert(Method::GET, "segments?limit=10&page=2");
        let segment = &page.data[0];
        assert_eq!(segment.id, "1");
        assert_eq!(segment.name, "Dummy segment");
        assert_eq!(segment.total, 21);
        assert_eq!(segment.click_rate.string, "0%");
        assert_eq!(
            segment.created_at.as_ref().map(Timestamp::as_str),
            Some("2021-08-06 10:17:45")
        );
        assert_eq!(page.meta.total, 1);
        assert_eq!(page.next_page(), None);
    }

    #[tokio::test]
    async fn subscribers() {
        let (client, mock) = client();
        mock.respond(200, fixture("segments_subscribers"));

        let page = client
            .segments()
            .subscribers(
                "5",
                &ListSubscribers::new()
                    .status(SubscriberStatus::Unsubscribed)
                    .limit(50)
                    .cursor("abc"),
            )
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "segments/5/subscribers?filter%5Bstatus%5D=unsubscribed&limit=50&cursor=abc",
        );
        assert_eq!(page.data[0].email, "dummy@example.com");
        assert_eq!(
            page.next_cursor(),
            Some("eyJpZCI6NzI1ODIxNjQ2NDY5ODcxMzYsIl9wb2ludHNUb05leHRJdGVtcyI6dHJ1ZX0")
        );
    }

    #[tokio::test]
    async fn update() {
        let (client, mock) = client();
        mock.respond(200, fixture("segments_update"));

        let segment = client.segments().update("1", "Renamed").await.unwrap();

        mock.request()
            .assert(Method::PUT, "segments/1")
            .body(&json!({"name": "Renamed"}));
        assert_eq!(segment.name, "Dummy segment");
        assert_eq!(segment.total, 21);
    }

    #[tokio::test]
    async fn delete() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);

        client.segments().delete("1").await.unwrap();

        mock.request()
            .assert(Method::DELETE, "segments/1")
            .no_body();
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn subscribers_all_follows_cursors() {
        use futures_util::TryStreamExt;

        let (client, mock) = client();
        let subscriber = fixture("segments_subscribers")["data"][0].clone();
        mock.respond(
            200,
            json!({"data": [subscriber], "meta": {"next_cursor": "next"}}),
        )
        .respond(
            200,
            json!({"data": [subscriber], "meta": {"next_cursor": null}}),
        );

        let all: Vec<Subscriber> = client
            .segments()
            .subscribers_all("5", ListSubscribers::new())
            .try_collect()
            .await
            .unwrap();

        assert_eq!(all.len(), 2);
        let requests = mock.requests();
        requests[0].assert(Method::GET, "segments/5/subscribers");
        requests[1].assert(Method::GET, "segments/5/subscribers?cursor=next");
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn list_all_walks_pages() {
        use futures_util::TryStreamExt;

        let (client, mock) = client();
        let segment = fixture("segments_list")["data"][0].clone();
        mock.respond(
            200,
            json!({"data": [segment], "meta": {"current_page": 1, "last_page": 2}}),
        )
        .respond(
            200,
            json!({"data": [segment], "meta": {"current_page": 2, "last_page": 2}}),
        );

        let all: Vec<Segment> = client
            .segments()
            .list_all(ListSegments::new().limit(1))
            .try_collect()
            .await
            .unwrap();

        assert_eq!(all.len(), 2);
        let requests = mock.requests();
        requests[0].assert(Method::GET, "segments?limit=1&page=1");
        requests[1].assert(Method::GET, "segments?limit=1&page=2");
    }
}
