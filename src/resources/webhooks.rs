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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.webhooks().list().await?;
    ///
    /// for webhook in response.data.data {
    ///     println!("Subscription ID: {}", webhook.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Server rejection returns [`crate::Error::Api`]. HTTP execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn list(&self) -> Result<Response<NumberedPage<Webhook>>> {
        self.client
            .request(Method::GET, &["webhooks"], &[], None, OperationPolicy::READ)
            .await
    }
    /// Fetch one subscription.
    ///
    /// Signing secrets in the returned model are redacted in debug output. Expose a
    /// secret only when configuring a trusted receiver; do not print it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.webhooks().get("123456789").await?;
    ///
    /// println!("Subscription ID: {}", response.data.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// An empty or dot (`.` or `..`) identifier returns [`crate::Error::Config`]
    /// before dispatch. Server rejection returns [`crate::Error::Api`]. HTTP execution
    /// may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
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
    ///
    /// Deliveries go to your callback URL; this does not install an HTTP handler.
    /// Use [`crate::webhooks`] for incoming payload types and optional verification.
    /// Writes are never automatically replayed. A timeout or transport failure does
    /// not prove that the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::{models::webhooks::WebhookEvent, resources::webhooks::CreateWebhook};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let params = CreateWebhook::new(
    ///     "https://example.com/webhooks/mailerlite",
    ///     [WebhookEvent::SubscriberCreated],
    /// );
    ///
    /// let response = mailerlite.webhooks().create(params).await?;
    ///
    /// println!("Subscription ID: {}", response.data.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] for no events, unknown event values, or batched-only
    /// events without `batchable(true)`. URL and account rules are server-validated.
    /// Serialization failures return [`Error::Serialize`]. Server rejection returns
    /// [`crate::Error::Api`]. HTTP execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
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
    ///
    /// Omitted properties are unchanged. Supplying events replaces the subscription's
    /// event selection. This does not verify incoming deliveries. Writes are never
    /// automatically replayed. A timeout or transport failure does
    /// not prove that the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::webhooks::UpdateWebhook;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let params = UpdateWebhook::new().name("Order notifications").enabled(false);
    ///
    /// let response = mailerlite.webhooks().update("123456789", params).await?;
    ///
    /// println!("Updated {}", response.data.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// An empty or dot (`.` or `..`) identifier returns [`crate::Error::Config`]
    /// before dispatch. When events are supplied, an empty/unknown event list or
    /// batched-only events
    /// without `batchable(true)` returns [`Error::Config`]. Omitted events are not fetched
    /// for local validation; the server checks changes against the existing subscription.
    /// Serialization failures return [`Error::Serialize`]. Server rejection returns
    /// [`crate::Error::Api`]. HTTP execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
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
    ///
    /// Removes the subscription. Handling deliveries already in flight remains the
    /// receiver's responsibility. Writes are never automatically replayed. A timeout or
    /// transport failure does
    /// not prove that the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// mailerlite.webhooks().delete("123456789").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// An empty or dot (`.` or `..`) identifier returns [`crate::Error::Config`]
    /// before dispatch. An unexpected success status or a nonempty success body returns
    /// [`Error::Decode`]. Server rejection returns [`crate::Error::Api`]. HTTP
    /// execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
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
