//! Group operations and request parameters.
use crate::resources::subscribers::{SubscriberMembers, set};
use crate::{
    Error, MailerLite, Response, Result,
    http::{OperationPolicy, body},
    models::groups::Group,
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
    pub fn list(&self) -> ListGroups<'a> {
        ListGroups {
            client: self.client,
            query: vec![],
            error: None,
        }
    }
    /// Create a named group.
    pub async fn create(&self, params: CreateGroup) -> Result<Response<Group>> {
        validate_name(&params.name)?;
        self.client
            .data(
                Method::POST,
                &["groups"],
                &[],
                Some(body(&params)?),
                OperationPolicy::WRITE.with_success_statuses(&[201]),
            )
            .await
    }
    /// Rename an existing group.
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
                Some(body(&params)?),
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Delete a group.
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
    pub fn subscribers(&self, id: impl Into<String>) -> SubscriberMembers<'a> {
        SubscriberMembers::new(
            self.client,
            vec!["groups".into(), id.into(), "subscribers".into()],
            Some(1000),
        )
    }
    /// Assign an existing subscriber to a group.
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
