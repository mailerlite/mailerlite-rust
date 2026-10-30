//! Subscribers: <https://developers.mailerlite.com/api/subscribers>

use std::collections::BTreeMap;

use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize, Serializer};

use crate::{
    CursorPage, MailerLite, Page, Result,
    groups::Group,
    reports::{EcommerceReport, ListReportOrders, ReportOrder, ShopQuery},
    types::{COUNT_QUERY, FieldValue, Id, Timestamp, Total, de, string_enum},
};

string_enum! {
    /// Subscription status.
    pub enum SubscriberStatus {
        /// Receives emails.
        Active => "active",
        /// Opted out.
        Unsubscribed => "unsubscribed",
        /// Has not confirmed a double opt-in yet.
        Unconfirmed => "unconfirmed",
        /// The address bounced.
        Bounced => "bounced",
        /// Marked as junk.
        Junk => "junk",
    }
}

string_enum! {
    /// Kind of entry in a subscriber's activity log.
    pub enum ActivityType {
        /// A campaign was sent to the subscriber.
        CampaignSend => "campaign_send",
        /// An automation email was sent.
        AutomationEmailSent => "automation_email_sent",
        /// An email was opened.
        EmailOpen => "email_open",
        /// A link was clicked.
        LinkClick => "link_click",
        /// An email bounced.
        EmailBounce => "email_bounce",
        /// The subscriber reported spam.
        SpamComplaint => "spam_complaint",
        /// The subscriber unsubscribed.
        Unsubscribed => "unsubscribed",
        /// An email was forwarded.
        EmailForward => "email_forward",
        /// Marketing preferences changed.
        MarketingPreferencesChange => "marketing_preferences_change",
        /// Preference center visited.
        PreferenceCenter => "preference_center",
        /// Added to a group.
        AddedToGroup => "added_to_group",
        /// Subscriber became active.
        Activated => "activated",
        /// Filter only: every popup-form activity.
        PopupForms => "popup_forms",
    }
}

/// A subscriber.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Subscriber {
    /// Subscriber id.
    pub id: Id,
    /// Email address.
    pub email: String,
    /// Subscription status.
    pub status: SubscriberStatus,
    /// Where the subscriber came from (`api`, `import`, `manual`, `subscribe app`, ...).
    #[serde(default)]
    pub source: Option<String>,
    /// Emails sent to the subscriber.
    #[serde(default)]
    pub sent: u64,
    /// Opens.
    #[serde(default)]
    pub opens_count: u64,
    /// Clicks.
    #[serde(default)]
    pub clicks_count: u64,
    /// Open rate (percent).
    #[serde(default, deserialize_with = "de::number")]
    pub open_rate: f64,
    /// Click rate (percent).
    #[serde(default, deserialize_with = "de::number")]
    pub click_rate: f64,
    /// Signup IP address.
    #[serde(default)]
    pub ip_address: Option<String>,
    /// When the subscriber subscribed.
    #[serde(default)]
    pub subscribed_at: Option<Timestamp>,
    /// When the subscriber unsubscribed.
    #[serde(default)]
    pub unsubscribed_at: Option<Timestamp>,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
    /// Custom and default field values keyed by field key (`name`, `last_name`, ...).
    #[serde(default, deserialize_with = "de::null_default")]
    pub fields: BTreeMap<String, FieldValue>,
    /// Groups the subscriber belongs to. Returned by single-subscriber responses (get, create,
    /// update, forget) and by lists requested with [`ListSubscribers::include_groups`]; empty
    /// when the API leaves it out.
    #[serde(default, deserialize_with = "de::null_default")]
    pub groups: Vec<Group>,
    /// When double opt-in was confirmed.
    #[serde(default)]
    pub opted_in_at: Option<Timestamp>,
    /// IP used for double opt-in.
    #[serde(default)]
    pub optin_ip: Option<String>,
    /// Deletion time (on forgotten / deleted subscribers).
    #[serde(default)]
    pub deleted_at: Option<Timestamp>,
    /// When the subscriber's data will be erased (after [`Subscribers::forget`]).
    #[serde(default)]
    pub forget_at: Option<Timestamp>,
    /// When the email address last changed.
    #[serde(default)]
    pub email_changed_at: Option<Timestamp>,
    /// Location derived from the signup IP (returned when forgetting a subscriber).
    #[serde(default)]
    pub location: Option<Location>,
}

