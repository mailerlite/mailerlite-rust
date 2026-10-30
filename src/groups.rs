//! Groups: <https://developers.mailerlite.com/api/groups>

use std::collections::BTreeMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    CursorPage, MailerLite, Page, Result,
    subscribers::{ListSubscribers, Subscriber},
    types::{FieldValue, Id, NameBody, Rate, Sort, Timestamp, string_enum},
};

/// A subscriber group.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Group {
    /// Group id.
    pub id: Id,
    /// Group name.
    pub name: String,
    /// Active subscribers.
    #[serde(default)]
    pub active_count: u64,
    /// Emails sent to the group.
    #[serde(default)]
    pub sent_count: u64,
    /// Opens.
    #[serde(default)]
    pub opens_count: u64,
    /// Open rate.
    #[serde(default)]
    pub open_rate: Rate,
    /// Clicks.
    #[serde(default)]
    pub clicks_count: u64,
    /// Click rate.
    #[serde(default)]
    pub click_rate: Rate,
    /// Unsubscribed members.
    #[serde(default)]
    pub unsubscribed_count: u64,
    /// Unconfirmed members.
    #[serde(default)]
    pub unconfirmed_count: u64,
    /// Bounced members.
    #[serde(default)]
    pub bounced_count: u64,
    /// Junk members.
    #[serde(default)]
    pub junk_count: u64,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// `true` while MailerLite is still processing membership changes.
    #[serde(default)]
    pub processing: Option<bool>,
    /// Whether a form uses the group.
    #[serde(default)]
    pub used_in_forms: Option<bool>,
    /// Whether a site uses the group.
    #[serde(default)]
    pub used_in_sites: Option<bool>,
}

string_enum! {
    /// Sort field for [`ListGroups`].
    ///
    /// These are the values the live API accepts. The docs list `total`, which the API rejects
    /// with `400` (verified 2026-10-30); use [`ActiveCount`](Self::ActiveCount) instead.
    pub enum GroupSort {
        /// Group id.
        Id => "id",
        /// Name.
        Name => "name",
        /// Active subscribers.
        ActiveCount => "active_count",
        /// Open rate.
        OpenRate => "open_rate",
        /// Click rate.
        ClickRate => "click_rate",
        /// Creation time.
        CreatedAt => "created_at",
    }
}

/// Query for [`Groups::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListGroups {
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Partial name match.
    #[serde(rename = "filter[name]", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort<GroupSort>>,
}

impl ListGroups {
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

    /// Only groups whose name contains `name`.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sort order, e.g. `Sort::desc(GroupSort::ActiveCount)`.
    #[must_use]
    pub fn sort(mut self, sort: Sort<GroupSort>) -> Self {
        self.sort = Some(sort);
        self
    }
}

/// One row of [`Groups::import_subscribers`].
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ImportSubscriber {
    /// Email address.
    pub email: String,
    /// Field values.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub fields: BTreeMap<String, FieldValue>,
}

impl ImportSubscriber {
    /// A row for `email`.
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            fields: BTreeMap::new(),
        }
    }

    /// Sets a field value.
    #[must_use]
    pub fn field(mut self, key: impl Into<String>, value: impl Into<FieldValue>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }
}

/// Response of [`Groups::import_subscribers`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportStarted {
    /// URL of the import status endpoint.
    pub import_progress_url: String,
}

impl ImportStarted {
    /// The import id, for [`Subscribers::import_status`](crate::subscribers::Subscribers::import_status).
    #[must_use]
    pub fn import_id(&self) -> Option<&str> {
        self.import_progress_url
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .filter(|id| !id.is_empty())
    }
}

#[derive(Serialize)]
struct ImportBody<'a> {
    subscribers: &'a [ImportSubscriber],
}

/// Group endpoints. Obtained with [`MailerLite::groups`].
#[derive(Clone, Copy, Debug)]
pub struct Groups<'a> {
    client: &'a MailerLite,
}

