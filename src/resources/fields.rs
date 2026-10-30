//! Custom field operations.
use crate::resources::{groups::validate_name, subscribers::set};
use crate::{
    Error, MailerLite, Response, Result,
    http::{OperationPolicy, body},
    models::fields::{Field, FieldType},
    pagination::NumberedPage,
};
use reqwest::Method;
use serde::Serialize;
/// Required values for a custom field.
#[derive(Debug, Clone, Serialize)]
pub struct CreateField {
    name: String,
    #[serde(rename = "type")]
    field_type: FieldType,
}
impl CreateField {
    /// Supply a name and its value type.
    pub fn new(name: impl Into<String>, field_type: FieldType) -> Self {
        Self {
            name: name.into(),
            field_type,
        }
    }
}
/// Rename an existing custom field.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateField {
    name: String,
}
impl UpdateField {
    /// Supply the replacement name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
/// Field API handle.
#[derive(Debug, Clone, Copy)]
pub struct Fields<'a> {
    client: &'a MailerLite,
}
impl<'a> Fields<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List custom fields.
    pub fn list(&self) -> ListFields<'a> {
        ListFields {
            client: self.client,
            query: vec![],
            error: None,
        }
    }
    /// Create a custom field.
    pub async fn create(&self, params: CreateField) -> Result<Response<Field>> {
        validate_name(&params.name)?;
        self.client
            .data(
                Method::POST,
                &["fields"],
                &[],
                Some(body(&params)?),
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Rename a custom field without changing its type.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: UpdateField,
    ) -> Result<Response<Field>> {
        validate_name(&params.name)?;
        self.client
            .data(
                Method::PUT,
                &["fields", id.as_ref()],
                &[],
                Some(body(&params)?),
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Delete a custom field.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<()>> {
        self.client
            .empty(
                Method::DELETE,
                &["fields", id.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE.with_success_statuses(&[204]),
            )
            .await
    }
}
/// Documented field sort options.
#[derive(Debug, Clone, Copy)]
pub enum FieldSort {
    /// Sort by name.
    Name,
    /// Sort by field type.
    Type,
}
/// Field list builder.
#[derive(Debug)]
pub struct ListFields<'a> {
    client: &'a MailerLite,
    query: Vec<(String, String)>,
    error: Option<String>,
}
impl<'a> ListFields<'a> {
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
    /// Filter by partial keyword matches.
    pub fn keyword(mut self, value: impl Into<String>) -> Self {
        set(&mut self.query, "filter[keyword]", value.into());
        self
    }
    /// Filter by field type.
    pub fn field_type(mut self, value: FieldType) -> Self {
        set(&mut self.query, "filter[type]", value.as_str().into());
        self
    }
    /// Sort by name or type.
    pub fn sort(mut self, value: FieldSort, descending: bool) -> Self {
        set(
            &mut self.query,
            "sort",
            format!(
                "{}{}",
                if descending { "-" } else { "" },
                match value {
                    FieldSort::Name => "name",
                    FieldSort::Type => "type",
                }
            ),
        );
        self
    }
    /// Fetch one page.
    pub async fn send(self) -> Result<Response<NumberedPage<Field>>> {
        if let Some(e) = self.error {
            return Err(Error::Config(e));
        }
        self.client
            .request(
                Method::GET,
                &["fields"],
                &self.query,
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Lazily fetch fields as pages are consumed.
    #[cfg(feature = "stream")]
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<Field>> + 'a {
        use futures_util::StreamExt;
        let count = if self.error.is_none() { usize::MAX } else { 0 };
        futures_util::stream::iter(self.error.map(Error::Config).into_iter().map(Err)).chain(
            crate::pagination::numbered_stream(self.client, vec!["fields".into()], self.query)
                .take(count),
        )
    }
}
