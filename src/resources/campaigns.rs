//! Campaign drafts, delivery, and subscriber reports.
use crate::models::campaigns::{Campaign, CampaignActivity};
use crate::{Error, MailerLite, Response, Result, http::OperationPolicy, pagination::NumberedPage};
use reqwest::Method;
/// CampaignStatus request value.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum CampaignStatus {
    /// Draft.
    #[serde(rename = "draft")]
    Draft,
    /// Ready.
    #[serde(rename = "ready")]
    Ready,
    /// Sent.
    #[serde(rename = "sent")]
    Sent,
}
impl CampaignStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Ready => "ready",
            Self::Sent => "sent",
        }
    }
}
/// CampaignFilterType request value.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum CampaignFilterType {
    /// Regular.
    #[serde(rename = "regular")]
    Regular,
    /// Ab.
    #[serde(rename = "ab")]
    Ab,
    /// Resend.
    #[serde(rename = "resend")]
    Resend,
    /// Rss.
    #[serde(rename = "rss")]
    Rss,
}
impl CampaignFilterType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::Ab => "ab",
            Self::Resend => "resend",
            Self::Rss => "rss",
        }
    }
}
/// ActivityType request value.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum ActivityType {
    /// Opened.
    #[serde(rename = "opened")]
    Opened,
    /// Unopened.
    #[serde(rename = "unopened")]
    Unopened,
    /// Clicked.
    #[serde(rename = "clicked")]
    Clicked,
    /// Unsubscribed.
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    /// Forwarded.
    #[serde(rename = "forwarded")]
    Forwarded,
    /// Hardbounced.
    #[serde(rename = "hardbounced")]
    Hardbounced,
    /// Softbounced.
    #[serde(rename = "softbounced")]
    Softbounced,
    /// Junk.
    #[serde(rename = "junk")]
    Junk,
}
impl ActivityType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Opened => "opened",
            Self::Unopened => "unopened",
            Self::Clicked => "clicked",
            Self::Unsubscribed => "unsubscribed",
            Self::Forwarded => "forwarded",
            Self::Hardbounced => "hardbounced",
            Self::Softbounced => "softbounced",
            Self::Junk => "junk",
        }
    }
}
/// Campaign resource handle.
#[derive(Debug, Clone, Copy)]
pub struct Campaigns<'a> {
    client: &'a MailerLite,
}
impl<'a> Campaigns<'a> {
    pub(crate) fn new(client: &'a MailerLite) -> Self {
        Self { client }
    }
    /// List campaigns. MailerLite defaults its status filter to ready.
    ///
    /// This only constructs a request. Choose a status explicitly to list drafts.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::campaigns::CampaignStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.campaigns().list()
    ///     .status(CampaignStatus::Draft).limit(25).send().await?;
    ///
    /// println!("{} drafts", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListCampaigns<'a> {
        ListCampaigns::new(self.client, vec!["campaigns".into()])
    }
    /// Fetch campaign details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.campaigns().get("123456789").await?;
    ///
    /// println!("{}", response.data.name);
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
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Response<Campaign>> {
        self.client
            .data(
                Method::GET,
                &["campaigns", id.as_ref()],
                &[],
                None,
                OperationPolicy::READ,
            )
            .await
    }
    /// Create a draft campaign; sending is a separate operation.
    ///
    /// Creating a draft does not send it. Use [`Self::schedule`] to initiate delivery.
    /// Writes are never automatically replayed. A timeout or transport failure does
    /// not prove that the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::campaigns::{CreateCampaign, Email};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let email = Email::new("Monthly news", "Example team", "sender@example.com");
    /// let params = CreateCampaign::new("Monthly newsletter", email).groups(["987654321"]);
    ///
    /// let response = mailerlite.campaigns().create(params).await?;
    ///
    /// println!("Created draft {}", response.data.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] for a name outside 1..=255 characters, no emails,
    /// an invalid A/B test split, a multivariate combination count outside 1..=8,
    /// or an email count inconsistent with the configured content variations.
    /// Sender verification, account limits, and other field rules are checked by the
    /// server. Request serialization can return [`Error::Serialize`]. Server rejection
    /// returns [`crate::Error::Api`]. HTTP execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn create(&self, params: CreateCampaign) -> Result<Response<Campaign>> {
        params.validate()?;
        self.client
            .data(
                Method::POST,
                &["campaigns"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Update an existing draft campaign.
    ///
    /// Writes are never automatically replayed. A timeout or transport failure does
    /// not prove that the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::campaigns::{Email, UpdateCampaign};
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let email = Email::new("Updated subject", "Example team", "sender@example.com");
    /// let params = UpdateCampaign::new("Updated newsletter", email);
    ///
    /// let response = mailerlite.campaigns().update("123456789", params).await?;
    ///
    /// println!("Updated {}", response.data.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// An empty or dot (`.` or `..`) identifier returns [`crate::Error::Config`]
    /// before dispatch. Draft state, sender eligibility, and field validity are checked
    /// by the server;
    /// this method does not apply the creation-only local validation rules.
    /// Serialization failures return [`Error::Serialize`]. Server rejection returns
    /// [`crate::Error::Api`]. HTTP execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn update(
        &self,
        id: impl AsRef<str>,
        params: UpdateCampaign,
    ) -> Result<Response<Campaign>> {
        self.client
            .data(
                Method::PUT,
                &["campaigns", id.as_ref()],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Schedule a campaign or send it immediately. This write is never automatically retried.
    ///
    /// This can send email to the campaign's recipients. Use a future date for a
    /// scheduled delivery; success does not mean all messages have been delivered.
    /// Writes are never automatically replayed. A timeout or transport failure does
    /// not prove that the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::campaigns::ScheduleCampaign;
    ///
    /// # async fn example(mailerlite: &MailerLite, send_date: &str) -> mailerlite_rs::Result<()> {
    /// let params = ScheduleCampaign::scheduled(send_date, "10", "00");
    ///
    /// let response = mailerlite.campaigns().schedule("123456789", params).await?;
    ///
    /// println!("Campaign state: {}", response.data.status);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// An empty or dot (`.` or `..`) identifier returns [`crate::Error::Config`]
    /// before dispatch. The server validates dates, delivery mode, campaign readiness, and account
    /// permissions. Serialization failures return [`Error::Serialize`]. Server
    /// rejection returns [`crate::Error::Api`]. HTTP execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn schedule(
        &self,
        id: impl AsRef<str>,
        params: ScheduleCampaign,
    ) -> Result<Response<Campaign>> {
        self.client
            .data(
                Method::POST,
                &["campaigns", id.as_ref(), "schedule"],
                &[],
                Some(&params),
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Cancel a campaign in the ready state.
    ///
    /// The server decides whether the campaign is still cancellable. Cancellation
    /// does not recall messages already delivered. Writes are never automatically
    /// replayed. A timeout or transport failure does
    /// not prove that the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = mailerlite.campaigns().cancel("123456789").await?;
    ///
    /// println!("Campaign state: {}", response.data.status);
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
    pub async fn cancel(&self, id: impl AsRef<str>) -> Result<Response<Campaign>> {
        self.client
            .data(
                Method::POST,
                &["campaigns", id.as_ref(), "cancel"],
                &[],
                None,
                OperationPolicy::WRITE,
            )
            .await
    }
    /// Delete a campaign.
    ///
    /// Writes are never automatically replayed. A timeout or transport failure does
    /// not prove that the server left the resource unchanged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// mailerlite.campaigns().delete("123456789").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// An empty or dot (`.` or `..`) identifier returns [`crate::Error::Config`]
    /// before dispatch. Unexpected success statuses or a nonempty success body return
    /// [`Error::Decode`]. Server rejection returns [`crate::Error::Api`]. HTTP
    /// execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn delete(&self, id: impl AsRef<str>) -> Result<Response<()>> {
        self.client
            .empty(
                Method::DELETE,
                &["campaigns", id.as_ref()],
                &[],
                None,
                OperationPolicy::WRITE,
            )
            .await
    }
    /// List subscriber activity for a sent campaign.
    ///
    /// This constructs a numbered-page request; nothing is fetched until `.send()`
    /// or stream consumption. See [`ListCampaignActivity::send`] for validation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::campaigns::ActivityType;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.campaigns().activity("123456789")
    ///     .kind(ActivityType::Opened).limit(25).send().await?;
    ///
    /// println!("{} activity records", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn activity(&self, id: impl AsRef<str>) -> ListCampaignActivity<'a> {
        ListCampaignActivity::new(
            self.client,
            vec![
                "campaigns".into(),
                id.as_ref().into(),
                "reports".into(),
                "subscriber-activity".into(),
            ],
        )
    }
}
/// Parameters for a paginated request.
#[derive(Debug)]
pub struct ListCampaigns<'a> {
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
}
impl<'a> ListCampaigns<'a> {
    fn new(client: &'a MailerLite, path: Vec<String>) -> Self {
        Self {
            client,
            path,
            query: Vec::new(),
        }
    }
    /// Set limit.
    pub fn limit(mut self, value: u32) -> Self {
        self.query.retain(|(key, _)| key != "limit");
        self.query.push(("limit".into(), value.to_string()));
        self
    }
    /// Set page.
    pub fn page(mut self, value: u64) -> Self {
        self.query.retain(|(key, _)| key != "page");
        self.query.push(("page".into(), value.to_string()));
        self
    }
    /// Set status.
    pub fn status(mut self, value: CampaignStatus) -> Self {
        self.query.retain(|(key, _)| key != "filter[status]");
        self.query
            .push(("filter[status]".into(), value.as_str().into()));
        self
    }
    /// Set kind.
    pub fn kind(mut self, value: CampaignFilterType) -> Self {
        self.query.retain(|(key, _)| key != "filter[type]");
        self.query
            .push(("filter[type]".into(), value.as_str().into()));
        self
    }
    /// Fetch one page.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::campaigns::CampaignStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.campaigns().list()
    ///     .status(CampaignStatus::Sent).page(1).limit(25).send().await?;
    ///
    /// println!("Next page: {:?}", page.data.next_page());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// A zero page or limit returns [`Error::Config`] before any request. Server
    /// rejection returns [`crate::Error::Api`]. HTTP execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn send(self) -> Result<Response<NumberedPage<Campaign>>> {
        validate_query(&self.query)?;
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &self.query, None, OperationPolicy::READ)
            .await
    }
    /// Fetch subsequent pages lazily while preserving request filters.
    ///
    /// Requires the `stream` feature; this example also uses `futures-util`.
    /// Each page is fetched on demand from the configured endpoint. Dropping the stream
    /// stops further requests. The operation deadline applies per page, not per stream.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use futures_util::TryStreamExt;
    /// use mailerlite_rs::resources::campaigns::CampaignStatus;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let campaigns = mailerlite.campaigns();
    /// let stream = campaigns.list().status(CampaignStatus::Sent).limit(25).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(campaign) = stream.try_next().await? {
    ///     let _ = campaign; // Process one record at a time.
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors are yielded as items. Invalid parameters (a zero page or limit)
    /// yield [`Error::Config`] before dispatch. Page fetches yield the errors described
    /// by [`Self::send`]; an unexpected server page number yields [`Error::Pagination`].
    /// The stream stops after an error; already yielded items are not rolled back.
    #[cfg(feature = "stream")]
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<Campaign>> + 'a {
        crate::pagination::validated_stream(
            validate_query(&self.query),
            crate::pagination::numbered_stream(self.client, self.path, self.query),
        )
    }
}
/// Parameters for a paginated request.
#[derive(Debug)]
pub struct ListCampaignActivity<'a> {
    client: &'a MailerLite,
    path: Vec<String>,
    query: Vec<(String, String)>,
}
impl<'a> ListCampaignActivity<'a> {
    fn new(client: &'a MailerLite, path: Vec<String>) -> Self {
        Self {
            client,
            path,
            query: Vec::new(),
        }
    }
    /// Set limit.
    pub fn limit(mut self, value: u32) -> Self {
        self.query.retain(|(key, _)| key != "limit");
        self.query.push(("limit".into(), value.to_string()));
        self
    }
    /// Set page.
    pub fn page(mut self, value: u64) -> Self {
        self.query.retain(|(key, _)| key != "page");
        self.query.push(("page".into(), value.to_string()));
        self
    }
    /// Set kind.
    pub fn kind(mut self, value: ActivityType) -> Self {
        self.query.retain(|(key, _)| key != "filter[type]");
        self.query
            .push(("filter[type]".into(), value.as_str().into()));
        self
    }
    /// Set search.
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "filter[search]");
        self.query.push(("filter[search]".into(), value.into()));
        self
    }
    /// Set sort.
    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "sort");
        self.query.push(("sort".into(), value.into()));
        self
    }
    /// Set include.
    pub fn include(mut self, value: impl Into<String>) -> Self {
        self.query.retain(|(key, _)| key != "include");
        self.query.push(("include".into(), value.into()));
        self
    }
    /// Fetch one page.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::campaigns::ActivityType;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let page = mailerlite.campaigns().activity("123456789")
    ///     .kind(ActivityType::Clicked).page(1).limit(25).send().await?;
    ///
    /// println!("{} activity records", page.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] before dispatch for a zero page, a limit other than
    /// 10, 25, 50, or 100, or an empty/dot campaign ID. The server validates other
    /// filters. Server rejection returns [`crate::Error::Api`]. HTTP execution may also return
    /// [`crate::Error::Transport`], [`crate::Error::Decode`], or [`crate::Error::Timeout`].
    pub async fn send(self) -> Result<Response<NumberedPage<CampaignActivity>>> {
        validate_activity_query(&self.query)?;
        let path: Vec<&str> = self.path.iter().map(String::as_str).collect();
        self.client
            .request(Method::GET, &path, &self.query, None, OperationPolicy::READ)
            .await
    }
    /// Fetch subsequent pages lazily while preserving request filters.
    ///
    /// Requires the `stream` feature; this example also uses `futures-util`.
    /// Each page is fetched on demand from the configured endpoint. Dropping the stream
    /// stops further requests. The operation deadline applies per page, not per stream.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use futures_util::TryStreamExt;
    /// use mailerlite_rs::resources::campaigns::ActivityType;
    ///
    /// # async fn example(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let campaigns = mailerlite.campaigns();
    /// let stream = campaigns.activity("123456789").kind(ActivityType::Opened).limit(25).into_stream();
    /// tokio::pin!(stream);
    ///
    /// while let Some(activity) = stream.try_next().await? {
    ///     let _ = activity; // Process one record at a time.
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors are yielded as items. Invalid parameters (an invalid campaign ID, zero
    /// page, or a limit outside 10/25/50/100)
    /// yield [`Error::Config`] before dispatch. Page fetches yield the errors described
    /// by [`Self::send`]; an unexpected server page number yields [`Error::Pagination`].
    /// The stream stops after an error; already yielded items are not rolled back.
    #[cfg(feature = "stream")]
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<CampaignActivity>> + 'a {
        crate::pagination::validated_stream(
            validate_activity_query(&self.query),
            crate::pagination::numbered_stream(self.client, self.path, self.query),
        )
    }
}
fn validate_activity_query(query: &[(String, String)]) -> Result<()> {
    validate_query(query)?;
    if query
        .iter()
        .any(|(key, value)| key == "limit" && !["10", "25", "50", "100"].contains(&value.as_str()))
    {
        return Err(Error::Config(
            "campaign activity limit must be 10, 25, 50, or 100".into(),
        ));
    }
    Ok(())
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

/// Email request parameters.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Email {
    /// Subject.
    pub subject: String,
    /// From name.
    pub from_name: String,
    /// From.
    pub from: String,
    /// Reply to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl Email {
    /// Build an email using a subject, sender name, and verified sender address.
    pub fn new(
        subject: impl Into<String>,
        from_name: impl Into<String>,
        from: impl Into<String>,
    ) -> Self {
        Self {
            subject: subject.into(),
            from_name: from_name.into(),
            from: from.into(),
            reply_to: None,
            content: None,
        }
    }
    /// Set HTML content; account plan and footer restrictions are checked by MailerLite.
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }
    /// Set a verified reply-to address.
    pub fn reply_to(mut self, value: impl Into<String>) -> Self {
        self.reply_to = Some(value.into());
        self
    }
}
/// WinnerMetric option.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum WinnerMetric {
    /// O.
    #[serde(rename = "o")]
    Opens,
    /// C.
    #[serde(rename = "c")]
    Clicks,
}
/// WaitUnit option.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum WaitUnit {
    /// H.
    #[serde(rename = "h")]
    Hours,
    /// D.
    #[serde(rename = "d")]
    Days,
}
/// AbTestType option.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum AbTestType {
    /// Subject.
    #[serde(rename = "subject")]
    Subject,
    /// Sender.
    #[serde(rename = "sender")]
    Sender,
    /// Sending time.
    #[serde(rename = "sending_time")]
    SendingTime,
}
/// Variant request parameters.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct Variant {
    /// Subject.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// From name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// From.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}
