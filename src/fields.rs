//! Custom subscriber fields: <https://developers.mailerlite.com/api/fields>

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    MailerLite, Page, Result,
    types::{Id, NameBody, Sort, string_enum},
};

string_enum! {
    /// Data type of a custom field.
    pub enum FieldType {
        /// Free text.
        Text => "text",
        /// Number.
        Number => "number",
        /// Date.
        Date => "date",
    }
}

/// A custom subscriber field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Field {
    /// Field id.
    pub id: Id,
    /// Display name.
    pub name: String,
    /// Key used in subscriber `fields` maps, e.g. `last_name`.
    pub key: String,
    /// Data type.
    #[serde(rename = "type")]
    pub field_type: FieldType,
}

string_enum! {
    /// Sort field for [`ListFields`].
    pub enum FieldSort {
        /// Name.
        Name => "name",
        /// Type.
        Type => "type",
    }
}

/// Query for [`Fields::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListFields {
    /// Page size (at most 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Partial name match.
    #[serde(rename = "filter[keyword]", skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// Only fields of this type.
    #[serde(rename = "filter[type]", skip_serializing_if = "Option::is_none")]
    pub field_type: Option<FieldType>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort<FieldSort>>,
}

impl ListFields {
    /// Default query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Page size (at most 100).
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// 1-based page number.
    #[must_use]
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Only fields whose name contains `keyword`.
    #[must_use]
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword = Some(keyword.into());
        self
    }

    /// Only fields of `field_type`.
    #[must_use]
    pub fn field_type(mut self, field_type: FieldType) -> Self {
        self.field_type = Some(field_type);
        self
    }

    /// Sort order, e.g. `Sort::desc(FieldSort::Name)`.
    #[must_use]
    pub fn sort(mut self, sort: Sort<FieldSort>) -> Self {
        self.sort = Some(sort);
        self
    }
}

#[derive(Serialize)]
struct CreateBody<'a> {
    name: &'a str,
    #[serde(rename = "type")]
    field_type: &'a FieldType,
}

/// Field endpoints. Obtained with [`MailerLite::fields`].
#[derive(Clone, Copy, Debug)]
pub struct Fields<'a> {
    client: &'a MailerLite,
}

impl MailerLite {
    /// Custom field endpoints.
    #[must_use]
    pub fn fields(&self) -> Fields<'_> {
        Fields { client: self }
    }
}

impl<'a> Fields<'a> {
    /// `GET /fields`: one page of fields.
    pub async fn list(&self, query: &ListFields) -> Result<Page<Field>> {
        self.client
            .call(Method::GET, &["fields"])
            .query(query)
            .send()
            .await
    }

    /// Every field matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        &self,
        query: ListFields,
    ) -> impl futures_util::Stream<Item = Result<Field>> + 'a {
        let fields = *self;
        crate::pagination::stream::paged(
            query,
            move |query| async move { fields.list(&query).await },
        )
    }

    /// `POST /fields`: creates a field (name at most 255 characters).
    pub async fn create(&self, name: &str, field_type: FieldType) -> Result<Field> {
        self.client
            .call(Method::POST, &["fields"])
            .json(&CreateBody {
                name,
                field_type: &field_type,
            })
            .data()
            .await
    }

    /// `PUT /fields/{id}`: renames a field. The type cannot be changed.
    pub async fn update(&self, id: impl AsRef<str>, name: &str) -> Result<Field> {
        self.client
            .call(Method::PUT, &["fields", id.as_ref()])
            .json(&NameBody { name })
            .data()
            .await
    }

    /// `DELETE /fields/{id}`: deletes a field and its values on every subscriber.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<()> {
        self.client
            .call(Method::DELETE, &["fields", id.as_ref()])
            .send_empty()
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::{Value, json};

    use super::*;
    use crate::http::mock::{client, fixture};

    #[tokio::test]
    async fn list_with_filters_and_sort() {
        let (client, mock) = client();
        mock.respond(200, fixture("fields_list"));

        let page = client
            .fields()
            .list(
                &ListFields::new()
                    .limit(100)
                    .page(1)
                    .keyword("name")
                    .field_type(FieldType::Date)
                    .sort(Sort::desc(FieldSort::Type)),
            )
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "fields?limit=100&page=1&filter%5Bkeyword%5D=name&filter%5Btype%5D=date&sort=-type",
        );
        let field = &page.data[0];
        assert_eq!(field.id, "1");
        assert_eq!(field.key, "new_field");
        assert_eq!(field.field_type, FieldType::Text);
        assert_eq!(page.meta.per_page, 100);
    }

    #[tokio::test]
    async fn create() {
        let (client, mock) = client();
        mock.respond(200, fixture("fields_create"));

        let field = client
            .fields()
            .create("new field", FieldType::Number)
            .await
            .unwrap();

        mock.request()
            .assert(Method::POST, "fields")
            .body(&json!({"name": "new field", "type": "number"}));
        assert_eq!(field.name, "new field");
        assert_eq!(field.field_type, FieldType::Text);
    }

    #[tokio::test]
    async fn update() {
        let (client, mock) = client();
        mock.respond(200, fixture("fields_update"));

        let field = client.fields().update("1", "new field").await.unwrap();

        mock.request()
            .assert(Method::PUT, "fields/1")
            .body(&json!({"name": "new field"}));
        assert_eq!(field.key, "new_field");
    }

    #[tokio::test]
    async fn delete() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);

        client.fields().delete("1").await.unwrap();

        mock.request().assert(Method::DELETE, "fields/1").no_body();
    }

    #[test]
    fn unknown_field_type_is_kept() {
        let field: Field = serde_json::from_value(
            json!({"id": "2", "name": "Tags", "key": "tags", "type": "multiselect"}),
        )
        .unwrap();
        assert_eq!(field.field_type, FieldType::Other("multiselect".into()));
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn list_all_walks_pages() {
        use futures_util::TryStreamExt;

        let (client, mock) = client();
        let field = fixture("fields_list")["data"][0].clone();
        mock.respond(
            200,
            json!({"data": [field], "meta": {"current_page": 1, "last_page": 2}}),
        )
        .respond(
            200,
            json!({"data": [field], "meta": {"current_page": 2, "last_page": 2}}),
        );

        let all: Vec<Field> = client
            .fields()
            .list_all(ListFields::new())
            .try_collect()
            .await
            .unwrap();

        assert_eq!(all.len(), 2);
        let requests = mock.requests();
        requests[0].assert(Method::GET, "fields?page=1");
        requests[1].assert(Method::GET, "fields?page=2");
    }
}
