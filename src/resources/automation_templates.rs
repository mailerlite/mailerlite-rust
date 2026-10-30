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
