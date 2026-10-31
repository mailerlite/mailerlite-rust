//! One deadline shared by serialization, HTTP attempts, and response decoding.
use crate::{Error, Result};
use std::{future::Future, time::Duration};
use tokio::time::Instant;

#[derive(Clone, Copy)]
pub(super) struct Deadline(Instant);

impl Deadline {
    pub(super) fn new(timeout: Duration) -> Result<Self> {
        Instant::now()
            .checked_add(timeout)
            .map(Self)
            .ok_or_else(|| Error::Config("operation timeout is too large".into()))
    }

    pub(super) fn check(self) -> Result<()> {
        if Instant::now() >= self.0 {
            Err(Error::Timeout)
        } else {
            Ok(())
        }
    }

    pub(super) fn remaining(self) -> Duration {
        self.0.saturating_duration_since(Instant::now())
    }

    pub(super) async fn run<T>(self, operation: impl Future<Output = Result<T>>) -> Result<T> {
        self.check()?;
        let result = tokio::time::timeout_at(self.0, operation)
            .await
            .map_err(|_| Error::Timeout)?;
        // Synchronous serde work may finish without yielding to Tokio's timer.
        // Never return a late success or decoding error as an on-time result.
        self.check()?;
        result
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests assert fixture and operation outcomes"
)]
mod tests {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn a_ready_result_after_the_deadline_is_rejected() {
        let deadline = Deadline::new(Duration::from_secs(1)).unwrap();
        let result = deadline
            .run(async {
                tokio::time::advance(Duration::from_secs(2)).await;
                Ok(())
            })
            .await;
        assert!(matches!(result, Err(Error::Timeout)));
    }

    #[tokio::test(start_paused = true)]
    async fn an_expired_deadline_does_not_poll_the_operation() {
        let deadline = Deadline::new(Duration::from_secs(1)).unwrap();
        tokio::time::advance(Duration::from_secs(1)).await;
        let result: Result<()> = deadline.run(async { panic!("must not be polled") }).await;
        assert!(matches!(result, Err(Error::Timeout)));
    }
}
