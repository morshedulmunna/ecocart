//! Rate limiting primitives
//!
//! This module provides a simple, production-ready sliding-window rate limiter
//! with an in-memory backend. It is designed to be dependency-free and safe to
//! use in async contexts because it uses short, non-blocking critical sections
//! guarded by `std::sync::Mutex`.
//!
//! Typical usage:
//! ```rust
//! use ecocart::pkg::rate_limit::{InMemorySlidingWindowRateLimiter, RateLimitConfig};
//! use std::time::Duration;
//!
//! let limiter = InMemorySlidingWindowRateLimiter::new(RateLimitConfig::new(10, Duration::from_secs(1)));
//! let decision = limiter.check_and_consume("user:123");
//! if decision.allowed {
//!     // proceed
//! } else {
//!     // throttle; check decision.retry_after
//! }
//! ```

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Immutable configuration for a sliding-window rate limiter
#[derive(Debug, Clone, Copy)]
pub struct RateLimitConfig {
    /// Maximum number of allowed events in the window
    pub capacity: u32,
    /// The window size during which events are counted
    pub window: Duration,
}

impl RateLimitConfig {
    /// Create a new configuration
    pub fn new(capacity: u32, window: Duration) -> Self {
        Self { capacity, window }
    }
}

/// Result of a rate limit check
#[derive(Debug, Clone)]
pub struct RateLimitDecision {
    /// Whether the action is allowed
    pub allowed: bool,
    /// Configured hard limit
    pub limit: u32,
    /// Remaining tokens in the current window if allowed, otherwise 0
    pub remaining: u32,
    /// Duration after which the next request may succeed when limited
    pub retry_after: Option<Duration>,
    /// Time until the rolling window resets relative to the oldest counted event
    pub reset_after: Duration,
}

impl RateLimitDecision {
    fn allowed(limit: u32, remaining: u32, reset_after: Duration) -> Self {
        Self {
            allowed: true,
            limit,
            remaining,
            retry_after: None,
            reset_after,
        }
    }

    fn limited(limit: u32, retry_after: Duration, reset_after: Duration) -> Self {
        Self {
            allowed: false,
            limit,
            remaining: 0,
            retry_after: Some(retry_after),
            reset_after,
        }
    }
}

/// A simple, thread-safe in-memory sliding window rate limiter.
///
/// This implementation keeps, for each key, a deque of `Instant`s representing
/// the timestamps of accepted events. On each check, timestamps older than the
/// window are pruned. If the deque length is below capacity, the event is
/// accepted and the current timestamp is recorded; otherwise the event is
/// rejected and the earliest time when it would be accepted is derived from the
/// oldest timestamp plus the window.
pub struct InMemorySlidingWindowRateLimiter {
    config: RateLimitConfig,
    // Key -> deque of accepted event instants within the current window
    state: Mutex<HashMap<String, VecDeque<Instant>>>,
}

impl InMemorySlidingWindowRateLimiter {
    /// Create a new in-memory limiter with the given configuration
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            state: Mutex::new(HashMap::new()),
        }
    }

    /// Check if an event for `key` is allowed and consume a slot if yes.
    ///
    /// This method is O(k) in the number of events currently in the window for
    /// the given key, dominated by pruning of old timestamps. In typical usage
    /// k <= capacity.
    pub fn check_and_consume(&self, key: &str) -> RateLimitDecision {
        let now = Instant::now();
        let limit = self.config.capacity;
        let window = self.config.window;

        let mut guard = self.state.lock().expect("mutex poisoned");
        let deque = guard.entry(key.to_string()).or_insert_with(VecDeque::new);

        // Prune timestamps outside the window
        let cutoff = now.checked_sub(window).unwrap_or(now);
        while let Some(&front) = deque.front() {
            if front < cutoff {
                deque.pop_front();
            } else {
                break;
            }
        }

        if (deque.len() as u32) < limit {
            // Accept and record this event
            deque.push_back(now);
            let remaining = limit - (deque.len() as u32);
            let reset_after = deque
                .front()
                .map(|&t| t + window - now)
                .unwrap_or(Duration::from_secs(0));
            RateLimitDecision::allowed(limit, remaining, reset_after)
        } else {
            // Reject; compute retry_after based on oldest event expiry
            if let Some(&oldest) = deque.front() {
                let retry_after = oldest + window - now;
                let reset_after = retry_after;
                RateLimitDecision::limited(limit, retry_after, reset_after)
            } else {
                // Should not happen: capacity == 0 or logic error; treat as immediate retry
                RateLimitDecision::limited(limit, Duration::from_secs(0), Duration::from_secs(0))
            }
        }
    }

    /// Reset counters for a specific key.
    pub fn reset(&self, key: &str) {
        let mut guard = self.state.lock().expect("mutex poisoned");
        if let Some(entry) = guard.get_mut(key) {
            entry.clear();
        }
    }

    /// Compose a stable key from parts, useful for namespacing (e.g., ip + route).
    pub fn compose_key(parts: &[&str]) -> String {
        parts.join(":")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn allows_up_to_capacity_within_window() {
        let limiter = InMemorySlidingWindowRateLimiter::new(RateLimitConfig::new(
            3,
            Duration::from_millis(100),
        ));
        let key = "k";

        for i in 0..3 {
            let d = limiter.check_and_consume(key);
            assert!(d.allowed, "iteration {} should be allowed", i);
        }

        let d = limiter.check_and_consume(key);
        assert!(!d.allowed, "exceeding capacity should be denied");
        assert!(d.retry_after.unwrap() <= Duration::from_millis(100));
    }

    #[test]
    fn frees_slot_after_window_expires() {
        let limiter = InMemorySlidingWindowRateLimiter::new(RateLimitConfig::new(
            1,
            Duration::from_millis(50),
        ));
        let key = "user:1";

        let d1 = limiter.check_and_consume(key);
        assert!(d1.allowed);

        let d2 = limiter.check_and_consume(key);
        assert!(!d2.allowed);

        // Sleep until the window expires
        thread::sleep(Duration::from_millis(60));

        let d3 = limiter.check_and_consume(key);
        assert!(d3.allowed);
    }

    #[test]
    fn reset_clears_state() {
        let limiter =
            InMemorySlidingWindowRateLimiter::new(RateLimitConfig::new(1, Duration::from_secs(1)));
        let key = "route:/health";
        assert!(limiter.check_and_consume(key).allowed);
        assert!(!limiter.check_and_consume(key).allowed);
        limiter.reset(key);
        assert!(limiter.check_and_consume(key).allowed);
    }
}