/// AbSettings request parameters.
#[derive(Debug, Clone, serde::Serialize)]
pub struct AbSettings {
    /// Test type.
    pub test_type: AbTestType,
    /// Test split.
    pub test_split: u8,
    /// Select winner by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_winner_by: Option<WinnerMetric>,
    /// After time amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time_amount: Option<u32>,
    /// After time unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time_unit: Option<WaitUnit>,
    /// B value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_value: Option<Variant>,
}
impl AbSettings {
    /// Test two delivery times; configure the pair with ScheduleCampaign.
    pub fn sending_time() -> Self {
        Self {
            test_type: AbTestType::SendingTime,
            test_split: 100,
            select_winner_by: None,
            after_time_amount: None,
            after_time_unit: None,
            b_value: None,
        }
    }
    /// Test a second subject with an explicit winning metric and waiting period.
    pub fn subject(
        subject: impl Into<String>,
        split: u8,
        metric: WinnerMetric,
        amount: u32,
        unit: WaitUnit,
    ) -> Self {
        Self {
            test_type: AbTestType::Subject,
            test_split: split,
            select_winner_by: Some(metric),
            after_time_amount: Some(amount),
            after_time_unit: Some(unit),
            b_value: Some(Variant {
                subject: Some(subject.into()),
                ..Variant::default()
            }),
        }
    }
    /// Test a second verified sender.
    pub fn sender(
        from: impl Into<String>,
        name: impl Into<String>,
        split: u8,
        metric: WinnerMetric,
        amount: u32,
        unit: WaitUnit,
    ) -> Self {
        Self {
            test_type: AbTestType::Sender,
            test_split: split,
            select_winner_by: Some(metric),
            after_time_amount: Some(amount),
            after_time_unit: Some(unit),
            b_value: Some(Variant {
                from: Some(from.into()),
                from_name: Some(name.into()),
                ..Variant::default()
            }),
        }
    }
}
/// Auto-resend configuration; its test type is always subject.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResendSettings {
    test_type: &'static str,
    select_winner_by: WinnerMetric,
    b_value: Variant,
}
impl ResendSettings {
    /// Select recipients who did not open/click and a second subject.
    pub fn new(subject: impl Into<String>, metric: WinnerMetric) -> Self {
        Self {
            test_type: "subject",
            select_winner_by: metric,
            b_value: Variant {
                subject: Some(subject.into()),
                ..Variant::default()
            },
        }
    }
}
/// Sender request parameters.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Sender {
    /// From.
    pub from: String,
    /// From name.
    pub from_name: String,
}
/// TestVariants request parameters.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TestVariants {
    /// Subject.
    pub subject: Vec<String>,
    /// Sender.
    pub sender: Vec<Sender>,
    /// Content.
    pub content: u8,
    /// Sending time.
    pub sending_time: u8,
}
/// MultivariateSettings request parameters.
#[derive(Debug, Clone, serde::Serialize)]
pub struct MultivariateSettings {
    /// Test variants.
    pub test_variants: TestVariants,
    /// Test split.
    pub test_split: u8,
    /// Select winner by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_winner_by: Option<WinnerMetric>,
    /// After time amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time_amount: Option<u32>,
    /// After time unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time_unit: Option<WaitUnit>,
}
/// TrackingSettings request parameters.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackingSettings {
    /// Ecommerce tracking.
    pub ecommerce_tracking: bool,
}
/// Email content for a multivariate campaign.
///
/// Subject and sender values can be omitted because MailerLite derives them
/// from [`TestVariants`]. Regular campaigns continue to require [`Email`].
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct MultivariateEmail {
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
}
impl MultivariateEmail {
    /// Create an email with subject and sender supplied by test variants.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set optional HTML content, subject to account plan and footer rules.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }
    /// Set an optional subject explicitly.
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }
    /// Set an optional verified sender address and display name explicitly.
    pub fn sender(mut self, from: impl Into<String>, name: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self.from_name = Some(name.into());
        self
    }
    /// Set an optional verified reply-to address.
    pub fn reply_to(mut self, reply_to: impl Into<String>) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }
}
impl From<Email> for MultivariateEmail {
    fn from(email: Email) -> Self {
        Self {
            subject: Some(email.subject),
            from_name: Some(email.from_name),
            from: Some(email.from),
            reply_to: email.reply_to,
            content: email.content,
        }
    }
}
/// Parameters for creating a campaign. Constructors set its type and required settings.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateCampaign {
    name: String,
    #[serde(rename = "type")]
    kind: &'static str,
    emails: Vec<MultivariateEmail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<Vec<crate::models::GroupId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segments: Option<Vec<crate::models::SegmentId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ab_settings: Option<AbSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resend_settings: Option<ResendSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multivariate_settings: Option<MultivariateSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<TrackingSettings>,
}
impl CreateCampaign {
    /// Create a regular draft campaign.
    pub fn new(name: impl Into<String>, email: Email) -> Self {
        Self {
            name: name.into(),
            kind: "regular",
            emails: vec![email.into()],
            language_id: None,
            groups: None,
            segments: None,
            ab_settings: None,
            resend_settings: None,
            multivariate_settings: None,
            settings: None,
        }
    }
    /// Create an A/B campaign.
    pub fn ab(name: impl Into<String>, email: Email, settings: AbSettings) -> Self {
        let mut p = Self::new(name, email);
        p.kind = "ab";
        p.ab_settings = Some(settings);
        p
    }
    /// Create an auto-resend campaign.
    pub fn resend(name: impl Into<String>, email: Email, settings: ResendSettings) -> Self {
        let mut p = Self::new(name, email);
        p.kind = "resend";
        p.resend_settings = Some(settings);
        p
    }
    /// Create a multivariate campaign with explicit content variations.
    pub fn multivariate(
        name: impl Into<String>,
        emails: Vec<impl Into<MultivariateEmail>>,
        settings: MultivariateSettings,
    ) -> Self {
        Self {
            name: name.into(),
            kind: "multivariate",
            emails: emails.into_iter().map(Into::into).collect(),
            language_id: None,
            groups: None,
            segments: None,
            ab_settings: None,
            resend_settings: None,
            multivariate_settings: Some(settings),
            settings: None,
        }
    }
    /// Set the unsubscribe template language identifier.
    pub fn language_id(mut self, id: u64) -> Self {
        self.language_id = Some(id);
        self
    }
    /// Set recipient groups; segments take precedence if both are provided.
    pub fn groups(
        mut self,
        ids: impl IntoIterator<Item = impl Into<crate::models::GroupId>>,
    ) -> Self {
        self.groups = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set recipient segments.
    pub fn segments(
        mut self,
        ids: impl IntoIterator<Item = impl Into<crate::models::SegmentId>>,
    ) -> Self {
        self.segments = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Enable or disable e-commerce link tracking.
    pub fn ecommerce_tracking(mut self, enabled: bool) -> Self {
        self.settings = Some(TrackingSettings {
            ecommerce_tracking: enabled,
        });
        self
    }
    fn validate(&self) -> Result<()> {
        if self.name.is_empty() || self.name.chars().count() > 255 || self.emails.is_empty() {
            return Err(Error::Config(
                "campaign requires a name of 1..=255 characters and at least one email".into(),
            ));
        }
        if let Some(settings) = &self.ab_settings {
            let valid = match settings.test_type {
                AbTestType::SendingTime => settings.test_split == 100,
                _ => (5..=25).contains(&settings.test_split),
            };
            if !valid {
                return Err(Error::Config("invalid A/B test split".into()));
            }
        }
        if let Some(settings) = &self.multivariate_settings {
            let v = &settings.test_variants;
            let combinations = v
                .subject
                .len()
                .checked_mul(v.sender.len())
                .and_then(|n| n.checked_mul(v.content.into()))
                .and_then(|n| n.checked_mul(v.sending_time.into()));
            if !matches!(combinations, Some(1..=8)) {
                return Err(Error::Config(
                    "multivariate test must contain between one and eight combinations".into(),
                ));
            }
            if self.emails.len() != usize::from(v.content) {
                return Err(Error::Config(
                    "email count must match multivariate content variations".into(),
                ));
            }
        }
        Ok(())
    }
}
/// Parameters for replacing the documented fields of a draft campaign.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UpdateCampaign {
    name: String,
    emails: Vec<Email>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<Vec<crate::models::GroupId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segments: Option<Vec<crate::models::SegmentId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ab_settings: Option<AbSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resend_settings: Option<ResendSettings>,
}
impl UpdateCampaign {
    /// Build required draft update parameters.
    pub fn new(name: impl Into<String>, email: Email) -> Self {
        Self {
            name: name.into(),
            emails: vec![email],
            language_id: None,
            groups: None,
            segments: None,
            ab_settings: None,
            resend_settings: None,
        }
    }
    /// Set a language identifier.
    pub fn language_id(mut self, id: u64) -> Self {
        self.language_id = Some(id);
        self
    }
    /// Replace recipient groups.
    pub fn groups(
        mut self,
        ids: impl IntoIterator<Item = impl Into<crate::models::GroupId>>,
    ) -> Self {
        self.groups = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Replace recipient segments.
    pub fn segments(
        mut self,
        ids: impl IntoIterator<Item = impl Into<crate::models::SegmentId>>,
    ) -> Self {
        self.segments = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set A/B testing parameters.
    pub fn ab_settings(mut self, value: AbSettings) -> Self {
        self.ab_settings = Some(value);
        self
    }
    /// Set auto-resend parameters.
    pub fn resend_settings(mut self, value: ResendSettings) -> Self {
        self.resend_settings = Some(value);
        self
    }
}
/// Delivery option.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum Delivery {
    /// Instant.
    #[serde(rename = "instant")]
    Instant,
    /// Scheduled.
    #[serde(rename = "scheduled")]
    Scheduled,
    /// Timezone based.
    #[serde(rename = "timezone_based")]
    TimezoneBased,
    /// Smart sending.
    #[serde(rename = "smart_sending")]
    SmartSending,
}
/// ResendDelivery option.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum ResendDelivery {
    /// Day.
    #[serde(rename = "day")]
    Day,
    /// Scheduled.
    #[serde(rename = "scheduled")]
    Scheduled,
}
/// Schedule request parameters.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct Schedule {
    /// Date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<String>,
    /// Minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes: Option<String>,
    /// Timezone id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_id: Option<u64>,
}
/// ResendSchedule request parameters.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResendSchedule {
    /// Delivery.
    pub delivery: ResendDelivery,
    /// Date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<String>,
    /// Minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes: Option<String>,
    /// Timezone id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_id: Option<u64>,
}
/// SendingTime request parameters.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SendingTime {
    /// Date.
    pub date: String,
    /// Hours.
    pub hours: String,
    /// Minutes.
    pub minutes: String,
    /// Timezone id.
    pub timezone_id: String,
}
/// ScheduleCampaign request parameters.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScheduleCampaign {
    /// Delivery.
    pub delivery: Delivery,
    /// Schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// Resend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resend: Option<ResendSchedule>,
    /// Sending time test schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_time_test_schedule: Option<[SendingTime; 2]>,
}
impl ScheduleCampaign {
    /// Send immediately; creating the campaign alone does not send it.
    pub fn instant() -> Self {
        Self {
            delivery: Delivery::Instant,
            schedule: None,
            resend: None,
            sending_time_test_schedule: None,
        }
    }
    /// Schedule delivery using MailerLite's account timezone unless overridden.
    pub fn scheduled(
        date: impl Into<String>,
        hours: impl Into<String>,
        minutes: impl Into<String>,
    ) -> Self {
        Self {
            delivery: Delivery::Scheduled,
            schedule: Some(Schedule {
                date: Some(date.into()),
                hours: Some(hours.into()),
                minutes: Some(minutes.into()),
                timezone_id: None,
            }),
            resend: None,
            sending_time_test_schedule: None,
        }
    }
    /// Attach a schedule for the second auto-resend delivery.
    pub fn resend(mut self, value: ResendSchedule) -> Self {
        self.resend = Some(value);
        self
    }
    /// Set both times for an A/B sending-time test.
    pub fn sending_times(mut self, value: [SendingTime; 2]) -> Self {
        self.sending_time_test_schedule = Some(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ab_subject_payload_omits_resend_settings() {
        let params = CreateCampaign::ab(
            "AB",
            Email::new("A", "Sender", "sender@example.com"),
            AbSettings::subject("B", 10, WinnerMetric::Opens, 1, WaitUnit::Hours),
        );
        let wire = serde_json::to_value(params).unwrap();
        assert_eq!(wire["ab_settings"]["test_type"], "subject");
        assert_eq!(wire["ab_settings"]["b_value"]["subject"], "B");
        assert!(wire.get("resend_settings").is_none());
    }

    #[test]
    fn resend_click_winner_uses_the_wire_code() {
        let params = CreateCampaign::resend(
            "Resend",
            Email::new("A", "Sender", "sender@example.com"),
            ResendSettings::new("B", WinnerMetric::Clicks),
        );
        assert_eq!(
            serde_json::to_value(params).unwrap()["resend_settings"]["select_winner_by"],
            "c"
        );
    }

    #[test]
    fn multivariate_payload_retains_numeric_variant_counts() {
        let params = CreateCampaign::multivariate(
            "Variants",
            vec![Email::new("First", "Sender", "sender@example.com")],
            MultivariateSettings {
                test_variants: TestVariants {
                    subject: vec!["A".into(), "B".into()],
                    sender: vec![Sender {
                        from: "sender@example.com".into(),
                        from_name: "Sender".into(),
                    }],
                    content: 1,
                    sending_time: 1,
                },
                test_split: 20,
                select_winner_by: Some(WinnerMetric::Opens),
                after_time_amount: Some(1),
                after_time_unit: Some(WaitUnit::Days),
            },
        );
        let wire = serde_json::to_value(params).unwrap();
        assert_eq!(wire["type"], "multivariate");
        assert_eq!(wire["multivariate_settings"]["test_variants"]["content"], 1);
    }

    #[test]
    fn scheduled_delivery_preserves_zero_padded_time_strings() {
        assert_eq!(
            serde_json::to_value(ScheduleCampaign::scheduled("2027-01-02", "09", "05")).unwrap(),
            json!({"delivery":"scheduled","schedule":{"date":"2027-01-02","hours":"09","minutes":"05"}})
        );
    }

    #[test]
    fn empty_multivariate_email_omits_all_optional_fields() {
        assert_eq!(
            serde_json::to_value(MultivariateEmail::new()).unwrap(),
            json!({})
        );
    }
}
