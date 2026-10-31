//! Typed JSON and empty-response decoding.
use super::{OperationPolicy, RequestBody, deadline::Deadline, execution::execute};
use crate::{Error, MailerLite, Response, Result, error::DecodeError};
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
    let deadline = Deadline::new(client.inner.operation_timeout)?;
    deadline
        .run(async {
            let (metadata, bytes) =
                execute(client, method, path, query, body, policy, deadline).await?;
            deadline.check()?;
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
            let (metadata, bytes) = execute(
                client,
                method,
                path,
                query,
                body,
                policy.with_success_statuses(&[204]),
                deadline,
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
        })
        .await
}

#[cfg(test)]
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
    async fn retryable_read_preserves_body_without_serializing_again() {
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

        // Exercise the shared executor's body handling on a synthetic safe read.
        let server = MockServer::start().await;
        let attempts = Arc::new(AtomicUsize::new(0));
        let received = Arc::clone(&attempts);
        Mock::given(method("GET"))
            .and(body_string(r#""same payload on every attempt""#))
            .respond_with(move |_: &wiremock::Request| {
                let status = if received.fetch_add(1, Ordering::SeqCst) < 2 {
                    503
                } else {
                    200
                };
                ResponseTemplate::new(status).set_body_json(true)
            })
            .expect(3)
            .mount(&server)
            .await;
        let client = client(&server, Duration::from_secs(5));
        let payload = CountedPayload(AtomicUsize::new(0));
        let response = request::<bool>(
            &client,
            Method::GET,
            &["test"],
            &[],
            Some(&payload),
            OperationPolicy::READ,
        )
        .await
        .unwrap();

        assert!(response.data);
        assert_eq!(payload.0.load(Ordering::SeqCst), 1);
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
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
