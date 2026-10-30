//! Dynamic segment operations.
use crate::resources::{
    groups::validate_name,
    subscribers::{SubscriberMembers, set},
};
use crate::{
    Error, MailerLite, Response, Result,
    http::{OperationPolicy, body},
    models::segments::Segment,
    pagination::NumberedPage,
};
use reqwest::Method;
use serde::Serialize;
/// Rename parameters; segment rule editing is not documented by this API.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSegment {
    name: String,
}
impl UpdateSegment {
    /// Supply the new name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
/// Segment API handle.
#[derive(Debug, Clone, Copy)]
pub struct Segments<'a> {
    client: &'a MailerLite,
}
impl<'a> Segments<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List segments using page numbers.
    pub fn list(&self) -> ListSegments<'a> {
        ListSegments {
            client: self.client,
            query: vec![],
            error: None,
        }
    }
    /// Rename an existing segment.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: UpdateSegment,
    ) -> Result<Response<Segment>> {
        validate_name(&params.name)?;
        self.client
            .data(
                Method::PUT,
                &["segments", id.as_ref()],
                &[],
                Some(body(&params)?),
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Delete a segment.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<()>> {
        self.client
            .empty(
                Method::DELETE,
                &["segments", id.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE.with_success_statuses(&[204]),
            )
            .await
    }
    /// List matching subscribers using opaque cursors.
    pub fn subscribers(&self, id: impl Into<String>) -> SubscriberMembers<'a> {
        SubscriberMembers::new(
            self.client,
            vec!["segments".into(), id.into(), "subscribers".into()],
            None,
        )
    }
}
/// Segment list builder.
#[derive(Debug)]
pub struct ListSegments<'a> {
    client: &'a MailerLite,
    query: Vec<(String, String)>,
    error: Option<String>,
}
impl<'a> ListSegments<'a> {
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
    /// Fetch one page.
    pub async fn send(self) -> Result<Response<NumberedPage<Segment>>> {
        if let Some(e) = self.error {
            return Err(Error::Config(e));
        }
        self.client
            .request(
                Method::GET,
                &["segments"],
                &self.query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Lazily fetch segments as pages are consumed.
    #[cfg(feature = "stream")]
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<Segment>> + 'a {
        use futures_util::StreamExt;
        let count = if self.error.is_none() { usize::MAX } else { 0 };
        futures_util::stream::iter(self.error.map(Error::Config).into_iter().map(Err)).chain(
            crate::pagination::numbered_stream(self.client, vec!["segments".into()], self.query)
                .take(count),
        )
    }
}
