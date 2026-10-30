//! Forms and their subscriber lists.
use crate::models::{forms::Form, subscribers::Subscriber};
use crate::{
    Error, MailerLite, Response, Result,
    http::OperationPolicy,
    pagination::{CursorPage, NumberedPage},
};
use reqwest::Method;
/// FormType request value.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum FormType {
    /// Popup.
    #[serde(rename = "popup")]
    Popup,
    /// Embedded.
    #[serde(rename = "embedded")]
    Embedded,
    /// Promotion.
    #[serde(rename = "promotion")]
    Promotion,
}
impl FormType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Popup => "popup",
            Self::Embedded => "embedded",
            Self::Promotion => "promotion",
        }
    }
}
/// Required new form name.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UpdateForm {
    name: String,
}
impl UpdateForm {
    /// Create rename parameters.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
/// Form resource handle.
#[derive(Debug, Clone, Copy)]
pub struct Forms<'a> {
    client: &'a MailerLite,
}
impl<'a> Forms<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List forms of a required type.
    ///
    /// The form type determines the endpoint. No request is sent until the returned
    /// builder is consumed; see [`ListForms::send`] for validation and errors.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::forms::FormType;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.forms().list(FormType::Embedded).limit(25).send().await?;
    ///
    /// println!("{} embedded forms", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self, kind: FormType) -> ListForms<'a> {
        ListForms::new(self.client, vec!["forms".into(), kind.as_str().into()])
    }
    /// Fetch a form.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.forms().get("123456789").await?;
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
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Form>> {
        self.client
            .data(
                Method::GET,
                &["forms", id.as_ref()],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Rename an existing form.
    ///
    /// This write is never automatically retried. A failure after dispatch does not
    /// prove the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::forms::UpdateForm;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.forms()
    ///     .update("123456789", UpdateForm::new("Newsletter signup")).await?;
    ///
    /// println!("{}", response.data.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for an empty ID or a dot path segment (`.` or `..`). Name validation is performed by the server.
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn update(&self, id: impl AsRef<str>, params: UpdateForm) -> Result<Response<Form>> {
        self.client
            .data(
                Method::PUT,
                &["forms", id.as_ref()],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Delete a form.
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
    /// mailerlite.forms().delete("123456789").await?;
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
                &["forms", id.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE,
            )
            .await
    }
    /// List subscribers who signed up using this form.
    ///
    /// Returns a lazy cursor-page builder. See [`FormSubscribers::send`] for
    /// validation and request errors.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.forms().subscribers("123456789").limit(25).send().await?;
    ///
    /// println!("{} subscribers", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn subscribers(&self, id: impl AsRef<str>) -> FormSubscribers<'a> {
        FormSubscribers::new(
            self.client,
            vec!["forms".into(), id.as_ref().into(), "subscribers".into()],
        )
    }
}
/// Parameters for a paginated request.
#[derive(Debug)]
pub struct ListForms<'a> {
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
}
impl<'a> ListForms<'a> {
    fn new(client: &'a MailerLite, path: Vec<String>) -> Self {
        Self {
            client,
            path,
            query: Vec::new(),
        }
    }
    /// Set limit.
    pub fn limit(mut self, value: u32) -> Self {
        self.query.retain(|(key, _)| key != "limit");
        self.query.push(("limit".into(), value.to_string()));
        self
    }
    /// Set page.
    pub fn page(mut self, value: u64) -> Self {
        self.query.retain(|(key, _)| key != "page");
        self.query.push(("page".into(), value.to_string()));
        self
    }
    /// Set name.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "filter[name]");
        self.query.push(("filter[name]".into(), value.into()));
        self
    }
    /// Set sort.
    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "sort");
        self.query.push(("sort".into(), value.into()));
        self
    }
    /// Fetch one page.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::forms::FormType;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.forms().list(FormType::Embedded)
    ///     .page(1).limit(25).send().await?;
    ///
    /// println!("Next page: {:?}", page.data.next_page());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for a zero page number or limit. Sort strings
    /// and other server-specific query rules are validated by MailerLite.
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn send(self) -> Result<Response<NumberedPage<Form>>> {
        validate_query(&self.query)?;
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &self.query, None, OperationPolicy::READ)
            .await
    }
    /// Fetch subsequent pages lazily while preserving request filters.
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
    /// use mailerlite_rs::resources::forms::FormType;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let resource = mailerlite.forms();
    /// let stream = resource.list(FormType::Embedded).limit(25).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(form) = stream.try_next().await? {
    ///     println!("{}", form.id);
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
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<Form>> + 'a {
        crate::pagination::validated_stream(
            validate_query(&self.query),
            crate::pagination::numbered_stream(self.client, self.path, self.query),
        )
    }
}
/// Parameters for a paginated request.
#[derive(Debug)]
pub struct FormSubscribers<'a> {
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
}
impl<'a> FormSubscribers<'a> {
    fn new(client: &'a MailerLite, path: Vec<String>) -> Self {
        Self {
            client,
            path,
            query: Vec::new(),
        }
    }
    /// Set limit.
    pub fn limit(mut self, value: u32) -> Self {
        self.query.retain(|(key, _)| key != "limit");
        self.query.push(("limit".into(), value.to_string()));
        self
    }
    /// Set cursor.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "cursor");
        self.query.push(("cursor".into(), value.into()));
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
    /// let page = mailerlite.forms().subscribers("123456789")
    ///     .limit(25).send().await?;
    ///
    /// println!("{} subscribers", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for a zero limit or an empty/dot form ID.
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn send(self) -> Result<Response<CursorPage<Subscriber>>> {
        validate_query(&self.query)?;
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &self.query, None, OperationPolicy::READ)
            .await
    }
    /// Fetch subsequent pages lazily while preserving request filters.
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
    /// let resource = mailerlite.forms();
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
        crate::pagination::validated_stream(
            validate_query(&self.query),
            crate::pagination::cursor_stream(self.client, self.path, self.query),
        )
    }
}
fn validate_query(query: &[(String, String)]) -> Result<()> {
    if query
        .iter()
        .any(|(key, value)| (key == "page" || key == "limit") && value == "0")
    {
        return Err(Error::Config("page and limit must be positive".into()));
    }
    Ok(())
}