impl Subscriber {
    /// Value of the field with `key`, if present and not `null`.
    #[must_use]
    pub fn field(&self, key: &str) -> Option<&FieldValue> {
        self.fields.get(key).filter(|value| !value.is_null())
    }
}

/// Approximate location of a subscriber.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Location {
    /// Location id.
    #[serde(default)]
    pub id: Option<Id>,
    /// City or place name.
    #[serde(default)]
    pub title: Option<String>,
    /// Country.
    #[serde(default)]
    pub country_name: Option<String>,
    /// Continent.
    #[serde(default)]
    pub continent_name: Option<String>,
    /// State or region.
    #[serde(default)]
    pub state_name: Option<String>,
    /// IANA timezone, e.g. `America/New_York`.
    #[serde(default)]
    pub time_zone: Option<String>,
}

/// Result of [`Subscribers::create`], which upserts by email.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Upserted<T> {
    /// The created or updated resource.
    pub data: T,
    /// `true` if a new subscriber was created (`201`), `false` if an existing one was
    /// updated (`200`).
    pub created: bool,
}

/// Query for subscriber listings (also used for group, segment and form subscribers).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListSubscribers {
    /// Only subscribers with this status. Group, segment and form listings default to
    /// `active` on the API side.
    #[serde(rename = "filter[status]", skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriberStatus>,
    /// Page size. API defaults: 25 for subscribers and forms; 50 for group and segment
    /// subscribers (at most 1000 for groups).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor from [`CursorPage::next_cursor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Embed each subscriber's groups (documented for `/subscribers` and group subscribers).
    #[serde(
        rename = "include",
        skip_serializing_if = "std::ops::Not::not",
        serialize_with = "include_groups"
    )]
    pub include_groups: bool,
}

#[allow(clippy::trivially_copy_pass_by_ref)] // signature required by `serialize_with`
fn include_groups<S: Serializer>(_: &bool, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str("groups")
}

impl ListSubscribers {
    /// Default query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Only subscribers with `status`.
    #[must_use]
    pub fn status(mut self, status: SubscriberStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Page size.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Continue from a cursor.
    #[must_use]
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Embed groups in each subscriber.
    #[must_use]
    pub fn include_groups(mut self) -> Self {
        self.include_groups = true;
        self
    }
}

/// Body for [`Subscribers::create`].
///
/// ```
/// use mailerlite_rs::subscribers::{CreateSubscriber, SubscriberStatus};
///
/// let body = CreateSubscriber::new("ann@example.com")
///     .field("name", "Ann")
///     .field("age", 42)
///     .groups(["4243829086487936"])
///     .status(SubscriberStatus::Active);
/// ```
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CreateSubscriber {
    /// Email address (the upsert key).
    pub email: String,
    /// Field values to set. Fields not listed are left unchanged.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub fields: BTreeMap<String, FieldValue>,
    /// Groups to add the subscriber to. Existing memberships are kept.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Id>,
    /// Status to set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriberStatus>,
    /// `yyyy-MM-dd HH:mm:ss`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_at: Option<String>,
    /// Signup IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// `yyyy-MM-dd HH:mm:ss`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opted_in_at: Option<String>,
    /// Double opt-in IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optin_ip: Option<String>,
    /// `yyyy-MM-dd HH:mm:ss`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed_at: Option<String>,
    /// Reactivate a subscriber that previously unsubscribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resubscribe: Option<bool>,
}

impl CreateSubscriber {
    /// A subscriber with `email`.
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            fields: BTreeMap::new(),
            groups: Vec::new(),
            status: None,
            subscribed_at: None,
            ip_address: None,
            opted_in_at: None,
            optin_ip: None,
            unsubscribed_at: None,
            resubscribe: None,
        }
    }

    /// Sets a field value (`name`, `last_name`, or any custom field key).
    #[must_use]
    pub fn field(mut self, key: impl Into<String>, value: impl Into<FieldValue>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }

    /// Adds the subscriber to these groups.
    #[must_use]
    pub fn groups<I: Into<Id>>(mut self, groups: impl IntoIterator<Item = I>) -> Self {
        self.groups.extend(groups.into_iter().map(Into::into));
        self
    }

    /// Sets the status.
    #[must_use]
    pub fn status(mut self, status: SubscriberStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the subscription time (`yyyy-MM-dd HH:mm:ss`).
    #[must_use]
    pub fn subscribed_at(mut self, at: impl Into<String>) -> Self {
        self.subscribed_at = Some(at.into());
        self
    }

    /// Sets the signup IP.
    #[must_use]
    pub fn ip_address(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    /// Sets the opt-in time (`yyyy-MM-dd HH:mm:ss`).
    #[must_use]
    pub fn opted_in_at(mut self, at: impl Into<String>) -> Self {
        self.opted_in_at = Some(at.into());
        self
    }

    /// Sets the opt-in IP.
    #[must_use]
    pub fn optin_ip(mut self, ip: impl Into<String>) -> Self {
        self.optin_ip = Some(ip.into());
        self
    }

    /// Sets the unsubscribe time (`yyyy-MM-dd HH:mm:ss`).
    #[must_use]
    pub fn unsubscribed_at(mut self, at: impl Into<String>) -> Self {
        self.unsubscribed_at = Some(at.into());
        self
    }

    /// Reactivates a previously unsubscribed subscriber.
    #[must_use]
    pub fn resubscribe(mut self, resubscribe: bool) -> Self {
        self.resubscribe = Some(resubscribe);
        self
    }
}

/// Body for [`Subscribers::update`]. Only fields that are set are sent.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UpdateSubscriber {
    /// Field values to set. Fields not listed are left unchanged.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub fields: BTreeMap<String, FieldValue>,
    /// **Replaces** group membership: the subscriber is removed from groups not listed.
    /// `None` leaves membership untouched; `Some(vec![])` removes every group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Id>>,
    /// Status to set. The API does not allow reactivating unsubscribed, bounced or junk
    /// subscribers this way.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriberStatus>,
    /// `yyyy-MM-dd HH:mm:ss`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_at: Option<String>,
    /// Signup IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// `yyyy-MM-dd HH:mm:ss`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opted_in_at: Option<String>,
    /// Double opt-in IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optin_ip: Option<String>,
    /// `yyyy-MM-dd HH:mm:ss`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed_at: Option<String>,
}