impl MailerLite {
    /// Group endpoints.
    #[must_use]
    pub fn groups(&self) -> Groups<'_> {
        Groups { client: self }
    }
}

impl<'a> Groups<'a> {
    /// `GET /groups`: one page of groups.
    pub async fn list(&self, query: &ListGroups) -> Result<Page<Group>> {
        self.client
            .call(Method::GET, &["groups"])
            .query(query)
            .send()
            .await
    }

    /// Every group matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        &self,
        query: ListGroups,
    ) -> impl futures_util::Stream<Item = Result<Group>> + 'a {
        let groups = *self;
        crate::pagination::stream::paged(
            query,
            move |query| async move { groups.list(&query).await },
        )
    }

    /// `POST /groups`: creates a group (name at most 255 characters).
    pub async fn create(&self, name: &str) -> Result<Group> {
        self.client
            .call(Method::POST, &["groups"])
            .json(&NameBody { name })
            .data()
            .await
    }

    /// `PUT /groups/{id}`: renames a group.
    pub async fn update(&self, id: impl AsRef<str>, name: &str) -> Result<Group> {
        self.client
            .call(Method::PUT, &["groups", id.as_ref()])
            .json(&NameBody { name })
            .data()
            .await
    }

    /// `DELETE /groups/{id}`: deletes a group (its subscribers are kept).
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<()> {
        self.client
            .call(Method::DELETE, &["groups", id.as_ref()])
            .send_empty()
            .await
    }

    /// `GET /groups/{id}/subscribers`: one page of the group's subscribers (active by default).
    pub async fn subscribers(
        &self,
        id: impl AsRef<str>,
        query: &ListSubscribers,
    ) -> Result<CursorPage<Subscriber>> {
        self.client
            .call(Method::GET, &["groups", id.as_ref(), "subscribers"])
            .query(query)
            .send()
            .await
    }

    /// Every subscriber of the group matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn subscribers_all(
        &self,
        id: impl Into<Id>,
        query: ListSubscribers,
    ) -> impl futures_util::Stream<Item = Result<Subscriber>> + 'a {
        let groups = *self;
        let id = id.into();
        crate::pagination::stream::cursored(query, move |query| {
            let id = id.clone();
            async move { groups.subscribers(&id, &query).await }
        })
    }

    /// `POST /subscribers/{subscriber_id}/groups/{group_id}`: adds a subscriber to the group.
    pub async fn assign_subscriber(
        &self,
        group_id: impl AsRef<str>,
        subscriber_id: impl AsRef<str>,
    ) -> Result<Group> {
        self.client
            .call(
                Method::POST,
                &[
                    "subscribers",
                    subscriber_id.as_ref(),
                    "groups",
                    group_id.as_ref(),
                ],
            )
            .data()
            .await
    }

    /// `DELETE /subscribers/{subscriber_id}/groups/{group_id}`: removes a subscriber from the
    /// group. `subscriber_id` may also be the subscriber's email, as shown in MailerLite's
    /// basic integration guide (the endpoint reference only mentions ids).
    pub async fn unassign_subscriber(
        &self,
        group_id: impl AsRef<str>,
        subscriber_id: impl AsRef<str>,
    ) -> Result<()> {
        self.client
            .call(
                Method::DELETE,
                &[
                    "subscribers",
                    subscriber_id.as_ref(),
                    "groups",
                    group_id.as_ref(),
                ],
            )
            .send_empty()
            .await
    }

    /// `POST /groups/{id}/import-subscribers`: bulk-imports subscribers into the group in the
    /// background. Limited to 5 imports per minute; poll progress with
    /// [`Subscribers::import_status`](crate::subscribers::Subscribers::import_status).
    pub async fn import_subscribers(
        &self,
        id: impl AsRef<str>,
        subscribers: &[ImportSubscriber],
    ) -> Result<ImportStarted> {
        self.client
            .call(Method::POST, &["groups", id.as_ref(), "import-subscribers"])
            .json(&ImportBody { subscribers })
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::{Value, json};

    use super::*;
    use crate::http::mock::{client, fixture};

    fn group_body() -> Value {
        json!({"data": fixture("groups_list")["data"][0]})
    }

    #[tokio::test]
    async fn list_with_filters_and_sort() {
        let (client, mock) = client();
        mock.respond(200, fixture("groups_list"));

        let page = client
            .groups()
            .list(
                &ListGroups::new()
                    .limit(5)
                    .page(2)
                    .name("vip")
                    .sort(Sort::desc(GroupSort::OpenRate)),
            )
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "groups?limit=5&page=2&filter%5Bname%5D=vip&sort=-open_rate",
        );
        assert_eq!(page.data[0].name, "dummy group");
        assert_eq!(page.data[0].open_rate.string, "0%");
        assert_eq!(page.next_page(), None);
    }

    #[tokio::test]
    async fn create_update_delete() {
        let (client, mock) = client();
        mock.respond(201, group_body())
            .respond(200, group_body())
            .respond(204, Value::Null);

        let created = client.groups().create("VIP").await.unwrap();
        client.groups().update("1", "VIPs").await.unwrap();
        client.groups().delete("1").await.unwrap();

        assert_eq!(created.id, "1");
        let requests = mock.requests();
        requests[0]
            .assert(Method::POST, "groups")
            .body(&json!({"name": "VIP"}));
        requests[1]
            .assert(Method::PUT, "groups/1")
            .body(&json!({"name": "VIPs"}));
        requests[2].assert(Method::DELETE, "groups/1").no_body();
    }

    #[tokio::test]
    async fn subscribers_and_membership() {
        let (client, mock) = client();
        mock.respond(200, fixture("subscribers_list"))
            .respond(201, group_body())
            .respond(204, Value::Null);

        let page = client
            .groups()
            .subscribers("7", &ListSubscribers::new().limit(1000))
            .await
            .unwrap();
        client.groups().assign_subscriber("7", "42").await.unwrap();
        client
            .groups()
            .unassign_subscriber("7", "ann@example.com")
            .await
            .unwrap();

        assert_eq!(page.data.len(), 1);
        let requests = mock.requests();
        requests[0].assert(Method::GET, "groups/7/subscribers?limit=1000");
        requests[1].assert(Method::POST, "subscribers/42/groups/7");
        requests[2].assert(Method::DELETE, "subscribers/ann@example.com/groups/7");
    }

    #[tokio::test]
    async fn import_subscribers() {
        let (client, mock) = client();
        mock.respond(
            200,
            json!({"import_progress_url": "https://connect.mailerlite.com/api/subscribers/import/100000000000000000"}),
        );

        let started = client
            .groups()
            .import_subscribers(
                "7",
                &[
                    ImportSubscriber::new("a@example.com").field("name", "A"),
                    ImportSubscriber::new("b@example.com"),
                ],
            )
            .await
            .unwrap();

        mock.request().assert(Method::POST, "groups/7/import-subscribers").body(&json!({
            "subscribers": [{"email": "a@example.com", "fields": {"name": "A"}}, {"email": "b@example.com"}]
        }));
        assert_eq!(started.import_id(), Some("100000000000000000"));
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn list_all_walks_pages() {
        use futures_util::TryStreamExt;

        let (client, mock) = client();
        let group = fixture("groups_list")["data"][0].clone();
        mock.respond(
            200,
            json!({"data": [group], "meta": {"current_page": 1, "last_page": 2}}),
        )
        .respond(
            200,
            json!({"data": [group], "meta": {"current_page": 2, "last_page": 2}}),
        );

        let all: Vec<Group> = client
            .groups()
            .list_all(ListGroups::new())
            .try_collect()
            .await
            .unwrap();

        assert_eq!(all.len(), 2);
        let requests = mock.requests();
        requests[0].assert(Method::GET, "groups?page=1");
        requests[1].assert(Method::GET, "groups?page=2");
    }
}
