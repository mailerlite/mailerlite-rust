//! Webhooks: <https://developers.mailerlite.com/api/webhooks>
//!
//! These endpoints manage webhook subscriptions. MailerLite signs every delivery with the
//! webhook's [`secret`](Webhook::secret) in the `Signature` header; check it with
//! `webhook_signature::verify` (feature `webhook-signature`) before trusting a payload. The
//! SDK does not model incoming payloads: decode them into your own types or
//! `serde_json::Value`.
//!
//! Deliveries are retried three times (after 10, 100 and 1000 seconds) unless your endpoint
//! answers `2xx` within 3 seconds.

use std::fmt;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    MailerLite, Page, Result,
    types::{Id, Timestamp, de, string_enum},
};

string_enum! {
    /// An event a webhook can subscribe to.
    ///
    /// [`SubscriberDeleted`](Self::SubscriberDeleted), [`CampaignOpen`](Self::CampaignOpen)
    /// and [`CampaignClick`](Self::CampaignClick) require `batchable: true`.
    pub enum WebhookEvent {
        /// A subscriber was added to the account.
        SubscriberCreated => "subscriber.created",
        /// A subscriber's fields changed or they confirmed their subscription.
        SubscriberUpdated => "subscriber.updated",
        /// A subscriber unsubscribed.
        SubscriberUnsubscribed => "subscriber.unsubscribed",
        /// A subscriber was added to a group.
        SubscriberAddedToGroup => "subscriber.added_to_group",
        /// A subscriber was removed from a group.
        SubscriberRemovedFromGroup => "subscriber.removed_from_group",
        /// An email address bounced.
        SubscriberBounced => "subscriber.bounced",
        /// A subscriber started an automation.
        SubscriberAutomationTriggered => "subscriber.automation_triggered",
        /// A subscriber finished an automation.
        SubscriberAutomationCompleted => "subscriber.automation_completed",
        /// A subscriber marked a campaign as spam.
        SubscriberSpamReported => "subscriber.spam_reported",
        /// A subscriber was deleted or forgotten. Requires `batchable: true`.
        SubscriberDeleted => "subscriber.deleted",
        /// A subscriber's status became active.
        SubscriberActive => "subscriber.active",
        /// A campaign was sent.
        CampaignSent => "campaign.sent",
        /// A subscriber opened a campaign. Requires `batchable: true`.
        CampaignOpen => "campaign.open",
        /// A subscriber clicked a link in a campaign. Requires `batchable: true`.
        CampaignClick => "campaign.click",
    }
}

/// A webhook subscription.
///
/// `Debug` output redacts [`secret`](Self::secret), so logging a webhook doesn't leak the
/// key that signs its deliveries.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Webhook {
    /// Webhook id.
    pub id: Id,
    /// Name.
    #[serde(default)]
    pub name: Option<String>,
    /// Delivery URL.
    pub url: String,
    /// Subscribed events.
    #[serde(default)]
    pub events: Vec<WebhookEvent>,
    /// Whether deliveries are enabled.
    #[serde(default, deserialize_with = "de::null_default")]
    pub enabled: bool,
    /// HMAC-SHA256 key used to sign deliveries (the `Signature` header).
    #[serde(default)]
    pub secret: String,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(default)]
    pub updated_at: Option<Timestamp>,
    /// Whether events are delivered in batches.
    #[serde(default, deserialize_with = "de::null_default")]
    pub batchable: bool,
}

/// Body of [`Webhooks::create`].
///
/// Set [`batchable(true)`](Self::batchable) when subscribing to
/// [`WebhookEvent::SubscriberDeleted`], [`WebhookEvent::CampaignOpen`] or
/// [`WebhookEvent::CampaignClick`]. The docs' event table lists all three as requiring it,
/// while the request-body table names only the two campaign events; setting it for all three
/// satisfies both.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CreateWebhook {
    /// Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Events to subscribe to (at least one).
    pub events: Vec<WebhookEvent>,
    /// Delivery URL.
    pub url: String,
    /// Whether deliveries are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Deliver events in batches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batchable: Option<bool>,
}

impl CreateWebhook {
    /// A webhook delivering `events` to `url`.
    pub fn new(url: impl Into<String>, events: impl IntoIterator<Item = WebhookEvent>) -> Self {
        Self {
            name: None,
            events: events.into_iter().collect(),
            url: url.into(),
            enabled: None,
            batchable: None,
        }
    }

