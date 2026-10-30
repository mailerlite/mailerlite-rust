//! Automation triggers and execution counts, verified against the application API.
use crate::models::automation_configuration::{
    TRIGGER_FIELDS, TriggerConfiguration, configuration_fields,
};
use crate::{
    Error, MailerLite, Response, Result,
    http::{OperationPolicy, body},
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
    pub fn configured(configuration: TriggerConfiguration) -> Result<Self> {
        Self::new(configuration.kind()).configuration(configuration)
    }
    /// Replace configuration, rejecting a mismatch with this trigger's type.
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
    pub fn configuration(mut self, configuration: TriggerConfiguration) -> Result<Self> {
        self.data = configuration_fields(&configuration)?;
        Ok(self)
    }
    fn insert_setting(mut self, key: &str, value: Value) -> Self {
        self.data.insert(key.into(), value);
        self
    }
    /// Add a future field; known keys must use typed configuration.
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
                Some(body(&params)?),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Replace a trigger's configuration on an inactive automation.
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
                Some(body(&params)?),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Remove a trigger from an inactive automation.
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
    pub async fn send(self) -> Result<Response<NumberedPage<StepSubscriber>>> {
        validate(&self.query)?;
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &self.query, None, OperationPolicy::READ)
            .await
    }
    /// Fetch pages lazily without following server-provided navigation URLs.
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
