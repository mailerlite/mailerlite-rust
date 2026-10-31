//! Email reads and explicit writes through the shared v2 transport.
use super::{
    params::{
        EditorContentBody, EmailSettingsUpdate, EtagBody, ListVersionsOptions, MoveBody, NameBody,
        NewBlock, Placement, SettingsKeys,
    },
    validation as validate,
};
use crate::{
    Error, MailerLite, Response,
    http::OperationPolicy,
    models::emails::{
        DataOnly, EmailBlock, EmailDocument, EmailHtml, EmailTemplate, EmailVersion, EtagOnly,
        SettingsResult, Validation, WithEtag,
    },
    pagination::NumberedPage,
};
use reqwest::Method;
use serde_json::{Map, Value};
#[derive(Clone)]
/// Borrowed simple-editor operations sharing client transport and rate limits.
pub struct Emails<'a> {
    mailerlite: &'a MailerLite,
    idempotency_key: Option<String>,
}

impl std::fmt::Debug for Emails<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Emails")
            .field("client", &self.mailerlite)
            .finish_non_exhaustive()
    }
}

impl<'a> Emails<'a> {
    pub(crate) fn new(mailerlite: &'a MailerLite) -> Self {
        Self {
            mailerlite,
            idempotency_key: None,
        }
    }

