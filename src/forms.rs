//! Forms: <https://developers.mailerlite.com/api/forms>
//!
//! Forms are built in the MailerLite app; the API can list, inspect, rename and delete them
//! and list the subscribers who signed up through them.

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{
    CursorPage, MailerLite, Page, Result,
    subscribers::{ListSubscribers, Subscriber},
    types::{Id, NameBody, Rate, Sort, Timestamp, de, string_enum},
};

string_enum! {
    /// Kind of form.
    pub enum FormType {
        /// Pop-up form.
        Popup => "popup",
        /// Embedded form.
        Embedded => "embedded",
        /// Promotion pop-up.
        Promotion => "promotion",
    }
}

/// A signup form.
#[allow(clippy::struct_excessive_bools)] // mirrors the API's flags
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Form {
    /// Form id.
    pub id: Id,
    /// Kind of form.
    #[serde(rename = "type")]
    pub form_type: FormType,
    /// Public slug, e.g. `Ec7t3n`.
    #[serde(default)]
    pub slug: String,
    /// Form name.
    pub name: String,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Signups.
    #[serde(default)]
    pub conversions_count: u64,
    /// Views.
    #[serde(default)]
    pub opens_count: u64,
    /// Signups per view.
    #[serde(default)]
    pub conversion_rate: Rate,
    /// Form settings (empty when the API returns `[]`).
    #[serde(default, deserialize_with = "de::object")]
    pub settings: Map<String, Value>,
    /// Time of the last signup.
    #[serde(default)]
    pub last_registration_at: Option<Timestamp>,
    /// Whether the form is published.
    #[serde(default, deserialize_with = "de::null_default")]
    pub active: bool,
    /// Whether MailerLite flagged the form as broken.
    #[serde(default, deserialize_with = "de::null_default")]
    pub is_broken: bool,
    /// Whether the form has been designed.
    #[serde(default, deserialize_with = "de::null_default")]
    pub has_content: bool,
    /// What the current API key may do with the form.
    #[serde(default)]
    pub can: FormPermissions,
    /// Whether an automation is triggered by the form.
    #[serde(default, deserialize_with = "de::null_default")]
    pub used_in_automations: bool,
    /// Configuration warnings shown in the app.
    #[serde(default, deserialize_with = "de::null_default")]
    pub warnings: Vec<Value>,
    /// Whether signups must confirm by email (double opt-in).
    #[serde(default)]
    pub double_optin: Option<bool>,
    /// Preview image URL.
    #[serde(default)]
    pub screenshot_url: Option<String>,
}

/// Permissions on a [`Form`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FormPermissions {
    /// The form can be updated.
    #[serde(default, deserialize_with = "de::null_default")]
    pub update: bool,
}

/// Number of forms of each type, attached to [`Forms::list`] responses.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FormAggregations {
    /// Pop-up forms.
    #[serde(default)]
    pub popup: u64,
    /// Embedded forms.
    #[serde(default)]
    pub embedded: u64,
    /// Promotion pop-ups.
    #[serde(default)]
    pub promotion: u64,
}

string_enum! {
    /// Sort field for [`ListForms`].
    pub enum FormSort {
        /// Creation time.
        CreatedAt => "created_at",
        /// Name.
        Name => "name",
        /// Signups.
        ConversionsCount => "conversions_count",
        /// Views.
        OpensCount => "opens_count",
        /// Visitors.
        Visitors => "visitors",
        /// Conversion rate.
        ConversionRate => "conversion_rate",
        /// Time of the last signup.
        LastRegistrationAt => "last_registration_at",
    }
}

/// Query for [`Forms::list`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListForms {
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Partial name match.
    #[serde(rename = "filter[name]", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort<FormSort>>,
}

impl ListForms {
    /// Default query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Page size.
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

    /// Only forms whose name contains `name`.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sort order, e.g. `Sort::desc(FormSort::ConversionRate)`.
    #[must_use]
    pub fn sort(mut self, sort: Sort<FormSort>) -> Self {
        self.sort = Some(sort);
        self
    }
}

/// Form endpoints. Obtained with [`MailerLite::forms`].
#[derive(Clone, Copy, Debug)]
pub struct Forms<'a> {
    client: &'a MailerLite,
}

impl MailerLite {
    /// Form endpoints.
    #[must_use]
    pub fn forms(&self) -> Forms<'_> {
        Forms { client: self }
    }
}

impl<'a> Forms<'a> {
    /// `GET /forms/{type}`: one page of forms of `form_type`, with counts per type in
    /// [`Page::aggregations`].
    pub async fn list(
        &self,
        form_type: &FormType,
        query: &ListForms,
    ) -> Result<Page<Form, FormAggregations>> {
        self.client
            .call(Method::GET, &["forms", form_type.as_str()])
            .query(query)
            .send()
            .await
    }