    /// Name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Enables or disables deliveries.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Delivers events in batches (required for `subscriber.deleted`, `campaign.open` and
    /// `campaign.click`).
    #[must_use]
    pub fn batchable(mut self, batchable: bool) -> Self {
        self.batchable = Some(batchable);
        self
    }
}

/// Body of [`Webhooks::update`]. Only the fields you set are changed.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct UpdateWebhook {
    /// Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Replaces the subscribed events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WebhookEvent>>,
    /// Delivery URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether deliveries are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Deliver events in batches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batchable: Option<bool>,
}

impl UpdateWebhook {
    /// An update that changes nothing yet.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Renames the webhook.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Replaces the subscribed events.
    #[must_use]
    pub fn events(mut self, events: impl IntoIterator<Item = WebhookEvent>) -> Self {
        self.events = Some(events.into_iter().collect());
        self
    }

    /// Changes the delivery URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Enables or disables deliveries.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Delivers events in batches (required for `subscriber.deleted`, `campaign.open` and
    /// `campaign.click`).
    #[must_use]
    pub fn batchable(mut self, batchable: bool) -> Self {
        self.batchable = Some(batchable);
        self
    }
}

impl fmt::Debug for Webhook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Webhook")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("url", &self.url)
            .field("events", &self.events)
            .field("enabled", &self.enabled)
            .field("secret", &"<redacted>")
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .field("batchable", &self.batchable)
            .finish()
    }
}

/// Query for [`Webhooks::list`]. The docs list no parameters, but the response is paginated
/// and its `links` use `?page=N`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ListWebhooks {
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 1-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl ListWebhooks {
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
}

/// Webhook endpoints. Obtained with [`MailerLite::webhooks`].
#[derive(Clone, Copy, Debug)]
pub struct Webhooks<'a> {
    client: &'a MailerLite,
}

impl MailerLite {
    /// Webhook endpoints.
    #[must_use]
    pub fn webhooks(&self) -> Webhooks<'_> {
        Webhooks { client: self }
    }
}

impl<'a> Webhooks<'a> {
    /// `GET /webhooks`: one page of webhooks.
    pub async fn list(&self, query: &ListWebhooks) -> Result<Page<Webhook>> {
        self.client
            .call(Method::GET, &["webhooks"])
            .query(query)
            .send()
            .await
    }

