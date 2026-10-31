//! Execute requests with authentication, deadlines, pacing, and retry policy.
use super::{OperationPolicy, RequestBody, deadline::Deadline, endpoint::endpoint};
use crate::{
    ApiVersion, Error, MailerLite, ResponseMetadata, Result,
    error::{ApiError, DecodeError},
};
use bytes::Bytes;
use reqwest::{
    Method, StatusCode, Url,
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

pub(super) async fn execute(
    client: &MailerLite,
    method: Method,
    path: &[&str],
    query: &[(String, String)],
    body: Option<&dyn RequestBody>,
    policy: OperationPolicy,
    deadline: Deadline,
) -> Result<(ResponseMetadata, Vec<u8>)> {
    let url = endpoint(&client.inner.base_url, path)?;
    execute_until(client, method, url, query, body, policy, deadline).await
}

async fn execute_until(
    client: &MailerLite,
    method: Method,
    url: Url,
    query: &[(String, String)],
    body: Option<&dyn RequestBody>,
    policy: OperationPolicy,
    deadline: Deadline,
) -> Result<(ResponseMetadata, Vec<u8>)> {
    let inner = &client.inner;
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
        if let ApiVersion::Pinned(version) = &inner.version {
            request = request.header("X-Version", version);
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
                if policy.read_only
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
            if policy.read_only
                && (meta.status.is_success() || inner.retry.permits_status(meta.status))
                && let Some(delay) = inner.retry.delay_for(attempt, meta.retry_after)
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
            return Ok((meta, bytes));
        }
        if policy.read_only
            && inner.retry.permits_status(meta.status)
            && let Some(delay) = inner.retry.delay_for(attempt, meta.retry_after)
            && delay < deadline.remaining()
        {
            tokio::time::sleep(delay).await;
            attempt += 1;
            continue;
        }
        return Err(Error::Api(ApiError::new(meta, &bytes)));
    }
}
