//! Typed JSON and empty-response decoding.
use super::{
    OperationPolicy, RequestBody, RequestOptions, deadline::Deadline, execution::execute,
    request::Request,
};
use crate::{Error, MailerLite, Response, ResponseMetadata, Result, error::DecodeError};
use reqwest::Method;
use serde::de::DeserializeOwned;

pub(crate) async fn request<T: DeserializeOwned>(
    client: &MailerLite,
    method: Method,
    path: &[&str],
    query: &[(String, String)],
    body: Option<&dyn RequestBody>,
    policy: OperationPolicy,
) -> Result<Response<T>> {
    request_with_metadata(
        client,
        method,
        path,
        query,
        body,
        policy.into(),
        |metadata, _| metadata,
    )
    .await
}

pub(crate) async fn request_with_metadata<T: DeserializeOwned, M>(
    client: &MailerLite,
    method: Method,
    path: &[&str],
    query: &[(String, String)],
    body: Option<&dyn RequestBody>,
    options: RequestOptions,
    map_metadata: impl FnOnce(crate::ResponseMetadata, bool) -> M,
) -> Result<Response<T, M>> {
    let deadline = Deadline::new(client.inner.operation_timeout)?;
    deadline
        .run(async {
            let (metadata, data, replayed) = execute(
                client,
                Request {
                    method,
                    path,
                    query,
                    body,
                    options,
                },
                deadline,
                decode_json::<T>,
            )
            .await?;
            Ok(Response {
                data,
                metadata: map_metadata(metadata, replayed),
            })
        })
        .await
}

pub(crate) async fn empty(
    client: &MailerLite,
    method: Method,
    path: &[&str],
    query: &[(String, String)],
    body: Option<&dyn RequestBody>,
    policy: OperationPolicy,
) -> Result<Response<()>> {
    let deadline = Deadline::new(client.inner.operation_timeout)?;
    deadline
        .run(async {
            let (metadata, (), _) = execute(
                client,
                Request {
                    method,
                    path,
                    query,
                    body,
                    options: policy.with_success_statuses(&[204]).into(),
                },
                deadline,
                decode_empty,
            )
            .await?;
            Ok(Response { data: (), metadata })
        })
        .await
}

fn decode_json<T: DeserializeOwned>(metadata: &ResponseMetadata, bytes: &[u8]) -> Result<T> {
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let data = serde_path_to_error::deserialize(&mut deserializer).map_err(|error| {
        Error::Decode(DecodeError::json(
            metadata.clone(),
            bytes,
            error.to_string(),
            error.into_inner(),
        ))
    })?;
    deserializer.end().map_err(|error| {
        Error::Decode(DecodeError::json(
            metadata.clone(),
            bytes,
            error.to_string(),
            error,
        ))
    })?;
    Ok(data)
}