    /// Sends the `Idempotency-Key` header with the writes of this value. If a write fails with a network
    /// error, send the same write again with the same key. If the first request was a success, the API
    /// returns the stored response, and `response.metadata.is_idempotent_replay()` is `true`.
    ///
    /// - Only these methods send the key: `create_block`, `update_block`, `delete_block`, `move_block`,
    ///   `update_settings`, `create_template`, `create_version` and `restore_version`. The read methods and
    ///   the validate methods ignore it.
    /// - The SDK sends the key as you give it. The key must be 1 to 255 printable ASCII characters.
    ///   Otherwise the API gives a `400` error. HTTP removes a space at the start or the end of the key.
    /// - If HTTP cannot carry the key, for example because of a new line, the write gives
    ///   `Error::Config` and sends no request.
    ///
    /// Warning: use a new key for each write. Do not use an `Emails` value that has a key for a different
    /// write. The API keeps a key for 24 hours, and it rejects the same key with a different request.
    pub fn with_idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());

        self
    }

    /// Retrieve the current resource using the shared client transport.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn get(
        &self,
        email_id: impl AsRef<str>,
    ) -> Result<Response<WithEtag<EmailDocument>>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;

        send(
            self.mailerlite,
            Method::GET,
            &["emails", email_id],
            &[],
            None,
            None,
            OperationPolicy::READ.with_success_statuses(&[200]),
        )
        .await
    }

    /// The API can give a temporary `503` error. Its body can be other than JSON.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn html(
        &self,
        email_id: impl AsRef<str>,
    ) -> Result<Response<WithEtag<EmailHtml>>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;

        send(
            self.mailerlite,
            Method::GET,
            &["emails", email_id, "html"],
            &[],
            None,
            None,
            OperationPolicy::READ.with_success_statuses(&[200]),
        )
        .await
    }

    /// Retrieve one block and the current email version.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn get_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
    ) -> Result<Response<WithEtag<EmailBlock>>, Error> {
        let (email_id, block_id): (&str, &str) = (email_id.as_ref(), block_id.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::path_id("block_id", block_id)?;

        send(
            self.mailerlite,
            Method::GET,
            &["emails", email_id, "blocks", block_id],
            &[],
            None,
            None,
            OperationPolicy::READ.with_success_statuses(&[200]),
        )
        .await
    }

    /// Inserts a block into an email. Returns the new block (status `201`) and the new etag.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn create_block(
        &self,
        email_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        block: &NewBlock,
    ) -> Result<Response<WithEtag<EmailBlock>>, Error> {
        let (email_id, etag): (&str, &str) = (email_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;

        let idempotency_key: Option<&str> = self.idempotency_key.as_deref();

        send(
            self.mailerlite,
            Method::POST,
            &["emails", email_id, "blocks"],
            &[],
            Some(&EtagBody { body: block, etag }),
            idempotency_key,
            OperationPolicy::WRITE.with_success_statuses(&[201]),
        )
        .await
    }

    /// Replaces the content of a block. The API removes the keys that you do not send. The type and the
    /// position of the block do not change. Returns the block and the new etag.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn update_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        editor_content: &Map<String, Value>,
    ) -> Result<Response<WithEtag<EmailBlock>>, Error> {
        let (email_id, block_id, etag): (&str, &str, &str) =
            (email_id.as_ref(), block_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::path_id("block_id", block_id)?;

        let idempotency_key: Option<&str> = self.idempotency_key.as_deref();

        send(
            self.mailerlite,
            Method::PATCH,
            &["emails", email_id, "blocks", block_id],
            &[],
            Some(&EditorContentBody {
                editor_content,
                etag,
            }),
            idempotency_key,
            OperationPolicy::WRITE.with_success_statuses(&[200]),
        )
        .await
    }

    /// Deletes a block and the blocks in it. The response has only the new etag.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn delete_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
        etag: impl AsRef<str>,
    ) -> Result<Response<EtagOnly>, Error> {
        let (email_id, block_id, etag): (&str, &str, &str) =
            (email_id.as_ref(), block_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::path_id("block_id", block_id)?;

        let idempotency_key: Option<&str> = self.idempotency_key.as_deref();

        send(
            self.mailerlite,
            Method::DELETE,
            &["emails", email_id, "blocks", block_id],
            &[],
            Some(&EtagBody {
                body: &Map::new(),
                etag,
            }),
            idempotency_key,
            OperationPolicy::WRITE.with_success_statuses(&[200]),
        )
        .await
    }

    /// Moves a block among the blocks at the same level. The nesting of the block does not change.
    /// Returns the block and the new etag.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn move_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        placement: &Placement,
    ) -> Result<Response<WithEtag<EmailBlock>>, Error> {
        let (email_id, block_id, etag): (&str, &str, &str) =
            (email_id.as_ref(), block_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::path_id("block_id", block_id)?;

        let idempotency_key: Option<&str> = self.idempotency_key.as_deref();

        send(
            self.mailerlite,
            Method::POST,
            &["emails", email_id, "blocks", block_id, "move"],
            &[],
            Some(&MoveBody { placement, etag }),
            idempotency_key,
            OperationPolicy::WRITE.with_success_statuses(&[200]),
        )
        .await
    }

    /// Checks a block with the rules of `create_block`. The API saves nothing. The method needs no email
    /// and no etag.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn validate_block(&self, block: &NewBlock) -> Result<Response<Validation>, Error> {
        send(
            self.mailerlite,
            Method::POST,
            &["emails", "blocks", "validate"],
            &[],
            Some(block),
            None,
            OperationPolicy::READ.with_success_statuses(&[200]),
        )
        .await
    }

    /// Changes the design settings of an email. The SDK sends only the values that you set.
    /// Returns the settings, the new etag and the keys of the unsafe values that the API removed.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn update_settings(
        &self,
        email_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        settings: &EmailSettingsUpdate,
    ) -> Result<Response<SettingsResult>, Error> {
        let (email_id, etag): (&str, &str) = (email_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;
        settings.check()?;

        let idempotency_key: Option<&str> = self.idempotency_key.as_deref();

        send(
            self.mailerlite,
            Method::PATCH,
            &["emails", email_id, "settings"],
            &[],
            Some(&EtagBody {
                body: settings,
                etag,
            }),
            idempotency_key,
            OperationPolicy::WRITE.with_success_statuses(&[200]),
        )
        .await
    }

    /// Checks a settings change with the rules of `update_settings`. The API saves nothing. The method
    /// needs no etag.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn validate_settings(
        &self,
        email_id: impl AsRef<str>,
        settings: &EmailSettingsUpdate,
    ) -> Result<Response<Validation>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;
        settings.check_keys("")?;

        send(
            self.mailerlite,
            Method::POST,
            &["emails", email_id, "settings", "validate"],
            &[],
            Some(settings),
            None,
            OperationPolicy::READ.with_success_statuses(&[200]),
        )
        .await
    }

    /// Saves a copy of an email as a new template (status `201`). The API cannot delete templates.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn create_template(
        &self,
        email_id: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Result<Response<EmailTemplate>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;

        let idempotency_key: Option<&str> = self.idempotency_key.as_deref();

        send::<DataOnly<EmailTemplate>>(
            self.mailerlite,
            Method::POST,
            &["emails", email_id, "templates"],
            &[],
            Some(&NameBody {
                name: name.as_ref(),
            }),
            idempotency_key,
            OperationPolicy::WRITE.with_success_statuses(&[201]),
        )
        .await
        .map(|response| response.map(|envelope| envelope.data))
    }

    /// Lists the saved versions of an email. The newest version is first.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn versions(
        &self,
        email_id: impl AsRef<str>,
        options: ListVersionsOptions,
    ) -> Result<Response<NumberedPage<EmailVersion>>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;
        let query: Vec<(&str, String)> = validate::query_pairs([
            ("page", options.page.map(|page| page.to_string())),
            ("limit", options.limit.map(|limit| limit.to_string())),
        ]);

        send(
            self.mailerlite,
            Method::GET,
            &["emails", email_id, "versions"],
            &query,
            None,
            None,
            OperationPolicy::READ.with_success_statuses(&[200]),
        )
        .await
    }

    /// Saves the email as a new version. The request has no body. If nothing changed after the last
    /// version, the API saves no new version and returns the last version: then `response.metadata.status` is `200`,
    /// not `201`.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn create_version(
        &self,
        email_id: impl AsRef<str>,
    ) -> Result<Response<EmailVersion>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;

        let idempotency_key: Option<&str> = self.idempotency_key.as_deref();

        send::<DataOnly<EmailVersion>>(
            self.mailerlite,
            Method::POST,
            &["emails", email_id, "versions"],
            &[],
            None,
            idempotency_key,
            OperationPolicy::WRITE.with_success_statuses(&[200, 201]),
        )
        .await
        .map(|response| response.map(|envelope| envelope.data))
    }

    /// Puts an email back to a saved version. Before the restore, the API saves the current state as a
    /// version, so you can undo the restore. Returns the full email and the new etag.
    ///
    /// # Errors
    /// Returns local configuration errors before dispatch, or shared transport, API,
    /// decoding, and operation-timeout errors. Writes are never automatically retried.
    pub async fn restore_version(
        &self,
        email_id: impl AsRef<str>,
        version_id: impl AsRef<str>,
        etag: impl AsRef<str>,
    ) -> Result<Response<WithEtag<EmailDocument>>, Error> {
        let (email_id, version_id, etag): (&str, &str, &str) =
            (email_id.as_ref(), version_id.as_ref(), etag.as_ref());
        validate::path_id("email_id", email_id)?;
        validate::path_id("version_id", version_id)?;

        let idempotency_key: Option<&str> = self.idempotency_key.as_deref();

        send(
            self.mailerlite,
            Method::POST,
            &["emails", email_id, "versions", version_id, "restore"],
            &[],
            Some(&EtagBody {
                body: &Map::new(),
                etag,
            }),
            idempotency_key,
            OperationPolicy::WRITE.with_success_statuses(&[200]),
        )
        .await
    }
}

async fn send<T: serde::de::DeserializeOwned>(
    client: &MailerLite,
    method: Method,
    path: &[&str],
    query: &[(&str, String)],
    body: Option<&dyn crate::http::RequestBody>,
    key: Option<&str>,
    mut policy: OperationPolicy,
) -> crate::Result<Response<T>> {
    policy.idempotency_key = key
        .map(reqwest::header::HeaderValue::from_str)
        .transpose()
        .map_err(|_| Error::Config("idempotency key cannot be encoded as an HTTP header".into()))?;
    if let Some(value) = policy.idempotency_key.as_mut() {
        value.set_sensitive(true);
    }
    let query: Vec<_> = query
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect();
    client.request(method, path, &query, body, policy).await
}
