//! Automation triggers and execution counts, verified against the application API.
use crate::models::automation_configuration::{
    TRIGGER_FIELDS, TriggerConfiguration, configuration_fields,
};
use crate::{
    Error, MailerLite, Response, Result,
    http::OperationPolicy,
    models::automation_steps::StepSubscriber,
    models::automation_triggers::{AllTriggerRunCounts, Trigger, TriggerRunCounts, UpdatedTrigger},
    pagination::NumberedPage,
};
use reqwest::Method;
use serde_json::{Map, Value};

/// Parameters for creating a trigger. Omit configuration to create an empty trigger.
#[derive(Clone, serde::Serialize)]
pub struct CreateAutomationTrigger {
    #[serde(rename = "type")]
    kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Map<String, Value>>,
}
impl CreateAutomationTrigger {
    /// Specify the trigger type, such as `subscriber_joins_group`.
    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            data: None,
        }
    }
    /// Configure included groups using string-preserved identifiers.
    pub fn group_ids(self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.insert_setting(
            "group_ids",
            Value::Array(ids.into_iter().map(|id| Value::String(id.into())).collect()),
        )
    }
    /// Configure groups whose members are excluded.
    pub fn exclude_group_ids(self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.insert_setting(
            "exclude_group_ids",
            Value::Array(ids.into_iter().map(|id| Value::String(id.into())).collect()),
        )
    }
    /// Construct a trigger with typed configuration and the matching discriminator.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_triggers::CreateAutomationTrigger;
    /// use mailerlite_rs::models::automation_configuration::{
    ///     GroupTriggerConfiguration, TriggerConfiguration,
    /// };
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let configuration = TriggerConfiguration::Group(
    ///     GroupTriggerConfiguration::new(["789"]),
    /// );
    /// let params = CreateAutomationTrigger::configured(configuration)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Serialize`] if serialization fails or [`crate::Error::Config`] if
    /// configuration does not produce an object. The discriminator is selected automatically;
    /// server validation still applies on submission.
    pub fn configured(configuration: TriggerConfiguration) -> Result<Self> {
        Self::new(configuration.kind()).configuration(configuration)
    }
    /// Replace configuration, rejecting a mismatch with this trigger's type.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_triggers::CreateAutomationTrigger;
    /// use mailerlite_rs::models::automation_configuration::{
    ///     GroupTriggerConfiguration, TriggerConfiguration,
    /// };
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let configuration = TriggerConfiguration::Group(
    ///     GroupTriggerConfiguration::new(["789"]),
    /// );
    /// let params = CreateAutomationTrigger::new("subscriber_joins_group").configuration(configuration)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Serialize`] if serialization fails or [`crate::Error::Config`] if
    /// configuration does not produce an object. A discriminator mismatch also returns
    /// `Error::Config`.
    pub fn configuration(mut self, configuration: TriggerConfiguration) -> Result<Self> {
        if self.kind != configuration.kind() {
            return Err(Error::Config(
                "trigger type does not match its configuration".into(),
            ));
        }
        self.data = Some(configuration_fields(&configuration)?);
        Ok(self)
    }
    fn insert_setting(mut self, key: &str, value: Value) -> Self {
        self.data
            .get_or_insert_with(Map::new)
            .insert(key.into(), value);
        self
    }
    /// Add a future configuration key; known keys must use typed configuration.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_triggers::CreateAutomationTrigger;
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let params = CreateAutomationTrigger::new("subscriber_joins_group").extension_setting("future_setting", true)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for a modeled or reserved trigger key. Use typed setters
    /// for known settings. Unknown keys are intended only for future server fields and remain
    /// subject to server validation.
    pub fn extension_setting(
        self,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Self> {
        let key = key.into();
        if TRIGGER_FIELDS.contains(&key.as_str()) {
            return Err(Error::Config(
                "use typed configuration for known trigger fields".into(),
            ));
        }
        Ok(self.insert_setting(&key, value.into()))
    }
}
/// Complete replacement configuration for an existing trigger.
///
/// The trigger type cannot change. Configuration values omitted from `data` are
/// cleared by the server rather than retained. The automation must be inactive.
#[derive(Clone, Default, serde::Serialize)]
pub struct UpdateAutomationTrigger {
    data: Map<String, Value>,
}
impl UpdateAutomationTrigger {
    /// Start a complete replacement configuration with no settings.
    pub fn new() -> Self {
        Self::default()
    }
    /// Configure included groups using string-preserved identifiers.
    pub fn group_ids(self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.insert_setting(
            "group_ids",
            Value::Array(ids.into_iter().map(|id| Value::String(id.into())).collect()),
        )
    }
    /// Configure groups whose members are excluded.
    pub fn exclude_group_ids(self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.insert_setting(
            "exclude_group_ids",
            Value::Array(ids.into_iter().map(|id| Value::String(id.into())).collect()),
        )
    }
    /// Replace data with typed configuration for the existing trigger.
    /// The API validates the existing trigger's discriminator and ownership.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_triggers::UpdateAutomationTrigger;
    /// use mailerlite_rs::models::automation_configuration::{
    ///     GroupTriggerConfiguration, TriggerConfiguration,
    /// };
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let configuration = TriggerConfiguration::Group(
    ///     GroupTriggerConfiguration::new(["789"]),
    /// );
    /// let params = UpdateAutomationTrigger::new().configuration(configuration)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Serialize`] if serialization fails or [`crate::Error::Config`] if
    /// configuration does not produce an object. Compatibility with the existing trigger type is
    /// checked by the server.
    pub fn configuration(mut self, configuration: TriggerConfiguration) -> Result<Self> {
        self.data = configuration_fields(&configuration)?;
        Ok(self)
    }
    fn insert_setting(mut self, key: &str, value: Value) -> Self {
        self.data.insert(key.into(), value);
        self
    }
    /// Add a future field; known keys must use typed configuration.
    ///
    /// These parameters are constructed locally; no API request is made.
    ///
    /// # Examples
    ///
    /// ```
    /// use mailerlite_rs::resources::automation_triggers::UpdateAutomationTrigger;
    ///
    /// # fn main() -> mailerlite_rs::Result<()> {
    /// let params = UpdateAutomationTrigger::new().extension_setting("future_setting", true)?;
    /// # let _ = params;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] for a modeled or reserved trigger key. Use typed setters
    /// for known settings. Unknown keys are intended only for future server fields and remain
    /// subject to server validation.
    pub fn extension_setting(
        self,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Self> {
        let key = key.into();
        if TRIGGER_FIELDS.contains(&key.as_str()) {
            return Err(Error::Config(
                "use typed configuration for known trigger fields".into(),
            ));
        }
        Ok(self.insert_setting(&key, value.into()))
    }
}
/// Execution state accepted by the trigger subscribers endpoint.
#[derive(Debug, Clone, Copy)]
pub enum TriggerSubscriberStatus {
    /// Executions still in progress.
    Queued,
    /// Finished executions.
    Completed,
}
impl TriggerSubscriberStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Completed => "completed",
        }
    }
}
/// Resource handle for automation trigger operations.
#[derive(Debug, Clone, Copy)]
pub struct AutomationTriggers<'a> {
    client: &'a MailerLite,
}
impl<'a> AutomationTriggers<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List all triggers on an automation; this endpoint is not paginated.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_triggers().list("123").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn list(&self, automation: impl AsRef<str>) -> Result<Response<Vec<Trigger>>> {
        self.client
            .data(
                Method::GET,
                &["automations", automation.as_ref(), "triggers"],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Fetch one trigger and its type-specific configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_triggers().get("123", "456").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn get(
        &self,
        automation: impl AsRef<str>,
        trigger: impl AsRef<str>,
    ) -> Result<Response<Trigger>> {
        self.client
            .data(
                Method::GET,
                &[
                    "automations",
                    automation.as_ref(),
                    "triggers",
                    trigger.as_ref(),
                ],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Create a trigger on an inactive automation, subject to its plan's trigger limit.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_triggers::CreateAutomationTrigger;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_triggers().create(
    ///     "123", CreateAutomationTrigger::new("subscriber_joins_group").group_ids(["789"]),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. Request serialization can return [`crate::Error::Serialize`]. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn create(
        &self,
        automation: impl AsRef<str>,
        params: CreateAutomationTrigger,
    ) -> Result<Response<Trigger>> {
        self.client
            .data(
                Method::POST,
                &["automations", automation.as_ref(), "triggers"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Replace a trigger's configuration on an inactive automation.
    ///
    /// Omitted settings are cleared by the server; send the complete desired configuration. The
    /// existing trigger type cannot change. This write is never automatically retried. A timeout or
    /// transport failure after dispatch does not establish whether the server applied it; inspect
    /// the result before repeating the operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_triggers::UpdateAutomationTrigger;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_triggers().update(
    ///     "123", "456", UpdateAutomationTrigger::new().group_ids(["789"]),
    /// ).await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. Request serialization can return [`crate::Error::Serialize`]. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn update(
        &self,
        automation: impl AsRef<str>,
        trigger: impl AsRef<str>,
        params: UpdateAutomationTrigger,
    ) -> Result<Response<UpdatedTrigger>> {
        self.client
            .data(
                Method::PUT,
                &[
                    "automations",
                    automation.as_ref(),
                    "triggers",
                    trigger.as_ref(),
                ],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Remove a trigger from an inactive automation.
    ///
    /// This write is never automatically retried. A timeout or transport failure after dispatch
    /// does not establish whether the server applied it; inspect the result before repeating the
    /// operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_triggers().delete("123", "456").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn delete(
        &self,
        automation: impl AsRef<str>,
        trigger: impl AsRef<str>,
    ) -> Result<Response<()>> {
        self.client
            .empty(
                Method::DELETE,
                &[
                    "automations",
                    automation.as_ref(),
                    "triggers",
                    trigger.as_ref(),
                ],
                &[],
                None,
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Fetch execution counts for all visible triggers and the optional manual trigger.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_triggers().run_counts("123").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn run_counts(
        &self,
        automation: impl AsRef<str>,
    ) -> Result<Response<AllTriggerRunCounts>> {
        self.client
            .data(
                Method::GET,
                &["automations", automation.as_ref(), "triggers", "run-counts"],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Fetch completed and queued execution counts for one trigger.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_triggers().run_count("123", "456").await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Empty path identifiers and dot path segments (`.` or `..`) return [`crate::Error::Config`] before
    /// dispatch. API rejection returns [`crate::Error::Api`], including server-side validation and
    /// account restrictions. HTTP execution can also return [`crate::Error::Transport`],
    /// [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn run_count(
        &self,
        automation: impl AsRef<str>,
        trigger: impl AsRef<str>,
    ) -> Result<Response<TriggerRunCounts>> {
        self.client
            .data(
                Method::GET,
                &[
                    "automations",
                    automation.as_ref(),
                    "triggers",
                    "run-counts",
                    trigger.as_ref(),
                ],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// List subscribers brought into an automation by a trigger, with required status.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_triggers::TriggerSubscriberStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_triggers()
    ///     .subscribers("123", "456", TriggerSubscriberStatus::Completed)
    ///     .limit(25).send().await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Builder creation does not fail or send a request. The following errors
    /// occur when the request is sent or the stream is polled.
    ///
    /// At dispatch, a zero page, a limit outside 1 through 1000, or sort other than `date`,
    /// `-date`, `email`, or `-email` returns [`crate::Error::Config`]. Empty path identifiers and dot
    /// path segments (`.` or `..`) return [`crate::Error::Config`] before dispatch. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub fn subscribers(
        &self,
        automation: impl Into<String>,
        trigger: impl Into<String>,
        status: TriggerSubscriberStatus,
    ) -> TriggerSubscribers<'a> {
        TriggerSubscribers {
            client: self.client,
            path: vec![
                "automations".into(),
                automation.into(),
                "activity".into(),
                "triggers".into(),
                trigger.into(),
                "subscribers".into(),
            ],
            query: vec![("status".into(), status.as_str().into())],
        }
    }
}
/// Builder for paginated subscribers associated with a trigger.
#[derive(Debug)]
#[must_use = "request builders do nothing until sent; call .send().await"]
pub struct TriggerSubscribers<'a> {
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
}
impl<'a> TriggerSubscribers<'a> {
    fn set(mut self, key: &str, value: String) -> Self {
        self.query.retain(|(k, _)| k != key);
        self.query.push((key.into(), value));
        self
    }
    /// Set page size, from 1 through 1000.
    pub fn limit(self, value: u32) -> Self {
        self.set("limit", value.to_string())
    }
    /// Set the page number, starting at one.
    pub fn page(self, value: u64) -> Self {
        self.set("page", value.to_string())
    }
    /// Set sort order: `date` or `email`, optionally prefixed with `-`.
    pub fn sort(self, value: impl Into<String>) -> Self {
        self.set("sort", value.into())
    }
    /// Filter by exact subscriber email.
    pub fn search(self, value: impl Into<String>) -> Self {
        self.set("filter[search]", value.into())
    }
    /// Filter from a date in `YYYY-MM-DD` format.
    pub fn date_from(self, value: impl Into<String>) -> Self {
        self.set("filter[date_from]", value.into())
    }
    /// Filter through a date in `YYYY-MM-DD` format.
    pub fn date_to(self, value: impl Into<String>) -> Self {
        self.set("filter[date_to]", value.into())
    }
    /// Filter execution rows by their returned identifiers.
    pub fn ids(self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.set(
            "filter[id]",
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<String>>()
                .join(","),
        )
    }
    /// Fetch one numbered page.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_triggers::TriggerSubscriberStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.automation_triggers()
    ///     .subscribers("123", "456", TriggerSubscriberStatus::Completed)
    ///     .limit(25).send().await?;
    ///
    /// println!("HTTP {}", response.metadata.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// At dispatch, a zero page, a limit outside 1 through 1000, or sort other than `date`,
    /// `-date`, `email`, or `-email` returns [`crate::Error::Config`]. Empty path identifiers and dot
    /// path segments (`.` or `..`) return [`crate::Error::Config`] before dispatch. API rejection
    /// returns [`crate::Error::Api`], including server-side validation and account restrictions.
    /// HTTP execution can also return [`crate::Error::Transport`], [`crate::Error::Decode`], or
    /// [`crate::Error::Timeout`].
    pub async fn send(self) -> Result<Response<NumberedPage<StepSubscriber>>> {
        validate(&self.query)?;
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &self.query, None, OperationPolicy::READ)
            .await
    }
    /// Fetch pages lazily without following server-provided navigation URLs.
    ///
    /// Requires the `stream` feature; this example also uses `futures-util`. Requests begin when
    /// polled and preserve filters. Server-provided links are never followed. Each page request has
    /// its own deadline; dropping the stream stops further fetches.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use futures_util::TryStreamExt;
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::automation_triggers::TriggerSubscriberStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let resource = mailerlite.automation_triggers();
    /// let stream = resource
    ///     .subscribers("123", "456", TriggerSubscriberStatus::Completed).limit(25).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(_item) = stream.try_next().await? {
    ///     // Process this item before requesting the next.
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors are yielded as stream items: validation and HTTP errors are the same as
    /// [`Self::send`]. A reported current page that differs from the requested page yields
    /// [`crate::Error::Pagination`]. The stream terminates after an error; previously yielded items
    /// are not rolled back. Missing continuation metadata ends the stream normally.
    #[cfg(feature = "stream")]
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<StepSubscriber>> + 'a {
        crate::pagination::validated_stream(
            validate(&self.query),
            crate::pagination::numbered_stream(self.client, self.path, self.query),
        )
    }
}
fn validate(query: &[(String, String)]) -> Result<()> {
    for (key, value) in query {
        if key == "limit" && !value.parse::<u32>().is_ok_and(|v| (1..=1000).contains(&v)) {
            return Err(Error::Config(
                "trigger subscriber limit must be between 1 and 1000".into(),
            ));
        }
        if key == "page" && value == "0" {
            return Err(Error::Config("page must start at 1".into()));
        }
        if key == "sort" && !["date", "-date", "email", "-email"].contains(&value.as_str()) {
            return Err(Error::Config(
                "trigger subscribers sort must be date or email, optionally prefixed with -".into(),
            ));
        }
    }
    Ok(())
}

impl std::fmt::Debug for CreateAutomationTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateAutomationTrigger")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
impl std::fmt::Debug for UpdateAutomationTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateAutomationTrigger")
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn create_omits_unset_configuration() {
        assert_eq!(
            serde_json::to_value(CreateAutomationTrigger::new("subscriber_joins_group")).unwrap(),
            json!({"type":"subscriber_joins_group"})
        );
    }

    #[test]
    fn update_serializes_explicit_empty_group_selection() {
        assert_eq!(
            serde_json::to_value(UpdateAutomationTrigger::new().group_ids(Vec::<String>::new()))
                .unwrap(),
            json!({"data":{"group_ids":[]}})
        );
    }
}