    /// Every form of `form_type` matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        &self,
        form_type: FormType,
        query: ListForms,
    ) -> impl futures_util::Stream<Item = Result<Form>> + 'a {
        let forms = *self;
        crate::pagination::stream::paged(query, move |query| {
            let form_type = form_type.clone();
            async move { forms.list(&form_type, &query).await }
        })
    }

    /// `GET /forms/{id}`: one form.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Form> {
        self.client
            .call(Method::GET, &["forms", id.as_ref()])
            .data()
            .await
    }

    /// `PUT /forms/{id}`: renames a form (name at most 255 characters).
    pub async fn update(&self, id: impl AsRef<str>, name: &str) -> Result<Form> {
        self.client
            .call(Method::PUT, &["forms", id.as_ref()])
            .json(&NameBody { name })
            .data()
            .await
    }

    /// `DELETE /forms/{id}`: deletes a form.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<()> {
        self.client
            .call(Method::DELETE, &["forms", id.as_ref()])
            .send_empty()
            .await
    }

    /// `GET /forms/{id}/subscribers`: one page of subscribers who signed up through the form
    /// (active by default).
    pub async fn subscribers(
        &self,
        id: impl AsRef<str>,
        query: &ListSubscribers,
    ) -> Result<CursorPage<Subscriber>> {
        self.client
            .call(Method::GET, &["forms", id.as_ref(), "subscribers"])
            .query(query)
            .send()
            .await
    }

    /// Every subscriber of the form matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn subscribers_all(
        &self,
        id: impl Into<Id>,
        query: ListSubscribers,
    ) -> impl futures_util::Stream<Item = Result<Subscriber>> + 'a {
        let forms = *self;
        let id = id.into();
        crate::pagination::stream::cursored(query, move |query| {
            let id = id.clone();
            async move { forms.subscribers(&id, &query).await }
        })
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use serde_json::{Value, json};

    use super::*;
    use crate::{
        http::mock::{client, fixture},
        subscribers::SubscriberStatus,
    };

    fn assert_docs_form(form: &Form) {
        assert_eq!(form.id, "38965245585131492");
        assert_eq!(form.form_type, FormType::Embedded);
        assert_eq!(form.slug, "Ec7t3n");
        assert_eq!(form.name, "inventore provident ut");
        assert_eq!(form.conversion_rate.string, "0%");
        assert!(form.settings.is_empty());
        assert_eq!(form.last_registration_at, None);
        assert!(!form.active);
        assert!(form.can.update);
        assert!(form.warnings.is_empty());
        assert_eq!(form.double_optin, None);
        assert_eq!(form.screenshot_url, None);
    }

    #[tokio::test]
    async fn list_with_filters_sort_and_aggregations() {
        let (client, mock) = client();
        mock.respond(200, fixture("forms_list"));

        let page = client
            .forms()
            .list(
                &FormType::Embedded,
                &ListForms::new()
                    .limit(10)
                    .page(1)
                    .name("signup")
                    .sort(Sort::desc(FormSort::LastRegistrationAt)),
            )
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "forms/embedded?limit=10&page=1&filter%5Bname%5D=signup&sort=-last_registration_at",
        );
        assert_docs_form(&page.data[0]);
        let aggregations = page.aggregations.unwrap();
        assert_eq!(
            (
                aggregations.popup,
                aggregations.embedded,
                aggregations.promotion
            ),
            (3, 1, 1)
        );
    }

    #[tokio::test]
    async fn get() {
        let (client, mock) = client();
        mock.respond(200, fixture("forms_get"));

        let form = client.forms().get("38965245585131492").await.unwrap();

        mock.request()
            .assert(Method::GET, "forms/38965245585131492")
            .no_body();
        assert_docs_form(&form);
    }

    #[tokio::test]
    async fn update() {
        let (client, mock) = client();
        mock.respond(200, fixture("forms_update"));

        let form = client
            .forms()
            .update("38965245585131492", "inventore provident ut")
            .await
            .unwrap();

        mock.request()
            .assert(Method::PUT, "forms/38965245585131492")
            .body(&json!({"name": "inventore provident ut"}));
        assert_docs_form(&form);
    }

    #[tokio::test]
    async fn delete() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);

        client.forms().delete("1").await.unwrap();

        mock.request().assert(Method::DELETE, "forms/1").no_body();
    }

    #[tokio::test]
    async fn subscribers() {
        let (client, mock) = client();
        mock.respond(200, fixture("forms_subscribers"));

        let page = client
            .forms()
            .subscribers(
                "1234567890",
                &ListSubscribers::new()
                    .status(SubscriberStatus::Unconfirmed)
                    .limit(25),
            )
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "forms/1234567890/subscribers?filter%5Bstatus%5D=unconfirmed&limit=25",
        );
        assert_eq!(page.data[0].id, "31986843064993537");
        assert!(page.has_more());
    }

    #[test]
    fn settings_object_is_kept() {
        let mut form = fixture("forms_get")["data"].clone();
        form["settings"] = json!({"double_optin": true});
        let form: Form = serde_json::from_value(form).unwrap();
        assert_eq!(form.settings["double_optin"], true);
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn streams_walk_every_page() {
        use futures_util::TryStreamExt;

        let (client, mock) = client();
        let form = fixture("forms_list")["data"][0].clone();
        let subscriber = fixture("forms_subscribers")["data"][0].clone();
        mock.respond(
            200,
            json!({"data": [form], "meta": {"current_page": 1, "last_page": 2}}),
        )
        .respond(
            200,
            json!({"data": [form], "meta": {"current_page": 2, "last_page": 2}}),
        )
        .respond(
            200,
            json!({"data": [subscriber], "meta": {"next_cursor": "c2"}}),
        )
        .respond(200, json!({"data": [], "meta": {"next_cursor": null}}));

        let forms: Vec<Form> = client
            .forms()
            .list_all(FormType::Popup, ListForms::new())
            .try_collect()
            .await
            .unwrap();
        let subscribers: Vec<Subscriber> = client
            .forms()
            .subscribers_all("9", ListSubscribers::new())
            .try_collect()
            .await
            .unwrap();

        assert_eq!((forms.len(), subscribers.len()), (2, 1));
        let requests = mock.requests();
        requests[0].assert(Method::GET, "forms/popup?page=1");
        requests[1].assert(Method::GET, "forms/popup?page=2");
        requests[2].assert(Method::GET, "forms/9/subscribers");
        requests[3].assert(Method::GET, "forms/9/subscribers?cursor=c2");
    }
}
