//! Execute requests with authentication, deadlines, pacing, and retry policy.
use super::{deadline::Deadline, endpoint::endpoint, request::Request};
use crate::{
    ApiVersion, Error, MailerLite, ResponseMetadata, Result,
    error::{ApiError, DecodeError},
};
use bytes::Bytes;
use reqwest::{
    StatusCode,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, USER_AGENT},
};
use std::time::{Duration, SystemTime};

fn metadata(status: StatusCode, headers: &HeaderMap) -> ResponseMetadata {
    let integer = |name: &str| {
        headers
            .get(name)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())
    };
    let retry_after = headers
        .get("retry-after")
        .and_then(|h| h.to_str().ok())
        .and_then(|value| {
            value
                .parse::<u64>()
                .ok()
                .map(Duration::from_secs)
                .or_else(|| {
                    httpdate::parse_http_date(value)
                        .ok()
                        .map(|date| date.duration_since(SystemTime::now()).unwrap_or_default())
                })
        });
    ResponseMetadata {
        status,
        rate_limit: integer("x-ratelimit-limit"),
        rate_remaining: integer("x-ratelimit-remaining"),
        retry_after,
        request_id: headers
            .get("x-request-id")
            .and_then(|h| h.to_str().ok())
            .map(str::to_owned),
    }
}

