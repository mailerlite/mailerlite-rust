//! Shared pacing and server-directed cooldowns.
use std::{future::pending, time::Duration};
use tokio::{
    sync::Mutex,
    time::{Instant, sleep_until},
};

pub(crate) struct RateLimiter {
    global_interval: Duration,
    import_interval: Duration,
    state: Mutex<State>,
}
struct State {
    // None represents a deadline beyond Instant's representable range. This
    // deliberately prevents early requests; the operation deadline still bounds
    // every wait. Clamping a server cooldown would violate Retry-After.
    next_global: Option<Instant>,
    next_import: Option<Instant>,
}
impl RateLimiter {
    pub(crate) fn new(global_interval: Duration, import_interval: Duration) -> Self {
        let now = Instant::now();
        Self {
            global_interval,
            import_interval,
            state: Mutex::new(State {
                next_global: Some(now),
                next_import: Some(now),
            }),
        }
    }
    pub(crate) async fn wait(&self, import: bool) {
        loop {
            let mut state = self.state.lock().await;
            let now = Instant::now();
            let deadline = if import {
                state
                    .next_global
                    .zip(state.next_import)
                    .map(|(global, import)| global.max(import))
            } else {
                state.next_global
            };
            if deadline.is_some_and(|deadline| deadline <= now) {
                state.next_global = now.checked_add(self.global_interval);
                if import {
                    state.next_import = now.checked_add(self.import_interval);
                }
                return;
            }
            drop(state);
            match deadline {
                Some(deadline) => sleep_until(deadline).await,
                None => pending::<()>().await,
            }
        }
    }
    pub(crate) async fn cooldown(&self, import: bool, delay: Duration) {
        let mut state = self.state.lock().await;
        let target = if import {
            &mut state.next_import
        } else {
            &mut state.next_global
        };
        *target = target
            .zip(Instant::now().checked_add(delay))
            .map(|(old, new)| old.max(new));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test(start_paused = true)]
    async fn import_and_global_buckets_are_distinct() {
        let limiter = RateLimiter::new(Duration::from_millis(500), Duration::from_secs(12));
        let start = Instant::now();
        limiter.wait(true).await;
        limiter.wait(false).await;
        assert_eq!(start.elapsed(), Duration::from_millis(500));
        limiter.wait(true).await;
        assert_eq!(start.elapsed(), Duration::from_secs(12));
    }
    #[tokio::test(start_paused = true)]
    async fn cooldown_is_a_minimum() {
        let limiter = RateLimiter::new(Duration::ZERO, Duration::ZERO);
        let start = Instant::now();
        limiter.cooldown(false, Duration::from_secs(60)).await;
        limiter.cooldown(false, Duration::from_secs(1)).await;
        limiter.wait(false).await;
        assert_eq!(start.elapsed(), Duration::from_secs(60));
    }

    #[tokio::test(start_paused = true)]
    async fn unrepresentable_global_cooldown_remains_bounded_by_operation_deadline() {
        let limiter = RateLimiter::new(Duration::ZERO, Duration::ZERO);
        limiter.cooldown(false, Duration::from_secs(u64::MAX)).await;
        // A later, shorter response must not remove the existing minimum.
        limiter.cooldown(false, Duration::from_secs(1)).await;

        for import in [false, true] {
            let start = Instant::now();
            let result = tokio::time::timeout(Duration::from_secs(2), limiter.wait(import)).await;

            assert!(result.is_err());
            assert_eq!(start.elapsed(), Duration::from_secs(2));
        }
    }

    #[tokio::test(start_paused = true)]
    async fn unrepresentable_import_cooldown_does_not_block_non_import_requests() {
        let limiter = RateLimiter::new(Duration::ZERO, Duration::ZERO);
        limiter.cooldown(true, Duration::from_secs(u64::MAX)).await;
        let start = Instant::now();

        assert!(
            tokio::time::timeout(Duration::from_secs(2), limiter.wait(true))
                .await
                .is_err()
        );
        limiter.wait(false).await;

        assert_eq!(start.elapsed(), Duration::from_secs(2));
    }

    #[tokio::test(start_paused = true)]
    async fn long_representable_cooldown_expires_without_being_shortened() {
        let limiter = RateLimiter::new(Duration::ZERO, Duration::ZERO);
        let delay = Duration::from_secs(365 * 24 * 60 * 60);
        let start = Instant::now();
        limiter.cooldown(false, delay).await;

        assert!(
            tokio::time::timeout(Duration::from_secs(2), limiter.wait(false))
                .await
                .is_err()
        );
        limiter.wait(false).await;

        assert_eq!(start.elapsed(), delay);
    }
}
