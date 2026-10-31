//! Shared pacing and server-directed cooldowns.
use std::{future::pending, time::Duration};
use tokio::{
    sync::Mutex,
    time::{Instant, sleep_until},
};

// Bound how long a response can block later caller-initiated operations.
// Automatic retries use the original Retry-After independently of this limit.
const MAX_SERVER_COOLDOWN: Duration = Duration::from_secs(300);

pub(crate) struct RateLimiter {
    global_interval: Duration,
    import_interval: Duration,
    state: Mutex<State>,
}
struct State {
    // None represents unrepresentable local pacing, not a server cooldown.
    // The operation deadline bounds every wait.
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
        let Some(until) = Instant::now().checked_add(delay.min(MAX_SERVER_COOLDOWN)) else {
            // Even a bounded addition can fail near Instant's platform limit.
            // Leave existing pacing intact instead of storing a permanent block.
            return;
        };
        let mut state = self.state.lock().await;
        let target = if import {
            &mut state.next_import
        } else {
            &mut state.next_global
        };
        *target = target.map(|old| old.max(until));
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
    async fn oversized_global_cooldown_expires_for_both_buckets() {
        let limiter = RateLimiter::new(Duration::ZERO, Duration::ZERO);
        limiter.cooldown(false, Duration::from_secs(u64::MAX)).await;
        // A later, shorter response must not remove the existing minimum.
        limiter.cooldown(false, Duration::from_secs(1)).await;

        let start = Instant::now();
        for import in [false, true] {
            let wait_start = Instant::now();
            let result = tokio::time::timeout(Duration::from_secs(2), limiter.wait(import)).await;

            assert!(result.is_err());
            assert_eq!(wait_start.elapsed(), Duration::from_secs(2));
        }
        tokio::time::advance(Duration::from_secs(300) - start.elapsed()).await;

        for import in [false, true] {
            assert!(
                tokio::time::timeout(Duration::from_secs(1), limiter.wait(import))
                    .await
                    .is_ok()
            );
        }
        assert_eq!(start.elapsed(), Duration::from_secs(300));
    }

    #[tokio::test(start_paused = true)]
    async fn oversized_import_cooldown_expires_without_blocking_other_requests() {
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
        tokio::time::advance(Duration::from_secs(298)).await;
        assert!(
            tokio::time::timeout(Duration::from_secs(1), limiter.wait(true))
                .await
                .is_ok()
        );
        assert_eq!(start.elapsed(), Duration::from_secs(300));
    }

    #[tokio::test(start_paused = true)]
    async fn long_representable_server_cooldown_is_bounded() {
        let limiter = RateLimiter::new(Duration::ZERO, Duration::ZERO);
        let delay = Duration::from_secs(365 * 24 * 60 * 60);
        let start = Instant::now();
        limiter.cooldown(false, delay).await;

        assert!(
            tokio::time::timeout(Duration::from_secs(2), limiter.wait(false))
                .await
                .is_err()
        );
        tokio::time::advance(Duration::from_secs(298)).await;
        assert!(
            tokio::time::timeout(Duration::from_secs(1), limiter.wait(false))
                .await
                .is_ok()
        );

        assert_eq!(start.elapsed(), Duration::from_secs(300));
    }

    #[tokio::test(start_paused = true)]
    async fn cooldown_at_the_cap_is_honored_in_full() {
        let limiter = RateLimiter::new(Duration::ZERO, Duration::ZERO);
        let start = Instant::now();
        limiter.cooldown(false, Duration::from_secs(300)).await;
        limiter.wait(false).await;

        assert_eq!(start.elapsed(), Duration::from_secs(300));
    }

    #[tokio::test(start_paused = true)]
    async fn server_cooldown_cap_does_not_shorten_local_pacing() {
        let limiter = RateLimiter::new(Duration::from_secs(600), Duration::ZERO);
        let start = Instant::now();
        limiter.wait(false).await;
        limiter.cooldown(false, Duration::from_secs(u64::MAX)).await;
        limiter.wait(false).await;

        assert_eq!(start.elapsed(), Duration::from_secs(600));
    }
}
