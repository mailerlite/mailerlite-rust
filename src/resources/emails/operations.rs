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
        BlockContent, ContentRef, DataOnly, EmailBlock, EmailDocument, EmailHtml,
        EmailResponseMetadata, EmailTemplate, EmailVersion, EtagOnly, SettingsResult, Validation,
        WithEtag,
    },
    pagination::NumberedPage,
    resources::query::query_pairs,
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

    /// Retrieve an email with its blocks, settings, and current etag (`GET /emails/{email}`).
    ///
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let email = client.emails().get("123").await?;
    ///
    /// println!("Current email version: {}", email.data.etag);
    /// println!("{:#?}", email.data.data);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    pub async fn get(
        &self,
        email_id: impl AsRef<str>,
    ) -> Result<Response<WithEtag<EmailDocument>, EmailResponseMetadata>, Error> {
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

    /// Retrieve the rendered HTML of an email and its current etag.
    ///
    /// The API can give a temporary `503` error. Its body can be other than JSON.
    ///
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let email = client.emails().html("123").await?;
    ///
    /// println!("{}", email.data.data.html);
    /// println!("Current email version: {}", email.data.etag);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    pub async fn html(
        &self,
        email_id: impl AsRef<str>,
    ) -> Result<Response<WithEtag<EmailHtml>, EmailResponseMetadata>, Error> {
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
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = client.emails().get_block("123", "456").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    pub async fn get_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
    ) -> Result<Response<WithEtag<EmailBlock>, EmailResponseMetadata>, Error> {
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
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::{models::emails::BlockContent, resources::emails::NewBlock};
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let block = NewBlock::new(BlockContent::Divider(Default::default()));
    ///
    /// let response = client.emails().create_block("123", "current-etag", &block).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// An idempotency key that cannot be encoded as an HTTP header also fails locally.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    /// Writes are never automatically retried. A failure after dispatch may leave the
    /// server outcome uncertain; reuse the same idempotency key only for the same write.
    /// A stale etag can return HTTP 409; inspect [`crate::ApiError::is_version_conflict`].
    pub async fn create_block(
        &self,
        email_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        block: &NewBlock,
    ) -> Result<Response<WithEtag<EmailBlock>, EmailResponseMetadata>, Error> {
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
    /// position of the block do not change. Only the document is sent; its variant selects
    /// the SDK encoding, not a server-side type assertion. The server validates content
    /// against the stored block kind. No implicit read is performed. Returns the block and new etag.
    ///
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::{MailerLite, models::emails::BlockContent};
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let content = BlockContent::paragraph("Updated text");
    ///
    /// let response = client.emails().update_block("123", "456", "current-etag", &content).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// An idempotency key that cannot be encoded as an HTTP header also fails locally.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    /// Writes are never automatically retried. A failure after dispatch may leave the
    /// server outcome uncertain; reuse the same idempotency key only for the same write.
    /// A stale etag can return HTTP 409; inspect [`crate::ApiError::is_version_conflict`].
    pub async fn update_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        editor_content: &BlockContent,
    ) -> Result<Response<WithEtag<EmailBlock>, EmailResponseMetadata>, Error> {
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
                editor_content: ContentRef(editor_content),
                etag,
            }),
            idempotency_key,
            OperationPolicy::WRITE.with_success_statuses(&[200]),
        )
        .await
    }

    /// Deletes a block and the blocks in it. The response has only the new etag.
    ///
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = client.emails().delete_block("123", "456", "current-etag").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// An idempotency key that cannot be encoded as an HTTP header also fails locally.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    /// Writes are never automatically retried. A failure after dispatch may leave the
    /// server outcome uncertain; reuse the same idempotency key only for the same write.
    /// A stale etag can return HTTP 409; inspect [`crate::ApiError::is_version_conflict`].
    pub async fn delete_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
        etag: impl AsRef<str>,
    ) -> Result<Response<EtagOnly, EmailResponseMetadata>, Error> {
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
                body: &Map::<String, Value>::new(),
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
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::emails::Placement;
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let placement = Placement::after("789");
    ///
    /// let response = client.emails().move_block("123", "456", "current-etag", &placement).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// An idempotency key that cannot be encoded as an HTTP header also fails locally.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    /// Writes are never automatically retried. A failure after dispatch may leave the
    /// server outcome uncertain; reuse the same idempotency key only for the same write.
    /// A stale etag can return HTTP 409; inspect [`crate::ApiError::is_version_conflict`].
    pub async fn move_block(
        &self,
        email_id: impl AsRef<str>,
        block_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        placement: &Placement,
    ) -> Result<Response<WithEtag<EmailBlock>, EmailResponseMetadata>, Error> {
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
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::{models::emails::BlockContent, resources::emails::NewBlock};
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let block = NewBlock::new(BlockContent::Divider(Default::default()));
    ///
    /// let response = client.emails().validate_block(&block).await?;
    ///
    /// println!("Valid: {}", response.data.valid);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    /// Invalid input can also return a successful response with `valid: false`.
    pub async fn validate_block(
        &self,
        block: &NewBlock,
    ) -> Result<Response<Validation, EmailResponseMetadata>, Error> {
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
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::{models::emails::BrandMode, resources::emails::{BrandUpdate, EmailSettingsUpdate}};
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let settings = EmailSettingsUpdate {
    ///     brand: Some(BrandUpdate { mode: Some(BrandMode::Custom), ..Default::default() }),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.emails().update_settings("123", "current-etag", &settings).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// An idempotency key that cannot be encoded as an HTTP header also fails locally.
    /// Duplicate typed keys in extension maps return [`Error::Config`].
    /// A top-level `etag` extension key is rejected because the SDK supplies it.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    /// Writes are never automatically retried. A failure after dispatch may leave the
    /// server outcome uncertain; reuse the same idempotency key only for the same write.
    /// A stale etag can return HTTP 409; inspect [`crate::ApiError::is_version_conflict`].
    pub async fn update_settings(
        &self,
        email_id: impl AsRef<str>,
        etag: impl AsRef<str>,
        settings: &EmailSettingsUpdate,
    ) -> Result<Response<SettingsResult, EmailResponseMetadata>, Error> {
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
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::emails::EmailSettingsUpdate;
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let settings = EmailSettingsUpdate::default();
    ///
    /// let response = client.emails().validate_settings("123", &settings).await?;
    ///
    /// println!("Valid: {}", response.data.valid);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// Duplicate typed keys in extension maps return [`Error::Config`].
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    /// Invalid input can also return a successful response with `valid: false`.
    pub async fn validate_settings(
        &self,
        email_id: impl AsRef<str>,
        settings: &EmailSettingsUpdate,
    ) -> Result<Response<Validation, EmailResponseMetadata>, Error> {
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
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = client.emails().create_template("123", "Newsletter").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// An idempotency key that cannot be encoded as an HTTP header also fails locally.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    /// Writes are never automatically retried. A failure after dispatch may leave the
    /// server outcome uncertain; reuse the same idempotency key only for the same write.
    pub async fn create_template(
        &self,
        email_id: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Result<Response<EmailTemplate, EmailResponseMetadata>, Error> {
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
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    /// use mailerlite_rs::resources::emails::ListVersionsOptions;
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let options = ListVersionsOptions { page: Some(1), limit: Some(10) };
    ///
    /// let response = client.emails().versions("123", options).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    pub async fn versions(
        &self,
        email_id: impl AsRef<str>,
        options: ListVersionsOptions,
    ) -> Result<Response<NumberedPage<EmailVersion>, EmailResponseMetadata>, Error> {
        let email_id: &str = email_id.as_ref();
        validate::path_id("email_id", email_id)?;
        let query: Vec<(&str, String)> = query_pairs([
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
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = client.emails().create_version("123").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// An idempotency key that cannot be encoded as an HTTP header also fails locally.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    /// Writes are never automatically retried. A failure after dispatch may leave the
    /// server outcome uncertain; reuse the same idempotency key only for the same write.
    pub async fn create_version(
        &self,
        email_id: impl AsRef<str>,
    ) -> Result<Response<EmailVersion, EmailResponseMetadata>, Error> {
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
    /// # Examples
    /// ```no_run
    /// use mailerlite_rs::MailerLite;
    ///
    /// # async fn example(client: &MailerLite) -> mailerlite_rs::Result<()> {
    /// let response = client.emails().restore_version("123", "901", "current-etag").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`Error::Config`] before dispatch for empty or non-ASCII-digit path IDs.
    /// An idempotency key that cannot be encoded as an HTTP header also fails locally.
    /// HTTP failures return [`Error::Api`]; inspect its status and validation messages.
    /// Transport, serialization, decoding, and deadline failures retain the shared client error contract.
    /// Writes are never automatically retried. A failure after dispatch may leave the
    /// server outcome uncertain; reuse the same idempotency key only for the same write.
    /// A stale etag can return HTTP 409; inspect [`crate::ApiError::is_version_conflict`].
    pub async fn restore_version(
        &self,
        email_id: impl AsRef<str>,
        version_id: impl AsRef<str>,
        etag: impl AsRef<str>,
    ) -> Result<Response<WithEtag<EmailDocument>, EmailResponseMetadata>, Error> {
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
                body: &Map::<String, Value>::new(),
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
) -> crate::Result<Response<T, EmailResponseMetadata>> {
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
    crate::http::request_with_metadata(
        client,
        method,
        path,
        &query,
        body,
        policy,
        |http, idempotent_replayed| EmailResponseMetadata {
            http,
            idempotent_replayed,
        },
    )
    .await
}
