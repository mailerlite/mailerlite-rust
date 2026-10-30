//! Bounded retry timing. The transport separately enforces operation safety.
use reqwest::StatusCode;
use std::time::Duration;

/// Retry timing for operations explicitly classified as safe reads.
///
/// Automatic retries are disabled by default. Opt in with [`Self::safe_reads`].
/// Writes are never automatically replayed, irrespective of this configuration.
/// A server's `Retry-After` is a minimum delay and is not shortened by the cap.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    max_retries: u32,
    initial_delay: Duration,
    max_delay: Duration,
}
impl Default for RetryPolicy {
    fn default() -> Self {
        Self::none()
    }
}
impl RetryPolicy {
    /// Retry safe reads at most twice, with 500 ms initial and 8 s capped backoff.
    pub fn safe_reads() -> Self {
        Self {
            max_retries: 2,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(8),
        }
    }
    /// Disable automatic retries.
    pub fn none() -> Self {
        Self::safe_reads().max_retries(0)
    }
    /// Set the number of retries after the original request.
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }
    /// Set the initial exponential-backoff ceiling, before jitter.
    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }
    /// Set the backoff ceiling, excluding a larger server-directed minimum.
    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }
    /// Maximum number of retry attempts after the original request.
    pub fn retry_limit(&self) -> u32 {
        self.max_retries
    }
    pub(crate) fn permits_status(&self, status: StatusCode) -> bool {
        matches!(status.as_u16(), 408 | 429 | 500 | 502 | 503 | 504)
    }
    pub(crate) fn delay_for(
        &self,
        attempt: u32,
        retry_after: Option<Duration>,
    ) -> Option<Duration> {
        self.delay_with_sample(attempt, retry_after, fastrand::u32(..))
    }
    fn delay_with_sample(
        &self,
        attempt: u32,
        retry_after: Option<Duration>,
        sample: u32,
    ) -> Option<Duration> {
        if attempt >= self.max_retries {
            return None;
        }
        let multiplier = 2u32.checked_pow(attempt).unwrap_or(u32::MAX);
        let ceiling = self
            .initial_delay
            .saturating_mul(multiplier)
            .min(self.max_delay);
        // The product fits u128 even for Duration::MAX and u32::MAX.
        let nanos = ceiling.as_nanos() * u128::from(sample) / u128::from(u32::MAX);
        let jittered = Duration::new(
            (nanos / 1_000_000_000) as u64,
            (nanos % 1_000_000_000) as u32,
        );
        Some(jittered.max(retry_after.unwrap_or_default()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retries_are_bounded_and_server_delay_is_not_capped() {
        let policy = RetryPolicy::safe_reads();
        assert_eq!(
            policy.delay_with_sample(0, None, u32::MAX),
            Some(Duration::from_millis(500))
        );
        assert_eq!(
            policy.delay_with_sample(1, None, u32::MAX),
            Some(Duration::from_secs(1))
        );
        assert_eq!(policy.delay_with_sample(2, None, u32::MAX), None);
        assert_eq!(
            policy.delay_with_sample(0, Some(Duration::from_secs(119)), 0),
            Some(Duration::from_secs(119))
        );
        assert_eq!(RetryPolicy::none().delay_for(0, None), None);
    }
    #[test]
    fn retryable_statuses_exclude_validation_and_authentication() {
        let policy = RetryPolicy::default();
        for status in [
            StatusCode::REQUEST_TIMEOUT,
            StatusCode::TOO_MANY_REQUESTS,
            StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::BAD_GATEWAY,
            StatusCode::SERVICE_UNAVAILABLE,
            StatusCode::GATEWAY_TIMEOUT,
        ] {
            assert!(policy.permits_status(status));
        }
        for status in [
            StatusCode::OK,
            StatusCode::BAD_REQUEST,
            StatusCode::UNAUTHORIZED,
            StatusCode::FORBIDDEN,
            StatusCode::NOT_FOUND,
            StatusCode::UNPROCESSABLE_ENTITY,
            StatusCode::NOT_IMPLEMENTED,
        ] {
            assert!(!policy.permits_status(status));
        }
    }
}