pub(super) async fn execute<T>(
    client: &MailerLite,
    request: Request<'_>,
    deadline: Deadline,
    decode: impl Fn(&ResponseMetadata, &[u8]) -> Result<T>,
) -> Result<(ResponseMetadata, T, bool)> {
    let Request {
        method,
        path,
        query,
        body,
        options,
    } = request;
    let url = endpoint(&client.inner.base_url, path)?;
    let inner = &client.inner;
    let policy = options.policy;
    let retry_keyed_write =
        !policy.read_only && options.idempotency_key.is_some() && inner.retry.idempotent_writes;
    let retry_safe = policy.read_only || retry_keyed_write;
    let serialized = body.map(|body| body.serialize()).transpose();
    deadline.check()?;
    // Take ownership of the buffer; each attempt clones only a shared handle.
    let serialized = serialized?.map(Bytes::from);
    let mut attempt = 0;
    loop {
        deadline.check()?;
        inner.limiter.wait(policy.import).await;
        deadline.check()?;
        let mut request = inner
            .http
            .request(method.clone(), url.clone())
            .query(query)
            .header(AUTHORIZATION, inner.authorization.clone())
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(
                USER_AGENT,
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
            )
            .timeout(inner.request_timeout);
        if let Some(key) = &options.idempotency_key {
            request = request.header("Idempotency-Key", key.clone());
        }
        if let ApiVersion::Pinned(version) = &inner.version {
            request = request.header("X-Version", version.as_str());
        }
        if let Some(body) = &serialized {
            request = request.body(body.clone());
        }
        deadline.check()?;
        let result = request.send().await;
        let mut response = match result {
            Ok(response) => response,
            Err(error) => {
                let transient = error.is_connect() || error.is_timeout() || error.is_body();
                if retry_safe
                    && transient
                    && let Some(delay) = inner.retry.delay_for(attempt, None)
                    && delay < deadline.remaining()
                {
                    tokio::time::sleep(delay).await;
                    attempt += 1;
                    continue;
                }
                return Err(Error::Transport(error.without_url()));
            }
        };
        let replayed = response
            .headers()
            .get("Idempotent-Replayed")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.trim().eq_ignore_ascii_case("true"));
        let meta = metadata(response.status(), response.headers());
        #[cfg(feature = "tracing")]
        tracing::debug!(
            attempt,
            status = meta.status.as_u16(),
            "MailerLite API response"
        );
        if let Some(delay) = meta.retry_after
            && (meta.status == StatusCode::TOO_MANY_REQUESTS
                || meta.status == StatusCode::SERVICE_UNAVAILABLE)
        {
            // A shared token may have exhausted either bucket; do not assume
            // an import response proves the global bucket is still available.
            // The limiter caps shared cooldowns; keep the original delay in
            // metadata and automatic retry decisions below.
            inner.limiter.cooldown(false, delay).await;
            if policy.import {
                inner.limiter.cooldown(true, delay).await;
            }
        }
        let mut bytes = Vec::new();
        let mut body_failure = None;
        while let Some(chunk) = match response.chunk().await {
            Ok(chunk) => chunk,
            Err(error) => {
                body_failure = Some(error);
                None
            }
        } {
            if chunk.len() > inner.max_response_bytes.saturating_sub(bytes.len()) {
                let remaining = inner.max_response_bytes.saturating_sub(bytes.len());
                bytes.extend_from_slice(&chunk[..remaining]);
                return Err(Error::Decode(DecodeError::incomplete(
                    meta,
                    &bytes,
                    "response exceeds configured byte limit".into(),
                )));
            }
            bytes.extend_from_slice(&chunk);
        }
        if let Some(error) = body_failure {
            // A torn conflict body cannot reliably distinguish a pending key
            // from other conflicts. Recover its response with the same write;
            // stop if a complete machine-readable conflict code was received.
            let interrupted_conflict = retry_keyed_write
                && meta.status == StatusCode::CONFLICT
                && ApiError::new(meta.clone(), &bytes).code().is_none();
            let retry_after = conflict_delay(meta.retry_after, interrupted_conflict);
            if retry_safe
                && ((meta.status.is_success()
                    && policy.success_statuses.contains(&meta.status.as_u16()))
                    || inner.retry.permits_status(meta.status)
                    || interrupted_conflict)
                && let Some(delay) = inner.retry.delay_for(attempt, retry_after)
                && delay < deadline.remaining()
            {
                tokio::time::sleep(delay).await;
                attempt += 1;
                continue;
            }
            return Err(Error::Decode(DecodeError::incomplete(
                meta,
                &bytes,
                format!("response body could not be read: {}", error.without_url()),
            )));
        }
        if meta.status.is_success() && !policy.success_statuses.contains(&meta.status.as_u16()) {
            return Err(Error::Decode(DecodeError::new(
                meta,
                &bytes,
                "unexpected success status for this operation".into(),
            )));
        }
        if meta.status.is_success() {
            deadline.check()?;
            match decode(&meta, &bytes) {
                Ok(data) => return Ok((meta, data, replayed)),
                Err(error) => {
                    deadline.check()?;
                    // Only a started JSON document with an EOF error suggests
                    // truncation. Empty bodies and schema errors need investigation.
                    let incomplete = bytes.iter().any(|byte| !byte.is_ascii_whitespace())
                        && matches!(&error, Error::Decode(error)
                            if error.json_error().is_some_and(serde_json::Error::is_eof));
                    if retry_safe
                        && incomplete
                        && let Some(delay) = inner.retry.delay_for(attempt, meta.retry_after)
                        && delay < deadline.remaining()
                    {
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(error);
                }
            }
        }
        let error = ApiError::new(meta, &bytes);
        let in_progress = retry_keyed_write && error.is_idempotency_in_progress();
        // The API supplies no Retry-After for pending keys. Avoid polling the
        // still-running write immediately even when jitter selects zero delay.
        let retry_after = conflict_delay(error.metadata().retry_after, in_progress);
        if retry_safe
            && (inner.retry.permits_status(error.status()) || in_progress)
            && let Some(delay) = inner.retry.delay_for(attempt, retry_after)
            && delay < deadline.remaining()
        {
            tokio::time::sleep(delay).await;
            attempt += 1;
            continue;
        }
        return Err(Error::Api(error));
    }
}

fn conflict_delay(retry_after: Option<Duration>, pending_or_interrupted: bool) -> Option<Duration> {
    if pending_or_interrupted {
        Some(retry_after.unwrap_or_default().max(Duration::from_secs(1)))
    } else {
        retry_after
    }
}