impl UpdateSubscriber {
    /// An empty update.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a field value.
    #[must_use]
    pub fn field(mut self, key: impl Into<String>, value: impl Into<FieldValue>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }

    /// Replaces group membership with exactly these groups.
    #[must_use]
    pub fn groups<I: Into<Id>>(mut self, groups: impl IntoIterator<Item = I>) -> Self {
        self.groups = Some(groups.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the status.
    #[must_use]
    pub fn status(mut self, status: SubscriberStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the subscription time (`yyyy-MM-dd HH:mm:ss`).
    #[must_use]
    pub fn subscribed_at(mut self, at: impl Into<String>) -> Self {
        self.subscribed_at = Some(at.into());
        self
    }

    /// Sets the signup IP.
    #[must_use]
    pub fn ip_address(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    /// Sets the opt-in time (`yyyy-MM-dd HH:mm:ss`).
    #[must_use]
    pub fn opted_in_at(mut self, at: impl Into<String>) -> Self {
        self.opted_in_at = Some(at.into());
        self
    }

    /// Sets the opt-in IP.
    #[must_use]
    pub fn optin_ip(mut self, ip: impl Into<String>) -> Self {
        self.optin_ip = Some(ip.into());
        self
    }

    /// Sets the unsubscribe time (`yyyy-MM-dd HH:mm:ss`).
    #[must_use]
    pub fn unsubscribed_at(mut self, at: impl Into<String>) -> Self {
        self.unsubscribed_at = Some(at.into());
        self
    }
}

/// An entry in a subscriber's activity log.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Activity {
    /// Entry id.
    pub id: Id,
    /// What happened.
    pub log_name: ActivityType,
    /// Id of the related object.
    #[serde(default)]
    pub subject_id: Option<Id>,
    /// Type of the related object (`subscribers`, `opens`, ...).
    #[serde(default)]
    pub subject_type: Option<String>,
    /// Event details; their shape depends on [`Activity::log_name`].
    #[serde(default, deserialize_with = "de::object")]
    pub properties: serde_json::Map<String, serde_json::Value>,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
}

/// Query for [`Subscribers::activity`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListActivity {
    /// Only entries of this type. Any [`ActivityType`] works, including the filter-only
    /// [`ActivityType::PopupForms`] group.
    #[serde(rename = "filter[log_name]", skip_serializing_if = "Option::is_none")]
    pub log_name: Option<ActivityType>,
    /// Entries per kind of activity (API default 100, at most 5000): the limit applies to
    /// each kind separately, so a page can hold more than `limit` entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl ListActivity {
    /// Default query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Only entries of `log_name`.
    #[must_use]
    pub fn log_name(mut self, log_name: ActivityType) -> Self {
        self.log_name = Some(log_name);
        self
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
}

/// Response of [`Subscribers::forget`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Forgotten {
    /// Confirmation message.
    #[serde(default)]
    pub message: String,
    /// The subscriber scheduled for erasure.
    #[serde(rename = "data")]
    pub subscriber: Subscriber,
}

/// Progress of a subscriber import.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
pub struct ImportStatus {
    /// Import id.
    pub id: Id,
    /// Rows in the import.
    #[serde(default)]
    pub total: u64,
    /// Rows processed so far.
    #[serde(default)]
    pub processed: u64,
    /// New subscribers.
    #[serde(default)]
    pub imported: u64,
    /// Existing subscribers updated.
    #[serde(default)]
    pub updated: u64,
    /// Rows that failed.
    #[serde(default)]
    pub errored: u64,
    /// Progress in percent, with one decimal while running (e.g. `33.3`).
    #[serde(default, deserialize_with = "de::number")]
    pub percent: f64,
    /// `true` once the import finished.
    #[serde(default, deserialize_with = "de::null_default")]
    pub done: bool,
    /// Sample of invalid rows.
    #[serde(default, deserialize_with = "de::null_default")]
    pub invalid: Vec<ImportSample>,
    /// Number of invalid rows.
    #[serde(default)]
    pub invalid_count: u64,
    /// Sample of rows with likely typos.
    #[serde(default, deserialize_with = "de::null_default")]
    pub mistyped: Vec<ImportSample>,
    /// Number of mistyped rows.
    #[serde(default)]
    pub mistyped_count: u64,
    /// Sample of changed subscribers.
    #[serde(default, deserialize_with = "de::null_default")]
    pub changed: Vec<ImportSample>,
    /// Number of changed subscribers.
    #[serde(default)]
    pub changed_count: u64,
    /// Sample of unchanged subscribers.
    #[serde(default, deserialize_with = "de::null_default")]
    pub unchanged: Vec<ImportSample>,
    /// Number of unchanged subscribers.
    #[serde(default)]
    pub unchanged_count: u64,
    /// Sample of unsubscribed rows.
    #[serde(default, deserialize_with = "de::null_default")]
    pub unsubscribed: Vec<ImportSample>,
    /// Number of unsubscribed rows.
    #[serde(default)]
    pub unsubscribed_count: u64,
    /// Sample of role-based addresses (`info@`, ...).
    #[serde(default, deserialize_with = "de::null_default")]
    pub role_based: Vec<ImportSample>,
    /// Number of role-based rows.
    #[serde(default)]
    pub role_based_count: u64,
    /// Sample of suspicious addresses.
    #[serde(default, deserialize_with = "de::null_default")]
    pub suspicious_format: Vec<ImportSample>,
    /// Number of suspicious rows.
    #[serde(default)]
    pub suspicious_format_count: u64,
    /// Sample of duplicates.
    #[serde(default, deserialize_with = "de::null_default")]
    pub duplicate: Vec<ImportSample>,
    /// Number of duplicates.
    #[serde(default)]
    pub duplicate_count: u64,
    /// Rows rejected because the address is banned.
    #[serde(default)]
    pub banned_import_emails_count: u64,
    /// The account's subscriber limit, when the import is capped by it.
    #[serde(default)]
    pub subscriber_limit: Option<u64>,
    /// Subscribers counted against [`subscriber_limit`](Self::subscriber_limit).
    #[serde(default)]
    pub subscriber_limit_count: Option<u64>,
    /// `true` if the import stopped because a free account hit its subscriber limit.
    #[serde(default, deserialize_with = "de::null_default")]
    pub is_stopped_due_to_free_account_limit_exceeded: bool,
    /// Account the import belongs to.
    #[serde(default)]
    pub account_id: Option<Id>,
    /// Uploaded file path (empty for API imports).
    #[serde(default)]
    pub file_path: Option<String>,
    /// How rows were matched to existing subscribers.
    #[serde(default)]
    pub match_route: Option<String>,
    /// Source label shown in the app.
    #[serde(default)]
    pub source_label: Option<String>,
    /// Last update time (ISO-8601).
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
    /// Completion time (ISO-8601).
    #[serde(default)]
    pub finished_at: Option<Timestamp>,
    /// When the import was undone.
    #[serde(default)]
    pub undone_at: Option<Timestamp>,
    /// When the import was stopped.
    #[serde(default)]
    pub stopped_at: Option<Timestamp>,
    /// When undoing started.
    #[serde(default)]
    pub undo_started_at: Option<Timestamp>,
}

/// A sample row in an [`ImportStatus`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportSample {
    /// Subscriber id, if one exists.
    #[serde(default)]
    pub id: Option<Id>,
    /// Email address.
    #[serde(default)]
    pub email: String,
}

/// Subscriber endpoints. Obtained with [`MailerLite::subscribers`].
#[derive(Clone, Copy, Debug)]
pub struct Subscribers<'a> {
    client: &'a MailerLite,
}