    /// Every webhook, fetching pages lazily.
    #[cfg(feature = "stream")]
    pub fn list_all(
        &self,
        query: ListWebhooks,
    ) -> impl futures_util::Stream<Item = Result<Webhook>> + 'a {
        let webhooks = *self;
        crate::pagination::stream::paged(
            query,
            move |query| async move { webhooks.list(&query).await },
        )
    }

    /// `GET /webhooks/{id}`: one webhook.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Webhook> {
        self.client
            .call(Method::GET, &["webhooks", id.as_ref()])
            .data()
            .await
    }

    /// `POST /webhooks`: creates a webhook.
    pub async fn create(&self, webhook: &CreateWebhook) -> Result<Webhook> {
        self.client
            .call(Method::POST, &["webhooks"])
            .json(webhook)
            .data()
            .await
    }

    /// `PUT /webhooks/{id}`: updates the fields set in `update`.
    pub async fn update(&self, id: impl AsRef<str>, update: &UpdateWebhook) -> Result<Webhook> {
        self.client
            .call(Method::PUT, &["webhooks", id.as_ref()])
            .json(update)
            .data()
            .await
    }

    /// `DELETE /webhooks/{id}`: deletes a webhook.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<()> {
        self.client
            .call(Method::DELETE, &["webhooks", id.as_ref()])
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

    fn assert_docs_webhook(webhook: &Webhook) {
        assert_eq!(webhook.id, "68074372004316503");
        assert_eq!(
            webhook.name.as_deref(),
            Some("Tenetur delectus eum fugiat.")
        );
        assert_eq!(webhook.events, [WebhookEvent::SubscriberBounced]);
        assert!(webhook.enabled);
        assert_eq!(webhook.secret, "4jQ3Y4UlLI");
        assert!(!webhook.batchable);
    }

    #[tokio::test]
    async fn list() {
        let (client, mock) = client();
        mock.respond(200, fixture("webhooks_list"));

        let page = client
            .webhooks()
            .list(&ListWebhooks::new().limit(50).page(2))
            .await
            .unwrap();

        mock.request()
            .assert(Method::GET, "webhooks?limit=50&page=2")
            .no_body();
        assert_eq!(page.data.len(), 17);
        assert_eq!(page.data[13].events, [WebhookEvent::CampaignOpen]);
        assert_eq!(page.data[14].events, [WebhookEvent::CampaignClick]);
        assert_eq!(page.data[15].events, [WebhookEvent::SubscriberDeleted]);
        assert_eq!(page.data[16].events, [WebhookEvent::SubscriberActive]);
        assert_eq!(page.meta.per_page, 50);
        assert_eq!(page.next_page(), None);
    }

    #[tokio::test]
    async fn get() {
        let (client, mock) = client();
        mock.respond(200, fixture("webhooks_get"));

        let webhook = client.webhooks().get("68074372004316503").await.unwrap();

        mock.request()
            .assert(Method::GET, "webhooks/68074372004316503")
            .no_body();
        assert_docs_webhook(&webhook);
    }

    #[tokio::test]
    async fn create() {
        let (client, mock) = client();
        mock.respond(200, fixture("webhooks_create"));

        let webhook = client
            .webhooks()
            .create(
                &CreateWebhook::new(
                    "https://example.com/hook",
                    [WebhookEvent::SubscriberCreated, WebhookEvent::CampaignOpen],
                )
                .name("Opens")
                .enabled(true)
                .batchable(true),
            )
            .await
            .unwrap();

        mock.request()
            .assert(Method::POST, "webhooks")
            .body(&json!({
                "name": "Opens",
                "events": ["subscriber.created", "campaign.open"],
                "url": "https://example.com/hook",
                "enabled": true,
                "batchable": true
            }));
        assert_docs_webhook(&webhook);
    }

    #[tokio::test]
    async fn create_minimal() {
        let (client, mock) = client();
        mock.respond(200, fixture("webhooks_create"));

        client
            .webhooks()
            .create(&CreateWebhook::new(
                "https://example.com/hook",
                [WebhookEvent::SubscriberBounced],
            ))
            .await
            .unwrap();

        mock.request()
            .assert(Method::POST, "webhooks")
            .body(&json!({"events": ["subscriber.bounced"], "url": "https://example.com/hook"}));
    }

    #[tokio::test]
    async fn update() {
        let (client, mock) = client();
        mock.respond(200, fixture("webhooks_update"));

        let webhook = client
            .webhooks()
            .update(
                "68074372004316503",
                &UpdateWebhook::new()
                    .events([WebhookEvent::SubscriberBounced])
                    .url("https://example.com/new")
                    .enabled(false),
            )
            .await
            .unwrap();

        mock.request()
            .assert(Method::PUT, "webhooks/68074372004316503")
            .body(&json!({
                "events": ["subscriber.bounced"],
                "url": "https://example.com/new",
                "enabled": false
            }));
        assert_docs_webhook(&webhook);
    }

    #[tokio::test]
    async fn delete() {
        let (client, mock) = client();
        mock.respond(204, Value::Null);

        client.webhooks().delete("1").await.unwrap();

        mock.request()
            .assert(Method::DELETE, "webhooks/1")
            .no_body();
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn list_all_walks_pages() {
        use futures_util::TryStreamExt;

        let (client, mock) = client();
        let webhook = fixture("webhooks_get")["data"].clone();
        mock.respond(
            200,
            json!({"data": [webhook], "meta": {"current_page": 1, "last_page": 2}}),
        )
        .respond(
            200,
            json!({"data": [webhook], "meta": {"current_page": 2, "last_page": 2}}),
        );

        let all: Vec<Webhook> = client
            .webhooks()
            .list_all(ListWebhooks::new())
            .try_collect()
            .await
            .unwrap();

        assert_eq!(all.len(), 2);
        let requests = mock.requests();
        requests[0].assert(Method::GET, "webhooks?page=1");
        requests[1].assert(Method::GET, "webhooks?page=2");
    }

    #[test]
    fn debug_redacts_secret() {
        let webhook: Webhook =
            serde_json::from_value(fixture("webhooks_get")["data"].clone()).unwrap();
        assert!(!webhook.secret.is_empty());

        let debug = format!("{webhook:?}");
        assert!(!debug.contains(&webhook.secret));
        assert!(debug.contains("<redacted>"));
    }
}
