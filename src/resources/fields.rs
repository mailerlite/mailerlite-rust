//! Custom field operations.
use crate::resources::{SortDirection, groups::validate_name, subscribers::set};
use crate::{
    Error, MailerLite, Response, Result,
    http::OperationPolicy,
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
    ///
    /// Only constructs a builder. See [`ListFields::send`] for validation and errors.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.fields().list().limit(25).send().await?;
    ///
    /// println!("{} fields", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListFields<'a> {
        ListFields {
            client: self.client,
            query: vec![],
        }
    }
    /// Create a custom field.
    ///
    /// This write is never automatically retried. A failure after dispatch does not
    /// prove the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::{models::fields::FieldType, resources::fields::CreateField};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.fields()
    ///     .create(CreateField::new("Company", FieldType::Text)).await?;
    ///
    /// println!("{}", response.data.id);
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
    pub async fn create(&self, params: CreateField) -> Result<Response<Field>> {
        validate_name(&params.name)?;
        self.client
            .data(
                Method::POST,
                &["fields"],
                &[],
                Some(&params),
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Rename a custom field without changing its type.
    ///
    /// This write is never automatically retried. A failure after dispatch does not
    /// prove the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::fields::UpdateField;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.fields()
    ///     .update("123456789", UpdateField::new("Updated name")).await?;
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
        params: UpdateField,
    ) -> Result<Response<Field>> {
        validate_name(&params.name)?;
        self.client
            .data(
                Method::PUT,
                &["fields", id.as_ref()],
                &[],
                Some(&params),
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Delete a custom field.
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
    /// mailerlite.fields().delete("123456789").await?;
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
                &["fields", id.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE.with_success_statuses(&[204]),
            )
            .await
    }
}
/// Documented field sort options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldSort {
    /// Sort by name.
    Name,
    /// Sort by field type.
    Type,
}
/// Field list builder.
#[derive(Debug)]
#[must_use = "request builders do nothing until sent; call .send().await"]
pub struct ListFields<'a> {
    client: &'a MailerLite,
    query: Vec<(String, String)>,
}
impl<'a> ListFields<'a> {
    /// Set a positive page size.
    pub fn limit(mut self, value: u64) -> Self {
        set(&mut self.query, "limit", value.to_string());
        self
    }
    /// Set a one-based page number.
    pub fn page(mut self, value: u64) -> Self {
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
    /// Sort by name or type in the selected direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::{MailerLite, resources::{SortDirection, fields::FieldSort}};
    ///
    /// # fn example(mailerlite: &MailerLite) {
    /// let fields = mailerlite.fields().list()
    ///     .sort(FieldSort::Name, SortDirection::Descending);
    /// # let _ = fields;
    /// # }
    /// ```
    pub fn sort(mut self, field: FieldSort, direction: SortDirection) -> Self {
        set(
            &mut self.query,
            "sort",
            format!(
                "{}{}",
                direction.prefix(),
                match field {
                    FieldSort::Name => "name",
                    FieldSort::Type => "type",
                }
            ),
        );
        self
    }
    /// Fetch one page.
    ///
    /// Repeated setters replace earlier values; only the final parameters are validated.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.fields().list().page(1).limit(25).send().await?;
    ///
    /// println!("Next page: {:?}", page.data.next_page());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] if the final page number
    /// or limit is zero.
    /// Rejected requests return [`crate::Error::Api`]. Transport failures, unexpected
    /// response statuses or bodies, and execution deadlines return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn send(self) -> Result<Response<NumberedPage<Field>>> {
        validate_query(&self.query)?;
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
    /// let resource = mailerlite.fields();
    /// let stream = resource.list().limit(25).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(field) = stream.try_next().await? {
    ///     println!("{}", field.id);
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
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<Field>> + 'a {
        crate::pagination::validated_stream(
            validate_query(&self.query),
            crate::pagination::numbered_stream(self.client, vec!["fields".into()], self.query),
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
