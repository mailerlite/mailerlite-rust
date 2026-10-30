//! Manage webhook subscriptions. Receiving and verifying callbacks is application-owned.
use crate::{
    Error, MailerLite, Response, Result,
    http::{OperationPolicy, body},
    models::webhooks::{Webhook, WebhookEvent},
    pagination::NumberedPage,
};
use reqwest::Method;
use serde::Serialize;

/// Webhook subscription operations.
#[derive(Debug, Clone, Copy)]
pub struct Webhooks<'a> {
    client: &'a MailerLite,
}
impl<'a> Webhooks<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List subscriptions, preserving the server's pagination metadata.
    ///
    /// The reference does not document list query parameters; no automatic
    /// traversal is inferred from the example metadata.
    pub async fn list(&self) -> Result<Response<NumberedPage<Webhook>>> {
        self.client
            .request(Method::GET, &["webhooks"], &[], None, OperationPolicy::READ)
            .await
    }
    /// Fetch one subscription.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Webhook>> {
        self.client
            .data(
                Method::GET,
                &["webhooks", id.as_ref()],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Create a subscription without automatic write retries.
    pub async fn create(&self, params: CreateWebhook) -> Result<Response<Webhook>> {
        validate_events(&params.events, params.batchable)?;
        self.client
            .data(
                Method::POST,
                &["webhooks"],
                &[],
                Some(body(&params)?),
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Update selected subscription properties.
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: UpdateWebhook,
    ) -> Result<Response<Webhook>> {
        if let Some(events) = &params.events {
            validate_events(events, params.batchable)?;
        }
        self.client
            .data(
                Method::PUT,
                &["webhooks", id.as_ref()],
                &[],
                Some(body(&params)?),
                OperationPolicy::WRITE.with_success_statuses(&[200]),
            )
            .await
    }
    /// Delete a subscription.
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<()>> {
        self.client
            .empty(
                Method::DELETE,
                &["webhooks", id.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE,
            )
            .await
    }
}
fn validate_events(events: &[WebhookEvent], batchable: Option<bool>) -> Result<()> {
    if events.is_empty()
        || events
            .iter()
            .any(|event| matches!(event, WebhookEvent::Unknown(_)))
    {
        return Err(Error::Config(
            "at least one documented webhook event is required".into(),
        ));
    }
    if events.iter().any(WebhookEvent::requires_batching) && batchable != Some(true) {
        return Err(Error::Config(
            "selected webhook events require batchable=true".into(),
        ));
    }
    Ok(())
}
/// Required callback URL and events for a new webhook.
#[derive(Debug, Clone, Serialize)]
pub struct CreateWebhook {
    url: String,
    events: Vec<WebhookEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batchable: Option<bool>,
}
impl CreateWebhook {
    /// Set required URL and events. Call `batchable(true)` for batched-only events.
    pub fn new(url: impl Into<String>, events: impl IntoIterator<Item = WebhookEvent>) -> Self {
        Self {
            url: url.into(),
            events: events.into_iter().collect(),
            name: None,
            enabled: None,
            batchable: None,
        }
    }
    /// Set a display name.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Enable or disable deliveries.
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Allow grouped event deliveries.
    pub fn batchable(mut self, value: bool) -> Self {
        self.batchable = Some(value);
        self
    }
}
/// Optional changes to an existing webhook.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<Vec<WebhookEvent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batchable: Option<bool>,
}
impl UpdateWebhook {
    /// Begin an update with all fields omitted.
    pub fn new() -> Self {
        Self::default()
    }
    /// Change the callback URL.
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }
    /// Replace the subscribed events.
    pub fn events(mut self, value: impl IntoIterator<Item = WebhookEvent>) -> Self {
        self.events = Some(value.into_iter().collect());
        self
    }
    /// Change the display name.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Enable or disable deliveries.
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Allow grouped event deliveries.
    pub fn batchable(mut self, value: bool) -> Self {
        self.batchable = Some(value);
        self
    }
}
