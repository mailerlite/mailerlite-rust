//! Documented automation operations.
use crate::models::automations::{Automation, AutomationActivity};
use crate::{Error, MailerLite, Response, Result, http::OperationPolicy, pagination::NumberedPage};
use reqwest::Method;
/// ActivityStatus request value.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum ActivityStatus {
    /// Completed.
    #[serde(rename = "completed")]
    Completed,
    /// Active.
    #[serde(rename = "active")]
    Active,
    /// Canceled.
    #[serde(rename = "canceled")]
    Canceled,
    /// Failed.
    #[serde(rename = "failed")]
    Failed,
}
impl ActivityStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Active => "active",
            Self::Canceled => "canceled",
            Self::Failed => "failed",
        }
    }
}
/// A draft automation name.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateAutomation {
    name: String,
}
impl CreateAutomation {
    /// Create draft parameters.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
/// Automation resource handle.
#[derive(Debug, Clone, Copy)]
pub struct Automations<'a> {
    pub(super) client: &'a MailerLite,
}
impl<'a> Automations<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List automations.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().list().limit(25).send().await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Builder creation does not fail or send a request. The following errors
    /// occur when the request is sent or the stream is polled.
    ///
    /// A zero page or limit returns [`crate::Error::Config`] when the request is sent.  API
    /// rejection returns [`crate::Error::Api`], including server-side validation and account
    /// restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub fn list(&self) -> ListAutomations<'a> {
        ListAutomations::new(self.client, vec!["automations".into()])
    }
    /// Fetch an automation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().get("123").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Automation>> {
        self.client
            .data(
                Method::GET,
                &["automations", id.as_ref()],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Create a disabled draft automation.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automations::CreateAutomation;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().create(CreateAutomation::new("Welcome"))
    ///     .await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// The server validates the name and account permissions. Request serialization can return
    /// [`crate::Error::Serialize`]. API rejection returns [`crate::Error::Api`], including server-
    /// side validation and account restrictions. HTTP execution can also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn create(&self, params: CreateAutomation) -> Result<Response<Automation>> {
        self.client
            .data(
                Method::POST,
                &["automations"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Delete an automation.
    ///
    /// Success is an empty response confirming deletion. This write is never automatically retried.
    /// A timeout or transport failure after dispatch does not establish whether the server applied
    /// it; inspect the result before repeating the operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().delete("123").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<()>> {
        self.client
            .empty(
                Method::DELETE,
                &["automations", id.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Inspect subscriber executions with a required status filter.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automations::ActivityStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().activity("123", ActivityStatus::Completed)
    ///     .limit(25).send().await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Builder creation does not fail or send a request. The following errors
    /// occur when the request is sent or the stream is polled.
    ///
    /// A zero page or limit returns [`crate::Error::Config`] when the request is sent. Empty
    /// identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub fn activity(
        &self,
        id: impl AsRef<str>,
        status: ActivityStatus,
    ) -> ListAutomationActivity<'a> {
        let mut request = ListAutomationActivity::new(
            self.client,
            vec!["automations".into(), id.as_ref().into(), "activity".into()],
        );
        request
            .query
            .push(("filter[status]".into(), status.as_str().into()));
        request
    }
}
/// Parameters for a paginated request.
#[derive(Debug)]
#[must_use = "request builders do nothing until sent; call .send().await"]
pub struct ListAutomations<'a> {
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
}
impl<'a> ListAutomations<'a> {
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
    /// Set enabled.
    pub fn enabled(mut self, value: bool) -> Self {
        self.query.retain(|(key, _)| key != "filter[enabled]");
        self.query
            .push(("filter[enabled]".into(), value.to_string()));
        self
    }
    /// Set name.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "filter[name]");
        self.query.push(("filter[name]".into(), value.into()));
        self
    }
    /// Set group.
    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "filter[group]");
        self.query.push(("filter[group]".into(), value.into()));
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
    /// let response = mailerlite.automations().list().limit(25).send().await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// A zero page or limit returns [`crate::Error::Config`] when the request is sent.  API
    /// rejection returns [`crate::Error::Api`], including server-side validation and account
    /// restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn send(self) -> Result<Response<NumberedPage<Automation>>> {
        validate_query(&self.query)?;
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &self.query, None, OperationPolicy::READ)
            .await
    }
    /// Fetch subsequent pages lazily while preserving request filters.
    ///
    /// Requires the `stream` feature; this example also uses `futures-util`. Requests begin when
    /// polled and preserve filters. Server-provided links are never followed. Each page request has
    /// its own deadline; dropping the stream stops further fetches.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use futures_util::TryStreamExt;
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let resource = mailerlite.automations();
    /// let stream = resource.list().limit(25).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(_item) = stream.try_next().await? {
    ///     // Process this item before requesting the next.
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors are yielded as stream items: validation and HTTP errors are the same as
    /// [`Self::send`]. A reported current page that differs from the requested page yields
    /// [`crate::Error::Pagination`]. The stream terminates after an error; previously yielded items
    /// are not rolled back. Missing continuation metadata ends the stream normally.
    #[cfg(feature = "stream")]
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<Automation>> + 'a {
        crate::pagination::validated_stream(
            validate_query(&self.query),
            crate::pagination::numbered_stream(self.client, self.path, self.query),
        )
    }
}
/// Parameters for a paginated request.
#[derive(Debug)]
#[must_use = "request builders do nothing until sent; call .send().await"]
pub struct ListAutomationActivity<'a> {
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
}
impl<'a> ListAutomationActivity<'a> {
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
    /// Set date from.
    pub fn date_from(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "filter[date_from]");
        self.query.push(("filter[date_from]".into(), value.into()));
        self
    }
    /// Set date to.
    pub fn date_to(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "filter[date_to]");
        self.query.push(("filter[date_to]".into(), value.into()));
        self
    }
    /// Set scheduled from.
    pub fn scheduled_from(mut self, value: impl Into<String>) -> Self {
        self.query
            .retain(|(key, _)| key != "filter[scheduled_from]");
        self.query
            .push(("filter[scheduled_from]".into(), value.into()));
        self
    }
    /// Set scheduled to.
    pub fn scheduled_to(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "filter[scheduled_to]");
        self.query
            .push(("filter[scheduled_to]".into(), value.into()));
        self
    }
    /// Set search.
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "filter[search]");
        self.query.push(("filter[search]".into(), value.into()));
        self
    }
    /// Fetch one page.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automations::ActivityStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automations().activity("123", ActivityStatus::Completed)
    ///     .limit(25).send().await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// A zero page or limit returns [`crate::Error::Config`] when the request is sent. Empty
    /// identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn send(self) -> Result<Response<NumberedPage<AutomationActivity>>> {
        validate_query(&self.query)?;
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &self.query, None, OperationPolicy::READ)
            .await
    }
    /// Fetch subsequent pages lazily while preserving request filters.
    ///
    /// Requires the `stream` feature; this example also uses `futures-util`. Requests begin when
    /// polled and preserve filters. Server-provided links are never followed. Each page request has
    /// its own deadline; dropping the stream stops further fetches.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use futures_util::TryStreamExt;
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automations::ActivityStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let resource = mailerlite.automations();
    /// let stream = resource.activity("123", ActivityStatus::Completed).limit(25).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(_item) = stream.try_next().await? {
    ///     // Process this item before requesting the next.
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors are yielded as stream items: validation and HTTP errors are the same as
    /// [`Self::send`]. A reported current page that differs from the requested page yields
    /// [`crate::Error::Pagination`]. The stream terminates after an error; previously yielded items
    /// are not rolled back. Missing continuation metadata ends the stream normally.
    #[cfg(feature = "stream")]
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<AutomationActivity>> + 'a {
        crate::pagination::validated_stream(
            validate_query(&self.query),
            crate::pagination::numbered_stream(self.client, self.path, self.query),
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
