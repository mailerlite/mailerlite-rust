//! Typed JSON and empty-response decoding.
use super::{OperationPolicy, execution::execute};
use crate::{Error, MailerLite, Response, Result, error::DecodeError};
use reqwest::Method;
use serde::de::DeserializeOwned;

pub(crate) async fn request<T: DeserializeOwned>(
    client: &MailerLite,
    method: Method,
    path: &[&str],
    query: &[(String, String)],
    body: Option<serde_json::Value>,
    policy: OperationPolicy,
) -> Result<Response<T>> {
    let (metadata, bytes) = execute(client, method, path, query, body, policy).await?;
    let mut deserializer = serde_json::Deserializer::from_slice(&bytes);
    let data = serde_path_to_error::deserialize(&mut deserializer).map_err(|error| {
        Error::Decode(DecodeError::new(
            metadata.clone(),
            &bytes,
            error.to_string(),
        ))
    })?;
    deserializer.end().map_err(|error| {
        Error::Decode(DecodeError::new(
            metadata.clone(),
            &bytes,
            error.to_string(),
        ))
    })?;
    Ok(Response { data, metadata })
}

pub(crate) async fn empty(
    client: &MailerLite,
    method: Method,
    path: &[&str],
    query: &[(String, String)],
    body: Option<serde_json::Value>,
    policy: OperationPolicy,
) -> Result<Response<()>> {
    let (metadata, bytes) = execute(
        client,
        method,
        path,
        query,
        body,
        policy.with_success_statuses(&[204]),
    )
    .await?;
    if !bytes.is_empty() {
        return Err(Error::Decode(DecodeError::new(
            metadata,
            &bytes,
            "expected an empty response body".into(),
        )));
    }
    Ok(Response { data: (), metadata })
}