fn decode_empty(metadata: &ResponseMetadata, bytes: &[u8]) -> Result<()> {
    if bytes.is_empty() {
        Ok(())
    } else {
        Err(Error::Decode(DecodeError::new(
            metadata.clone(),
            bytes,
            "expected an empty response body".into(),
        )))
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use super::*;
    use crate::RetryPolicy;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::{cell::Cell, time::Duration};
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    struct Payload {
        delay: Duration,
        fail: bool,
    }

    impl Serialize for Payload {
        fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
            // A blocking delay deliberately models serde work that cannot yield
            // to a Tokio timer. Tests assert outcomes, not elapsed upper bounds.
            std::thread::sleep(self.delay);
            if self.fail {
                Err(serde::ser::Error::custom("invalid test payload"))
            } else {
                serializer.serialize_bool(true)
            }
        }
    }

    thread_local! {
        static DECODED: Cell<bool> = const { Cell::new(false) };
    }

    struct SlowResponse;

    impl<'de> Deserialize<'de> for SlowResponse {
        fn deserialize<D: Deserializer<'de>>(
            deserializer: D,
        ) -> std::result::Result<Self, D::Error> {
            let fail = bool::deserialize(deserializer)?;
            DECODED.set(true);
            std::thread::sleep(Duration::from_millis(1100));
            if fail {
                Err(serde::de::Error::custom("invalid test response"))
            } else {
                Ok(Self)
            }
        }
    }

    fn client(server: &MockServer, timeout: Duration) -> MailerLite {
        MailerLite::builder("test-token")
            .base_url(server.uri())
            .operation_timeout(timeout)
            .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
            .retry_policy(RetryPolicy::safe_reads().initial_delay(Duration::ZERO))
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn retryable_operations_preserve_body_without_serializing_again() {
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        use wiremock::matchers::body_string;

        struct CountedPayload(AtomicUsize);
        impl Serialize for CountedPayload {
            fn serialize<S: Serializer>(
                &self,
                serializer: S,
            ) -> std::result::Result<S::Ok, S::Error> {
                self.0.fetch_add(1, Ordering::SeqCst);
                serializer.serialize_str("same payload on every attempt")
            }
        }

        thread_local! {
            static DECODE_ATTEMPTS: Cell<usize> = const { Cell::new(0) };
        }
        struct CountedResponse(bool);
        impl<'de> Deserialize<'de> for CountedResponse {
            fn deserialize<D: Deserializer<'de>>(
                deserializer: D,
            ) -> std::result::Result<Self, D::Error> {
                DECODE_ATTEMPTS.set(DECODE_ATTEMPTS.get() + 1);
                bool::deserialize(deserializer).map(Self)
            }
        }

        for (verb, options) in [
            (Method::GET, OperationPolicy::READ.into()),
            (
                Method::POST,
                RequestOptions {
                    policy: OperationPolicy::WRITE,
                    idempotency_key: Some(reqwest::header::HeaderValue::from_static("same-write")),
                },
            ),
        ] {
            let server = MockServer::start().await;
            let attempts = Arc::new(AtomicUsize::new(0));
            let received = Arc::clone(&attempts);
            Mock::given(method(verb.as_str()))
                .and(body_string(r#""same payload on every attempt""#))
                .respond_with(move |_: &wiremock::Request| {
                    match received.fetch_add(1, Ordering::SeqCst) {
                        0 => ResponseTemplate::new(503),
                        1 => ResponseTemplate::new(200).set_body_string("tru"),
                        _ => ResponseTemplate::new(200).set_body_json(true),
                    }
                })
                .expect(3)
                .mount(&server)
                .await;
            let client = MailerLite::builder("test-token")
                .base_url(server.uri())
                .rate_limit_intervals(Duration::ZERO, Duration::ZERO)
                .retry_policy(
                    RetryPolicy::safe_reads()
                        .with_idempotent_writes()
                        .initial_delay(Duration::ZERO),
                )
                .build()
                .unwrap();
            let payload = CountedPayload(AtomicUsize::new(0));
            DECODE_ATTEMPTS.set(0);
            let response = request_with_metadata::<CountedResponse, _>(
                &client,
                verb,
                &["test"],
                &[],
                Some(&payload),
                options,
                |metadata, _| metadata,
            )
            .await
            .unwrap();

            assert!(response.data.0);
            assert_eq!(
                DECODE_ATTEMPTS.get(),
                2,
                "one typed decode per success-status body, including the EOF failure"
            );
            assert_eq!(payload.0.load(Ordering::SeqCst), 1);
            assert_eq!(attempts.load(Ordering::SeqCst), 3);
        }
    }

    async fn serialization_result(delay: Duration, fail: bool) -> Result<Response<bool>> {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(true))
            .expect(0)
            .mount(&server)
            .await;
        let timeout = if delay.is_zero() {
            Duration::from_secs(1)
        } else {
            Duration::from_millis(10)
        };
        let client = client(&server, timeout);
        let result = request(
            &client,
            Method::POST,
            &["test"],
            &[],
            Some(&Payload { delay, fail }),
            OperationPolicy::WRITE,
        )
        .await;
        assert!(server.received_requests().await.unwrap().is_empty());
        result
    }

    #[tokio::test]
    async fn serialization_overrun_prevents_dispatch() {
        assert!(matches!(
            serialization_result(Duration::from_millis(20), false).await,
            Err(Error::Timeout)
        ));
    }

    #[tokio::test]
    async fn late_serialization_error_returns_timeout() {
        assert!(matches!(
            serialization_result(Duration::from_millis(20), true).await,
            Err(Error::Timeout)
        ));
    }

    #[tokio::test]
    async fn serialization_error_within_budget_is_preserved() {
        assert!(matches!(
            serialization_result(Duration::ZERO, true).await,
            Err(Error::Serialize(_))
        ));
    }

    async fn assert_decoding_timeout(fail: bool) {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(fail))
            .expect(1)
            .mount(&server)
            .await;
        let client = client(&server, Duration::from_secs(1));
        DECODED.set(false);
        let result = request::<SlowResponse>(
            &client,
            Method::GET,
            &["test"],
            &[],
            None,
            OperationPolicy::READ,
        )
        .await;
        assert!(
            DECODED.get(),
            "the response must reach synchronous decoding"
        );
        assert!(matches!(result, Err(Error::Timeout)));
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn decoding_overrun_returns_timeout_without_retry() {
        assert_decoding_timeout(false).await;
    }

    #[tokio::test]
    async fn late_decoding_error_returns_timeout_without_retry() {
        assert_decoding_timeout(true).await;
    }
}
