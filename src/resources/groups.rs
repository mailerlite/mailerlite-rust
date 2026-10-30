//! Group operations and request parameters.
use crate::resources::subscribers::{SubscriberMembers, set};
use crate::{
    Error, MailerLite, Response, Result, http::OperationPolicy, models::groups::Group,
    pagination::NumberedPage,
};
use reqwest::Method;
use serde::Serialize;
/// Required name for creating a group.
#[derive(Debug, Clone, Serialize)]
pub struct CreateGroup {
    name: String,
}
impl CreateGroup {
    /// Supply the new group name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
/// Required replacement name for a group.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateGroup {
    name: String,
}
impl UpdateGroup {
    /// Supply the new group name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
/// Group API handle.
#[derive(Debug, Clone, Copy)]
pub struct Groups<'a> {
    client: &'a MailerLite,
}
impl<'a> Groups<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List groups using page numbers.
    ///
    /// This only creates a builder. See [`ListGroups::send`] for validation and errors.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.groups().list().limit(25).send().await?;
    ///
    /// println!("{} groups", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListGroups<'a> {
        ListGroups {
            client: self.client,
            query: vec![],
            error: None,
        }
    }
    /// Create a named group.
    ///
    /// This write is never automatically retried. A failure after dispatch does not
    /// prove the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::groups::CreateGroup;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let group = mailerlite.groups().create(CreateGroup::new("Newsletter")).await?;
    ///
    /// println!("{}", group.data.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] if the name is empty or exceeds 255 characters.
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn create(&self, params: CreateGroup) -> Result<Response<Group>> {
        validate_name(&params.name)?;
        self.client
            .data(
                Method::POST,
                &["groups"],
                &[],
                Some(&params),
                OperationPolicy::WRITE.with_success_statuses(&[201]),
            )
            .await
    }
    /// Rename an existing group.
    ///
    /// This write is never automatically retried. A failure after dispatch does not
    /// prove the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::groups::UpdateGroup;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let group = mailerlite.groups()
    ///     .update("123456789", UpdateGroup::new("Weekly newsletter")).await?;
    ///
    /// println!("{}", group.data.name);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] if the name is empty or exceeds 255 characters.
    /// Returns [`crate::Error::Config`] for an empty ID or a dot path segment (`.` or `..`).
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: UpdateGroup,
    ) -> Result<Response<Group>> {
        validate_name(&params.name)?;
        self.client
            .data(
                Method::PUT,
                &["groups", id.as_ref()],
                &[],
                Some(&params),
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Delete a group.
    ///
    /// This write is never automatically retried. A failure after dispatch does not
    /// prove the server left the resource unchanged.
    /// Success returns an empty response.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// mailerlite.groups().delete("123456789").await?;
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
                &["groups", id.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE.with_success_statuses(&[204]),
            )
            .await
    }
    /// List subscribers belonging to a group.
    ///
    /// No request is sent until the returned builder is consumed. Group pages allow
    /// limits up to 1000; see [`SubscriberMembers::send`] for validation and errors.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.groups().subscribers("123456789")
    ///     .limit(100).send().await?;
    ///
    /// println!("{} members", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn subscribers(&self, id: impl Into<String>) -> SubscriberMembers<'a> {
        SubscriberMembers::new(
            self.client,
            vec!["groups".into(), id.into(), "subscribers".into()],
            Some(1000),
        )
    }
    /// Assign an existing subscriber to a group.
    ///
    /// This write is never automatically retried. A failure after dispatch does not
    /// prove the server left the resource unchanged.
    /// The arguments are group ID first, then subscriber ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.groups().assign("123456789", "987654321").await?;
    ///
    /// println!("Assigned to group {}", response.data.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty ID or a dot path segment (`.` or `..`). Both group and subscriber IDs are checked.
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn assign(
        &self,
        group_id: impl AsRef<str>,
        subscriber_id: impl AsRef<str>,
    ) -> Result<Response<Group>> {
        self.client
            .data(
                Method::POST,
                &[
                    "subscribers",
                    subscriber_id.as_ref(),
                    "groups",
                    group_id.as_ref(),
                ],
                &[],
                None,
                OperationPolicy::WRITE.with_success_statuses(&[200, 201]),
            )
            .await
    }
    /// Unassign a subscriber from a group.
    ///
    /// This write is never automatically retried. A failure after dispatch does not
    /// prove the server left the resource unchanged.
    /// This removes group membership, rather than deleting the subscriber.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// mailerlite.groups().unassign("123456789", "987654321").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty ID or a dot path segment (`.` or `..`). Both group and subscriber IDs are checked.
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn unassign(
        &self,
        group_id: impl AsRef<str>,
        subscriber_id: impl AsRef<str>,
    ) -> Result<Response<()>> {
        self.client
            .empty(
                Method::DELETE,
                &[
                    "subscribers",
                    subscriber_id.as_ref(),
                    "groups",
                    group_id.as_ref(),
                ],
                &[],
                None,
                OperationPolicy::WRITE.with_success_statuses(&[204]),
            )
            .await
    }
}
/// Supported group sort fields.
#[derive(Debug, Clone, Copy)]
pub enum GroupSort {
    /// Group name.
    Name,
    /// Total subscribers.
    Total,
    /// Open rate.
    OpenRate,
    /// Click rate.
    ClickRate,
    /// Creation timestamp.
    CreatedAt,
}
impl GroupSort {
    fn as_str(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Total => "total",
            Self::OpenRate => "open_rate",
            Self::ClickRate => "click_rate",
            Self::CreatedAt => "created_at",
        }
    }
}
/// Builder for group pages.
#[derive(Debug)]
#[must_use = "request builders do nothing until sent; call .send().await"]
pub struct ListGroups<'a> {
    client: &'a MailerLite,
    query: Vec<(String, String)>,
    error: Option<String>,
}
impl<'a> ListGroups<'a> {
    /// Set a positive page size.
    pub fn limit(mut self, value: u64) -> Self {
        if value == 0 {
            self.error = Some("limit must be positive".into());
        }
        set(&mut self.query, "limit", value.to_string());
        self
    }
    /// Set a one-based page number.
    pub fn page(mut self, value: u64) -> Self {
        if value == 0 {
            self.error = Some("page starts at one".into());
        }
        set(&mut self.query, "page", value.to_string());
        self
    }
    /// Filter by a partial name match.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        set(&mut self.query, "filter[name]", value.into());
        self
    }
    /// Sort by a documented field, optionally descending.
    pub fn sort(mut self, field: GroupSort, descending: bool) -> Self {
        set(
            &mut self.query,
            "sort",
            format!("{}{}", if descending { "-" } else { "" }, field.as_str()),
        );
        self
    }
    /// Fetch one page.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let groups = mailerlite.groups();
    /// let first = groups.list().page(1).limit(25).send().await?;
    ///
    /// if let Some(page) = first.data.next_page() {
    ///     let second = groups.list().page(page).limit(25).send().await?;
    ///
    ///     println!("{} groups on the next page", second.data.data.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] before dispatch if zero was supplied as the page
    /// number or limit. API rejection, transport failure, unexpected responses,
    /// and execution deadlines return [`Error::Api`], [`Error::Transport`],
    /// [`Error::Decode`], or [`Error::Timeout`]. Missing continuation metadata is
    /// not an error; [`NumberedPage::next_page`] then returns `None`.
    pub async fn send(self) -> Result<Response<NumberedPage<Group>>> {
        if let Some(e) = self.error {
            return Err(Error::Config(e));
        }
        self.client
            .request(
                Method::GET,
                &["groups"],
                &self.query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Lazily fetch groups as pages are consumed.
    ///
    /// Requires the `stream` feature. Requests begin on polling; filters are
    /// preserved and response links are never followed. Dropping the stream
    /// stops further requests. Each page has its own operation deadline.
    ///
    /// # Examples
    ///
    /// Add `futures-util` to your application's dependencies to consume the stream.
    ///
    /// ```no_run
    /// use futures_util::TryStreamExt;
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let groups = mailerlite.groups();
    /// let stream = groups.list().limit(25).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(group) = stream.try_next().await? {
    ///     println!("{}", group.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors are yielded as stream items. Invalid builder parameters yield
    /// [`Error::Config`] without dispatch. Page requests propagate the errors
    /// described by [`Self::send`]. A reported current page that differs from the
    /// requested page yields [`Error::Pagination`]. Missing continuation metadata
    /// ends the stream normally. After an error the stream terminates; already
    /// yielded items are not rolled back.
    #[cfg(feature = "stream")]
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<Group>> + 'a {
        use futures_util::StreamExt;
        let count = if self.error.is_none() { usize::MAX } else { 0 };
        futures_util::stream::iter(self.error.map(Error::Config).into_iter().map(Err)).chain(
            crate::pagination::numbered_stream(self.client, vec!["groups".into()], self.query)
                .take(count),
        )
    }
}
pub(crate) fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() || name.chars().count() > 255 {
        Err(Error::Config(
            "name must contain 1 to 255 characters".into(),
        ))
    } else {
        Ok(())
    }
}
