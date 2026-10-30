//! Subscriber operations and typed parameters.
use crate::{
    Error, MailerLite, Response, Result,
    http::OperationPolicy,
    models::{
        GroupId,
        subscribers::{
            FieldValue, ForgetSubscriber, Subscriber, SubscriberActivity, SubscriberStatus,
        },
    },
    pagination::CursorPage,
    response::Count,
};
use reqwest::Method;
use serde::Serialize;
use std::collections::BTreeMap;

/// Optional fields for updating an existing subscriber.
///
/// Supplying `groups` replaces membership; omitting it preserves membership.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateSubscriber {
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    fields: BTreeMap<String, FieldValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<Vec<GroupId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<SubscriberStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscribed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opted_in_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optin_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unsubscribed_at: Option<String>,
}
impl UpdateSubscriber {
    /// Construct an update that leaves optional fields unchanged.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set a custom field. Use `FieldValue::Null` to explicitly clear it.
    pub fn field(mut self, name: impl Into<String>, value: impl Into<FieldValue>) -> Self {
        self.fields.insert(name.into(), value.into());
        self
    }
    /// Replace all group memberships, including clearing them with an empty list.
    pub fn groups(mut self, groups: impl IntoIterator<Item = impl Into<GroupId>>) -> Self {
        self.groups = Some(groups.into_iter().map(Into::into).collect());
        self
    }
    /// Set subscription status; account policy may reject reactivation.
    pub fn status(mut self, status: SubscriberStatus) -> Self {
        self.status = Some(status);
        self
    }
    /// Set subscription time in `yyyy-MM-dd HH:mm:ss` format.
    pub fn subscribed_at(mut self, value: impl Into<String>) -> Self {
        self.subscribed_at = Some(value.into());
        self
    }
    /// Set subscription IP address.
    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }
    /// Set opt-in time in `yyyy-MM-dd HH:mm:ss` format.
    pub fn opted_in_at(mut self, value: impl Into<String>) -> Self {
        self.opted_in_at = Some(value.into());
        self
    }
    /// Set opt-in IP address.
    pub fn optin_ip(mut self, value: impl Into<String>) -> Self {
        self.optin_ip = Some(value.into());
        self
    }
    /// Set unsubscription time in `yyyy-MM-dd HH:mm:ss` format.
    pub fn unsubscribed_at(mut self, value: impl Into<String>) -> Self {
        self.unsubscribed_at = Some(value.into());
        self
    }
}
/// Parameters for creating or updating a subscriber by email.
///
/// Upserts add supplied groups and preserve omitted memberships.
#[derive(Debug, Clone, Serialize)]
pub struct UpsertSubscriber {
    email: String,
    #[serde(flatten)]
    changes: UpdateSubscriber,
    #[serde(skip_serializing_if = "Option::is_none")]
    resubscribe: Option<bool>,
}
impl UpsertSubscriber {
    /// Supply the required email address.
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            changes: UpdateSubscriber::new(),
            resubscribe: None,
        }
    }
    /// Set a custom field.
    pub fn field(mut self, name: impl Into<String>, value: impl Into<FieldValue>) -> Self {
        self.changes = self.changes.field(name, value);
        self
    }
    /// Add the subscriber to these groups.
    pub fn groups(mut self, groups: impl IntoIterator<Item = impl Into<GroupId>>) -> Self {
        self.changes = self.changes.groups(groups);
        self
    }
    /// Set the subscriber status.
    pub fn status(mut self, status: SubscriberStatus) -> Self {
        self.changes = self.changes.status(status);
        self
    }
    /// Explicitly request resubscription.
    pub fn resubscribe(mut self, value: bool) -> Self {
        self.resubscribe = Some(value);
        self
    }
    /// Set subscription time in the documented format.
    pub fn subscribed_at(mut self, value: impl Into<String>) -> Self {
        self.changes = self.changes.subscribed_at(value);
        self
    }
    /// Set subscription IP address.
    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.changes = self.changes.ip_address(value);
        self
    }
    /// Set opt-in time in the documented format.
    pub fn opted_in_at(mut self, value: impl Into<String>) -> Self {
        self.changes = self.changes.opted_in_at(value);
        self
    }
    /// Set opt-in IP address.
    pub fn optin_ip(mut self, value: impl Into<String>) -> Self {
        self.changes = self.changes.optin_ip(value);
        self
    }
    /// Set unsubscription time in the documented format.
    pub fn unsubscribed_at(mut self, value: impl Into<String>) -> Self {
        self.changes = self.changes.unsubscribed_at(value);
        self
    }
}
/// Subscriber API handle.
#[derive(Debug, Clone, Copy)]
pub struct Subscribers<'a> {
    client: &'a MailerLite,
}
impl<'a> Subscribers<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List subscribers with cursor pagination.
    ///
    /// Returns a lazy builder; no request is sent until [`ListSubscribers::send`] or
    /// stream consumption. See that method for validation and request errors.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.subscribers().list().limit(100).send().await?;
    ///
    /// println!("{} subscribers", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListSubscribers<'a> {
        ListSubscribers {
            client: self.client,
            query: Vec::new(),
            error: None,
        }
    }
    /// Create or update a subscriber. This write is never automatically retried.
    ///
    /// The email address selects the subscriber. Supplied groups add membership;
    /// omitted groups preserve it. Replaying an upsert is not guaranteed to be
    /// free of side effects. Use [`Self::update`] to replace group membership.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{MailerLite, resources::subscribers::UpsertSubscriber};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.subscribers()
    ///     .upsert(UpsertSubscriber::new("jane@example.com").field("name", "Jane"))
    ///     .await?;
    ///
    /// println!("Subscriber ID: {}", response.data.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Serialize`] if parameters cannot be serialized, including
    /// an unknown subscriber status. Email and account rules are validated by
    /// MailerLite; rejected requests return [`Error::Api`]. For HTTP 422, inspect
    /// [`crate::ApiError::validation_errors`]. Transport failures, malformed or
    /// unexpected responses, and execution deadlines return [`Error::Transport`],
    /// [`Error::Decode`], and [`Error::Timeout`] respectively.
    /// A failure after dispatch does not prove the subscriber was unchanged.
    pub async fn upsert(&self, params: UpsertSubscriber) -> Result<Response<Subscriber>> {
        self.client
            .data(
                Method::POST,
                &["subscribers"],
                &[],
                Some(&params),
                OperationPolicy::WRITE.with_success_statuses(&[200, 201]),
            )
            .await
    }
    /// Update a subscriber by ID; supplied groups replace membership.
    ///
    /// Omitted groups preserve membership; an empty group list removes all
    /// memberships. This write is never automatically retried. Use
    /// [`Self::upsert`] when identifying the subscriber by email or adding groups.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::{Error, MailerLite, resources::subscribers::UpdateSubscriber};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let update = UpdateSubscriber::new()
    ///     .field("name", "Jane")
    ///     .groups(["123456789"]); // Replaces existing memberships.
    ///
    /// match mailerlite.subscribers().update("31986843064993537", update).await {
    ///     Ok(response) => println!("Updated {}", response.data.id),
    ///     Err(Error::Api(error)) if error.is_validation() => {
    ///         // Messages may contain submitted personal data; inspect deliberately.
    ///         for field in error.validation_errors().keys() {
    ///             eprintln!("Invalid field: {field}");
    ///         }
    ///     }
    ///     Err(error) => return Err(error),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] for an empty ID or a dot path segment (`.` or
    /// `..`), and [`Error::Serialize`] for an unserializable request, including an
    /// unknown status. Server validation, missing resources, and other rejected
    /// requests return [`Error::Api`]. HTTP execution may also return
    /// [`Error::Transport`], [`Error::Decode`], or [`Error::Timeout`]. After a
    /// transport failure or timeout, check the resource before repeating a write.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: UpdateSubscriber,
    ) -> Result<Response<Subscriber>> {
        self.client
            .data(
                Method::PUT,
                &["subscribers", id.as_ref()],
                &[],
                Some(&params),
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Fetch by subscriber ID or email. The value is encoded as one path segment.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.subscribers().get("jane@example.com").await?;
    ///
    /// println!("{}", response.data.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty ID or a dot path segment (`.` or `..`).
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn get(&self, id_or_email: impl AsRef<str>) -> Result<Response<Subscriber>> {
        self.client
            .data(
                Method::GET,
                &["subscribers", id_or_email.as_ref()],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Fetch the total subscriber count.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.subscribers().count().await?;
    ///
    /// println!("{} subscribers", response.data.total);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn count(&self) -> Result<Response<Count>> {
        self.client
            .request(
                Method::GET,
                &["subscribers"],
                &[("limit".into(), "0".into())],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// List activity manually. The reference does not establish automatic continuation.
    ///
    /// Returns a manual-query builder without making a request. See
    /// [`ListSubscriberActivity::send`] for errors. There is no automatic activity stream.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.subscribers().activity("123456789")
    ///     .page(1).limit(25).send().await?;
    ///
    /// println!("{} activity entries", response.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn activity(&self, id: impl Into<String>) -> ListSubscriberActivity<'a> {
        ListSubscriberActivity {
            client: self.client,
            id: id.into(),
            query: Vec::new(),
            error: None,
        }
    }
    /// Remove the subscriber while retaining their history for resubscription.
    ///
    /// This write is never automatically retried. A failure after dispatch does not
    /// prove the server left the resource unchanged.
    /// Success has an empty body. Use [`Self::forget`] for a complete forgetting request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// mailerlite.subscribers().delete("123456789").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty ID or a dot path segment (`.` or `..`).
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<()>> {
        self.client
            .empty(
                Method::DELETE,
                &["subscribers", id.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE.with_success_statuses(&[204]),
            )
            .await
    }
    /// Request complete forgetting within the server's retention period.
    ///
    /// This write is never automatically retried. A failure after dispatch does not
    /// prove the server left the resource unchanged.
    /// The response acknowledges the request; it does not prove deletion has finished.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let receipt = mailerlite.subscribers().forget("123456789").await?;
    ///
    /// println!("Forgetting requested for {}", receipt.data.data.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty ID or a dot path segment (`.` or `..`).
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn forget(&self, id: impl AsRef<str>) -> Result<Response<ForgetSubscriber>> {
        self.client
            .request(
                Method::POST,
                &["subscribers", id.as_ref(), "forget"],
                &[],
                None,
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
}
/// Builder for the cursor-paginated subscriber list.
#[derive(Debug)]
pub struct ListSubscribers<'a> {
    client: &'a MailerLite,
    query: Vec<(String, String)>,
    error: Option<String>,
}
impl<'a> ListSubscribers<'a> {
    /// Filter by a documented subscriber status.
    pub fn status(mut self, status: SubscriberStatus) -> Self {
        if status.validate().is_err() {
            self.error = Some("unknown subscriber status".into());
        }
        set(&mut self.query, "filter[status]", status.as_str().into());
        self
    }
    /// Set a positive page size.
    pub fn limit(mut self, limit: u64) -> Self {
        if limit == 0 {
            self.error = Some("list limit must be positive; use count()".into());
        }
        set(&mut self.query, "limit", limit.to_string());
        self
    }
    /// Resume from an opaque cursor.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        set(&mut self.query, "cursor", cursor.into());
        self
    }
    /// Include group resources in subscriber responses.
    pub fn include_groups(mut self) -> Self {
        set(&mut self.query, "include", "groups".into());
        self
    }
    /// Fetch one page.
    ///
    /// Missing continuation metadata means no known next page; links are
    /// retained for inspection and never followed directly.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let subscribers = mailerlite.subscribers();
    /// let first = subscribers.list().limit(100).send().await?;
    ///
    /// if let Some(cursor) = first.data.next_cursor() {
    ///     let second = subscribers.list()
    ///         .limit(100)
    ///         .cursor(cursor)
    ///         .send()
    ///         .await?;
    ///
    ///     println!("{} subscribers on the next page", second.data.data.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] before dispatch if a zero limit or unknown
    /// status was supplied to the builder. API rejection, transport failure,
    /// malformed responses, and execution deadlines return [`Error::Api`],
    /// [`Error::Transport`], [`Error::Decode`], or [`Error::Timeout`].
    pub async fn send(self) -> Result<Response<CursorPage<Subscriber>>> {
        if let Some(e) = self.error {
            return Err(Error::Config(e));
        }
        self.client
            .request(
                Method::GET,
                &["subscribers"],
                &self.query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Lazily fetch subscribers while preserving filters.
    ///
    /// Requires the `stream` feature. The example also uses `futures-util`.
    /// Requests begin when the stream is polled. It buffers approximately one
    /// page and uses the configured endpoint, never server-provided links.
    /// Dropping the stream stops fetching additional pages. The operation timeout
    /// applies to each page request, not to consumption of the entire stream.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use futures_util::TryStreamExt;
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let subscribers = mailerlite.subscribers();
    /// let stream = subscribers.list().limit(100).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(subscriber) = stream.try_next().await? {
    ///     println!("{}", subscriber.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors are yielded as stream items. Invalid builder parameters yield
    /// [`Error::Config`] without a request. Page requests can yield the same
    /// errors as [`Self::send`]; a detected cursor cycle yields
    /// [`Error::Pagination`]. The stream terminates after an error. Previously
    /// yielded items remain available to the caller.
    #[cfg(feature = "stream")]
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<Subscriber>> + 'a {
        use futures_util::StreamExt;
        let count = if self.error.is_none() { usize::MAX } else { 0 };
        let error = self.error.map(Error::Config);
        let initial = futures_util::stream::iter(error.into_iter().map(Err));
        initial.chain(
            crate::pagination::cursor_stream(self.client, vec!["subscribers".into()], self.query)
                .take(count),
        )
    }
}
/// Cursor-paginated subscribers belonging to a group, segment, or form.
#[derive(Debug)]
pub struct SubscriberMembers<'a> {
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
    error: Option<String>,
    max_limit: Option<u64>,
}
impl<'a> SubscriberMembers<'a> {
    /// Include groups in a group-membership query.
    pub fn include_groups(mut self) -> Self {
        if self.path.first().map(String::as_str) != Some("groups") {
            self.error =
                Some("include groups is only documented for group membership lists".into());
        }
        set(&mut self.query, "include", "groups".into());
        self
    }
    /// Set the legacy segment continuation ID; prefer an opaque cursor when available.
    pub fn after(mut self, id: impl Into<String>) -> Self {
        if self.path.first().map(String::as_str) != Some("segments") {
            self.error = Some("after is only documented for segment membership lists".into());
        }
        set(&mut self.query, "after", id.into());
        self
    }
    pub(crate) fn new(client: &'a MailerLite, path: Vec<String>, max_limit: Option<u64>) -> Self {
        Self {
            client,
            path,
            query: Vec::new(),
            error: None,
            max_limit,
        }
    }
    /// Filter subscriber status.
    pub fn status(mut self, status: SubscriberStatus) -> Self {
        if status.validate().is_err() {
            self.error = Some("unknown subscriber status".into());
        }
        set(&mut self.query, "filter[status]", status.as_str().into());
        self
    }
    /// Set a positive page size within the endpoint's documented bound.
    pub fn limit(mut self, limit: u64) -> Self {
        if limit == 0 || self.max_limit.is_some_and(|max| limit > max) {
            self.error = Some("invalid subscriber page size".into());
        }
        set(&mut self.query, "limit", limit.to_string());
        self
    }
    /// Resume from an opaque cursor.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        set(&mut self.query, "cursor", cursor.into());
        self
    }
    /// Fetch one page.
    ///
    /// Group and segment accessors share this builder. The segment `after` parameter
    /// is separate from the opaque `cursor`; preserve the same filters when continuing.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.groups().subscribers("123456789")
    ///     .limit(100).include_groups().send().await?;
    ///
    /// println!("{} group members", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty/dot resource ID, zero limit,
    /// limits over 1000 on group membership lists, unknown subscriber status, or
    /// using `include_groups` outside group lists or `after` outside segment lists.
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn send(self) -> Result<Response<CursorPage<Subscriber>>> {
        if let Some(e) = self.error {
            return Err(Error::Config(e));
        }
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &self.query, None, OperationPolicy::READ)
            .await
    }
    /// Lazily fetch membership pages.
    ///
    /// Requires the `stream` feature; this example also uses `futures-util`.
    /// Requests begin when polled, preserve filters, and never follow response links.
    /// Dropping the stream stops further page fetches. Each page request has its
    /// own operation deadline.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use futures_util::TryStreamExt;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let resource = mailerlite.groups();
    /// let stream = resource.subscribers("123456789").limit(25).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(subscriber) = stream.try_next().await? {
    ///     println!("{}", subscriber.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors are yielded as stream items. Invalid parameters and request failures
    /// follow [`Self::send`]. Detected cursor cycles yield [`crate::Error::Pagination`].
    /// The stream terminates after an error; previously yielded items remain available.
    /// Missing continuation metadata ends the stream normally.
    #[cfg(feature = "stream")]
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<Subscriber>> + 'a {
        use futures_util::StreamExt;
        let count = if self.error.is_none() { usize::MAX } else { 0 };
        let initial =
            futures_util::stream::iter(self.error.map(Error::Config).into_iter().map(Err));
        initial
            .chain(crate::pagination::cursor_stream(self.client, self.path, self.query).take(count))
    }
}
/// Manual subscriber activity query.
#[derive(Debug)]
pub struct ListSubscriberActivity<'a> {
    client: &'a MailerLite,
    id: String,
    query: Vec<(String, String)>,
    error: Option<String>,
}
impl ListSubscriberActivity<'_> {
    /// Filter by a documented activity kind.
    pub fn log_name(mut self, kind: ActivityKind) -> Self {
        set(&mut self.query, "filter[log_name]", kind.as_str().into());
        self
    }
    /// Set the one-based page number.
    pub fn page(mut self, page: u64) -> Self {
        if page == 0 {
            self.error = Some("page starts at one".into());
        }
        set(&mut self.query, "page", page.to_string());
        self
    }
    /// Set a positive page size.
    pub fn limit(mut self, limit: u64) -> Self {
        if limit == 0 {
            self.error = Some("limit must be positive".into());
        }
        set(&mut self.query, "limit", limit.to_string());
        self
    }
    /// Fetch activity entries without inventing pagination metadata.
    ///
    /// The result contains entries without continuation metadata. The reviewed
    /// backend does not provide reliable exhaustive continuation, so this SDK exposes
    /// manual requests rather than an automatic stream.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let entries = mailerlite.subscribers().activity("123456789")
    ///     .page(1).limit(25).send().await?;
    ///
    /// println!("{} activity entries", entries.data.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty/dot subscriber ID or if zero
    /// was supplied as the page number or limit.
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn send(self) -> Result<Response<Vec<SubscriberActivity>>> {
        if let Some(e) = self.error {
            return Err(Error::Config(e));
        }
        self.client
            .data(
                Method::GET,
                &["subscribers", &self.id, "activity-log"],
                &self.query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
}
/// Supported subscriber activity filters.
#[derive(Debug, Clone, Copy)]
pub enum ActivityKind {
    /// Campaign delivery.
    CampaignSend,
    /// Automation delivery.
    AutomationEmailSent,
    /// Email opened.
    EmailOpen,
    /// Link clicked.
    LinkClick,
    /// Email bounced.
    EmailBounce,
    /// Spam complaint.
    SpamComplaint,
    /// Unsubscription.
    Unsubscribed,
    /// Email forwarded.
    EmailForward,
    /// Marketing preferences changed.
    MarketingPreferencesChange,
    /// Preference center activity.
    PreferenceCenter,
}
impl ActivityKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::CampaignSend => "campaign_send",
            Self::AutomationEmailSent => "automation_email_sent",
            Self::EmailOpen => "email_open",
            Self::LinkClick => "link_click",
            Self::EmailBounce => "email_bounce",
            Self::SpamComplaint => "spam_complaint",
            Self::Unsubscribed => "unsubscribed",
            Self::EmailForward => "email_forward",
            Self::MarketingPreferencesChange => "marketing_preferences_change",
            Self::PreferenceCenter => "preference_center",
        }
    }
}
pub(crate) fn set(query: &mut Vec<(String, String)>, key: &str, value: String) {
    query.retain(|(k, _)| k != key);
    query.push((key.into(), value));
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn omitted_update_fields_are_not_serialized() {
        assert_eq!(
            serde_json::to_value(UpdateSubscriber::new()).unwrap(),
            json!({})
        );
    }

    #[test]
    fn explicit_null_and_empty_membership_are_serialized() {
        assert_eq!(
            serde_json::to_value(
                UpdateSubscriber::new()
                    .field("name", FieldValue::Null)
                    .groups(Vec::<String>::new())
            )
            .unwrap(),
            json!({"fields":{"name":null},"groups":[]})
        );
    }
}
