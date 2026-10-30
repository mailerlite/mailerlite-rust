//! Dynamic segment operations.
use crate::resources::{
    groups::validate_name,
    subscribers::{SubscriberMembers, set},
};
use crate::{
    Error, MailerLite, Response, Result, http::OperationPolicy, models::segments::Segment,
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
    ///
    /// Only constructs a builder. See [`ListSegments::send`] for validation and errors.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.segments().list().limit(25).send().await?;
    ///
    /// println!("{} segments", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListSegments<'a> {
        ListSegments {
            client: self.client,
            query: vec![],
            error: None,
        }
    }
    /// Rename an existing segment.
    ///
    /// This write is never automatically retried. A failure after dispatch does not
    /// prove the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::segments::UpdateSegment;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.segments()
    ///     .update("123456789", UpdateSegment::new("Updated name")).await?;
    ///
    /// println!("{}", response.data.name);
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
        params: UpdateSegment,
    ) -> Result<Response<Segment>> {
        validate_name(&params.name)?;
        self.client
            .data(
                Method::PUT,
                &["segments", id.as_ref()],
                &[],
                Some(&params),
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Delete a segment.
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
    /// mailerlite.segments().delete("123456789").await?;
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
                &["segments", id.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE.with_success_statuses(&[204]),
            )
            .await
    }
    /// List matching subscribers using opaque cursors.
    ///
    /// Returns a builder without making a request. See [`SubscriberMembers::send`]
    /// for errors. Only group membership lists support `include_groups`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.segments().subscribers("123456789")
    ///     .limit(100).send().await?;
    ///
    /// println!("{} matching subscribers", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
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
#[must_use = "request builders do nothing until sent; call .send().await"]
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.segments().list().page(1).limit(25).send().await?;
    ///
    /// println!("Next page: {:?}", page.data.next_page());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] if zero was supplied as the page number
    /// or limit.
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
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
    /// let resource = mailerlite.segments();
    /// let stream = resource.list().limit(25).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(segment) = stream.try_next().await? {
    ///     println!("{}", segment.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors are yielded as stream items. Invalid parameters and request failures
    /// follow [`Self::send`]. Unexpected current-page numbers yield [`crate::Error::Pagination`].
    /// The stream terminates after an error; previously yielded items remain available.
    /// Missing continuation metadata ends the stream normally.
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
