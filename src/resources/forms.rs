//! Forms and their subscriber lists.
use crate::models::{forms::Form, subscribers::Subscriber};
use crate::{
    Error, MailerLite, Response, Result,
    http::{OperationPolicy, body},
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
    pub fn list(&self, kind: FormType) -> ListForms<'a> {
        ListForms::new(self.client, vec!["forms".into(), kind.as_str().into()])
    }
    /// Fetch a form.
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
    pub async fn update(&self, id: impl AsRef<str>, params: UpdateForm) -> Result<Response<Form>> {
        self.client
            .data(
                Method::PUT,
                &["forms", id.as_ref()],
                &[],
                Some(body(&params)?),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Delete a form.
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
    pub async fn send(self) -> Result<Response<NumberedPage<Form>>> {
        validate_query(&self.query)?;
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &self.query, None, OperationPolicy::READ)
            .await
    }
    /// Fetch subsequent pages lazily while preserving request filters.
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
    pub async fn send(self) -> Result<Response<CursorPage<Subscriber>>> {
        validate_query(&self.query)?;
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &self.query, None, OperationPolicy::READ)
            .await
    }
    /// Fetch subsequent pages lazily while preserving request filters.
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
