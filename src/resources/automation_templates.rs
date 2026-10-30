//! Automation templates and asynchronous workflow creation.
use crate::{
    Error, MailerLite, Response, Result,
    http::OperationPolicy,
    models::{automation_templates::AutomationTemplate, automations::Automation},
    pagination::NumberedPage,
};
use reqwest::Method;

/// Resource handle for automation template operations.
#[derive(Debug, Clone, Copy)]
pub struct AutomationTemplates<'a> {
    client: &'a MailerLite,
}
impl<'a> AutomationTemplates<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// Configure a page of automation templates, ordered by priority then identifier.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_templates().list().limit(25).send().await?;
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
    /// A zero page or limit returns [`crate::Error::Config`] when the request is sent. API
    /// rejection returns [`crate::Error::Api`], including server-side validation and account
    /// restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub fn list(&self) -> ListAutomationTemplates<'a> {
        ListAutomationTemplates {
            client: self.client,
            query: Vec::new(),
        }
    }
    /// Create an inactive automation from a template.
    ///
    /// Workflow creation happens in the background. The response's
    /// `building_in_progress` flag indicates acceptance, not completion. Fetch
    /// the automation until the flag clears before configuring or enabling it.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_templates().select("123").await?;
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
    pub async fn select(&self, template: impl AsRef<str>) -> Result<Response<Automation>> {
        self.client
            .data(
                Method::POST,
                &["automations", "select-template", template.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE,
            )
            .await
    }
}
/// Builder for numbered pages of automation templates.
#[derive(Debug)]
pub struct ListAutomationTemplates<'a> {
    client: &'a MailerLite,
    query: Vec<(String, String)>,
}
impl<'a> ListAutomationTemplates<'a> {
    fn set(mut self, key: &str, value: String) -> Self {
        self.query.retain(|(k, _)| k != key);
        self.query.push((key.into(), value));
        self
    }
    /// Filter templates by type, such as `welcome`.
    pub fn kind(self, value: impl Into<String>) -> Self {
        self.set("filter[type]", value.into())
    }
    /// Filter templates by category identifier.
    pub fn category(self, value: impl Into<String>) -> Self {
        self.set("filter[category]", value.into())
    }
    /// Set the number of templates per page; the server default is 20.
    pub fn limit(self, value: u32) -> Self {
        self.set("limit", value.to_string())
    }
    /// Set the page number, starting at one.
    pub fn page(self, value: u64) -> Self {
        self.set("page", value.to_string())
    }
    /// Fetch one numbered page.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_templates().list().limit(25).send().await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// A zero page or limit returns [`crate::Error::Config`] when the request is sent. API
    /// rejection returns [`crate::Error::Api`], including server-side validation and account
    /// restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn send(self) -> Result<Response<NumberedPage<AutomationTemplate>>> {
        validate(&self.query)?;
        self.client
            .request(
                Method::GET,
                &["automation-templates"],
                &self.query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Fetch pages lazily without following server-provided navigation URLs.
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
    /// let resource = mailerlite.automation_templates();
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
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<AutomationTemplate>> + 'a {
        crate::pagination::validated_stream(
            validate(&self.query),
            crate::pagination::numbered_stream(
                self.client,
                vec!["automation-templates".into()],
                self.query,
            ),
        )
    }
}
fn validate(query: &[(String, String)]) -> Result<()> {
    if query
        .iter()
        .any(|(key, value)| (key == "page" || key == "limit") && value == "0")
    {
        return Err(Error::Config(
            "template page and limit must be greater than zero".into(),
        ));
    }
    Ok(())
}