impl MailerLite {
    /// Subscriber endpoints.
    #[must_use]
    pub fn subscribers(&self) -> Subscribers<'_> {
        Subscribers { client: self }
    }
}

impl<'a> Subscribers<'a> {
    /// `GET /subscribers`: one page of subscribers.
    pub async fn list(&self, query: &ListSubscribers) -> Result<CursorPage<Subscriber>> {
        self.client
            .call(Method::GET, &["subscribers"])
            .query(query)
            .send()
            .await
    }

    /// Every subscriber matching `query`, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        &self,
        query: ListSubscribers,
    ) -> impl futures_util::Stream<Item = Result<Subscriber>> + 'a {
        let subscribers = *self;
        crate::pagination::stream::cursored(query, move |query| async move {
            subscribers.list(&query).await
        })
    }

    /// `GET /subscribers?limit=0`: total number of subscribers.
    pub async fn count(&self) -> Result<u64> {
        let total: Total = self
            .client
            .call(Method::GET, &["subscribers"])
            .query(&COUNT_QUERY)
            .send()
            .await?;
        Ok(total.total)
    }

    /// `GET /subscribers/{id_or_email}`: one subscriber, including its groups.
    pub async fn get(&self, id_or_email: impl AsRef<str>) -> Result<Subscriber> {
        self.client
            .call(Method::GET, &["subscribers", id_or_email.as_ref()])
            .data()
            .await
    }

    /// `POST /subscribers`: creates a subscriber, or updates the one with the same email.
    ///
    /// ```no_run
    /// # async fn run(client: mailerlite_rs::MailerLite) -> mailerlite_rs::Result<()> {
    /// use mailerlite_rs::subscribers::CreateSubscriber;
    ///
    /// let upserted = client
    ///     .subscribers()
    ///     .create(&CreateSubscriber::new("ann@example.com").field("name", "Ann"))
    ///     .await?;
    /// if upserted.created {
    ///     println!("welcome {}", upserted.data.email);
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn create(&self, subscriber: &CreateSubscriber) -> Result<Upserted<Subscriber>> {
        let response = self
            .client
            .call(Method::POST, &["subscribers"])
            .json(subscriber)
            .execute()
            .await?;
        Ok(Upserted {
            created: response.status == StatusCode::CREATED,
            data: response.data()?,
        })
    }

    /// `PUT /subscribers/{id}`: updates a subscriber.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        update: &UpdateSubscriber,
    ) -> Result<Subscriber> {
        self.client
            .call(Method::PUT, &["subscribers", id.as_ref()])
            .json(update)
            .data()
            .await
    }

    /// `DELETE /subscribers/{id}`: deletes a subscriber. Their data is kept so they can
    /// resubscribe; use [`Subscribers::forget`] to erase it.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<()> {
        self.client
            .call(Method::DELETE, &["subscribers", id.as_ref()])
            .send_empty()
            .await
    }

    /// `POST /subscribers/{id}/forget`: erases the subscriber's data within 30 days (GDPR).
    pub async fn forget(&self, id: impl AsRef<str>) -> Result<Forgotten> {
        self.client
            .call(Method::POST, &["subscribers", id.as_ref(), "forget"])
            .send()
            .await
    }

    /// `GET /subscribers/{id}/activity-log`: the subscriber's activity.
    ///
    /// The API returns no pagination metadata for this list, so [`Page::next_page`] is always
    /// `None`. Pages after the first repeat entries (the API skips without limiting), so raise
    /// [`ListActivity::limit`] instead of paging.
    pub async fn activity(
        &self,
        id: impl AsRef<str>,
        query: &ListActivity,
    ) -> Result<Page<Activity>> {
        self.client
            .call(Method::GET, &["subscribers", id.as_ref(), "activity-log"])
            .query(query)
            .send()
            .await
    }

    /// `GET /subscribers/import/{import_id}`: progress of an import started with
    /// [`Groups::import_subscribers`](crate::groups::Groups::import_subscribers).
    pub async fn import_status(&self, import_id: impl AsRef<str>) -> Result<ImportStatus> {
        self.client
            .call(Method::GET, &["subscribers", "import", import_id.as_ref()])
            .data()
            .await
    }

    /// `GET /subscribers/{id}/reports/ecommerce`: the subscriber's order totals in a shop.
    pub async fn ecommerce_stats(
        &self,
        id: impl AsRef<str>,
        shop_id: impl AsRef<str>,
    ) -> Result<EcommerceReport> {
        self.client
            .call(
                Method::GET,
                &["subscribers", id.as_ref(), "reports", "ecommerce"],
            )
            .query(&ShopQuery {
                shop_id: shop_id.as_ref(),
            })
            .data()
            .await
    }

    /// `GET /subscribers/{id}/reports/ecommerce/orders`: the subscriber's orders in a shop.
    pub async fn ecommerce_orders(
        &self,
        id: impl AsRef<str>,
        query: &ListReportOrders,
    ) -> Result<Page<ReportOrder>> {
        self.client
            .call(
                Method::GET,
                &["subscribers", id.as_ref(), "reports", "ecommerce", "orders"],
            )
            .query(query)
            .send()
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
    async fn list_with_filters() {
        let (client, mock) = client();
        mock.respond(200, fixture("subscribers_list"));

        let page = client
            .subscribers()
            .list(
                &ListSubscribers::new()
                    .status(SubscriberStatus::Unsubscribed)
                    .limit(10)
                    .cursor("abc")
                    .include_groups(),
            )
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "subscribers?filter%5Bstatus%5D=unsubscribed&limit=10&cursor=abc&include=groups",
        );
        assert_eq!(page.data[0].email, "dummy@example.com");
        assert_eq!(page.data[0].status, SubscriberStatus::Active);
        assert_eq!(
            page.data[0].field("last_name").and_then(FieldValue::as_str),
            Some("Testerson")
        );
        assert!(page.data[0].field("city").is_none());
        assert!(
            page.next_cursor()
                .is_some_and(|cursor| cursor.starts_with("eyJ"))
        );
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn list_all_follows_cursors() {
        use futures_util::TryStreamExt;

        let (client, mock) = client();
        mock.respond(
            200,
            json!({"data": [fixture("subscriber")["data"]], "meta": {"next_cursor": "c2"}}),
        )
        .respond(
            200,
            json!({"data": [fixture("subscriber")["data"]], "meta": {"next_cursor": null}}),
        );

        let all: Vec<Subscriber> = client
            .subscribers()
            .list_all(ListSubscribers::new().limit(1))
            .try_collect()
            .await
            .unwrap();

        assert_eq!(all.len(), 2);
        let requests = mock.requests();
        requests[0].assert(Method::GET, "subscribers?limit=1");
        requests[1].assert(Method::GET, "subscribers?limit=1&cursor=c2");
    }

    #[tokio::test]
    async fn count() {
        let (client, mock) = client();
        mock.respond(200, json!({"total": 100}));

        assert_eq!(client.subscribers().count().await.unwrap(), 100);
        mock.request().assert(Method::GET, "subscribers?limit=0");
    }

    #[tokio::test]
    async fn get_by_email_is_encoded() {
        let (client, mock) = client();
        mock.respond(200, fixture("subscriber"));

        let subscriber = client.subscribers().get("a b@example.com").await.unwrap();

        mock.request()
            .assert(Method::GET, "subscribers/a%20b@example.com");
        assert_eq!(subscriber.id, "31897397363737859");
        assert!(subscriber.groups.is_empty());
    }

    #[tokio::test]
    async fn create_reports_whether_created() {
        let (client, mock) = client();
        mock.respond(201, fixture("subscriber"))
            .respond(200, fixture("subscriber"));
        let body = CreateSubscriber::new("ann@example.com")
            .field("name", "Ann")
            .field("score", 7)
            .groups(["1", "2"])
            .status(SubscriberStatus::Active)
            .resubscribe(true);

        let created = client.subscribers().create(&body).await.unwrap();
        let updated = client.subscribers().create(&body).await.unwrap();

        assert!(created.created);
        assert!(!updated.created);
        mock.requests()[0]
            .assert(Method::POST, "subscribers")
            .body(&json!({
                "email": "ann@example.com",
                "fields": {"name": "Ann", "score": 7},
                "groups": ["1", "2"],
                "status": "active",
                "resubscribe": true
            }));
    }

    #[tokio::test]
    async fn update_can_clear_groups() {
        let (client, mock) = client();
        mock.respond(200, fixture("subscriber"));

        client
            .subscribers()
            .update(
                "1",
                &UpdateSubscriber::new()
                    .field("name", None::<&str>)
                    .groups(Vec::<Id>::new()),
            )
            .await
            .unwrap();

        mock.request()
            .assert(Method::PUT, "subscribers/1")
            .body(&json!({"fields": {"name": null}, "groups": []}));
    }

    #[tokio::test]
    async fn delete_and_forget() {
        let (client, mock) = client();
        mock.respond(204, Value::Null)
            .respond(200, fixture("subscriber_forget"));

        client.subscribers().delete("1").await.unwrap();
        let forgotten = client.subscribers().forget("1").await.unwrap();

        let requests = mock.requests();
        requests[0]
            .assert(Method::DELETE, "subscribers/1")
            .no_body();
        requests[1].assert(Method::POST, "subscribers/1/forget");
        assert!(forgotten.message.contains("forgotten"));
        assert_eq!(forgotten.subscriber.groups[0].name, "sint incidunt");
        assert_eq!(
            forgotten.subscriber.forget_at.unwrap().as_str(),
            "2023-05-19 05:40:50"
        );
        let location = forgotten.subscriber.location.unwrap();
        assert_eq!(location.time_zone.as_deref(), Some("America/New_York"));
        assert_eq!(forgotten.subscriber.email_changed_at, None);
    }

    #[tokio::test]
    async fn activity_handles_empty_properties() {
        let (client, mock) = client();
        mock.respond(200, fixture("subscriber_activity"));

        let page = client
            .subscribers()
            .activity(
                "1",
                &ListActivity::new()
                    .log_name(ActivityType::EmailOpen)
                    .page(2),
            )
            .await
            .unwrap();

        mock.request().assert(
            Method::GET,
            "subscribers/1/activity-log?filter%5Blog_name%5D=email_open&page=2",
        );
        assert_eq!(page.data[0].log_name, ActivityType::AddedToGroup);
        assert!(page.data[1].properties.is_empty());
        assert_eq!(page.data[2].properties["campaign_name"], "Example name");
        assert_eq!(page.next_page(), None);
    }

    #[tokio::test]
    async fn import_status() {
        let (client, mock) = client();
        mock.respond(200, fixture("subscriber_import"));

        let status = client
            .subscribers()
            .import_status("47366063630845932")
            .await
            .unwrap();

        mock.request()
            .assert(Method::GET, "subscribers/import/47366063630845932");
        assert!(status.done);
        assert_eq!(status.unchanged_count, 3);
        assert_eq!(status.file_path.as_deref(), Some(""));
        assert_eq!(status.source_label.as_deref(), Some(""));
        assert_eq!(status.unchanged[1].email, "dummy_two+two@import.com");
    }

    #[tokio::test]
    async fn ecommerce_reports() {
        let (client, mock) = client();
        mock.respond(200, fixture("subscriber_ecommerce_stats"))
            .respond(200, fixture("subscriber_ecommerce_orders"));

        let stats = client
            .subscribers()
            .ecommerce_stats("1", "9")
            .await
            .unwrap();
        let orders = client
            .subscribers()
            .ecommerce_orders("1", &ListReportOrders::new("9").limit(5).page(2))
            .await
            .unwrap();

        let requests = mock.requests();
        requests[0].assert(Method::GET, "subscribers/1/reports/ecommerce?shop_id=9");
        requests[1].assert(
            Method::GET,
            "subscribers/1/reports/ecommerce/orders?shop_id=9&limit=5&page=2",
        );
        assert_eq!(stats.ecommerce.orders_count, 2);
        assert!((stats.ecommerce.revenue - 40.0).abs() < f64::EPSILON);
        assert_eq!(orders.data[0].order, "2 X T-shirt, 1 X Mug");
        assert_eq!(orders.meta.total, 1);
    }
}
