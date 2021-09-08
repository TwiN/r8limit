//! An extremely simple window rate limiter.
//!
//! # Usage
//! In your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! r8limit = "0.1"
//! ```
//!
//! In your code:
//! ```
//! use std::time::Duration;
//!
//! // Allow 3 attempts every 5 seconds
//! let mut limiter = r8limit::RateLimiter::new(3, Duration::from_secs(5));
//!
//! println!("{}", limiter.attempt()); // true
//! println!("{}", limiter.attempt()); // true
//! println!("{}", limiter.attempt()); // true
//! println!("{}", limiter.attempt()); // false
//! ```

use std::time::{Duration, Instant};
use crate::RefillPolicy::Full;

pub struct RateLimiter {
    max_executions_per_window: f64,
    executions_left_in_window: f64,
    window_start_time: Instant,
    window_duration: Duration,
    refill_policy: RefillPolicy
}

impl RateLimiter {
    pub fn new(max_executions_per_internal: u64, interval: Duration) -> RateLimiter {
        RateLimiter {
            max_executions_per_window: max_executions_per_internal as f64,
            executions_left_in_window: max_executions_per_internal as f64 ,
            window_start_time: Instant::now(),
            window_duration: interval,
            refill_policy: Full
        }
    }

    pub fn with_refill_policy(mut self, refill_policy: RefillPolicy) -> RateLimiter {
        self.refill_policy = refill_policy;
        self
    }

    pub fn attempt(&mut self) -> bool {
        match &self.refill_policy {
            RefillPolicy::Full => {
                if self.window_start_time.elapsed() > self.window_duration {
                    self.window_start_time = Instant::now();
                    self.executions_left_in_window = self.max_executions_per_window;
                }
            },
            RefillPolicy::Gradual => {
                if self.executions_left_in_window != self.max_executions_per_window {
                    if self.window_start_time.elapsed() > self.window_duration {
                        // Been a while since the rate limiter was triggered, so we'll reset everything
                        self.window_start_time = Instant::now();
                        self.executions_left_in_window = self.max_executions_per_window;
                    } else {
                        // Need to move window and update the executions_left_in_window
                        let percentage_done = self.window_start_time.elapsed().as_secs_f64()/self.window_duration.as_secs_f64();
                        let executions_due = percentage_done * self.max_executions_per_window;
                        self.window_start_time = Instant::now();
                        self.executions_left_in_window += executions_due;
                    }
                }
            }
        }
        if self.executions_left_in_window < 1.0 {
            return false;
        }
        self.executions_left_in_window -= 1.0;
        return true;
    }
}

pub enum RefillPolicy {
    Full,
    Gradual
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread::sleep;
    use crate::{RateLimiter, RefillPolicy};

    #[test]
    fn rate_limiter_full() {
        let mut limiter = RateLimiter::new(3, Duration::from_secs(1));
        assert_eq!(limiter.attempt(), true); // executions remaining for window: 2
        assert_eq!(limiter.attempt(), true); // executions remaining for window: 1
        assert_eq!(limiter.attempt(), true); // executions remaining for window: 0
        assert_eq!(limiter.attempt(), false); // executions remaining for window: 0
        // Remember that the interval is set to 1s
        sleep(Duration::from_millis(500)); // executions remaining for window: 0
        // As you can see, even though half of the interval has passed, there are still 0 executions available.
        assert_eq!(limiter.attempt(), false);
        // That is what the default refill policy, RefillPolicy::Full, does.
        // We'll sleep for the remainder of the window
        sleep(Duration::from_millis(500)); // executions remaining for window: 3
        assert_eq!(limiter.attempt(), true); // executions remaining for window: 2
        assert_eq!(limiter.attempt(), true); // executions remaining for window: 1
        assert_eq!(limiter.attempt(), true); // executions remaining for window: 0
        assert_eq!(limiter.attempt(), false); // executions remaining for window: 0
        assert_eq!(limiter.attempt(), false); // executions remaining for window: 0
    }

    #[test]
    fn rate_limiter_gradual() {
        let mut limiter = RateLimiter::new(4, Duration::from_millis(500)).with_refill_policy(RefillPolicy::Gradual);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), false);
        sleep(Duration::from_millis(250));
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), false);
        sleep(Duration::from_millis(500));
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), false);
        sleep(Duration::from_millis(125));
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), false);
        sleep(Duration::from_millis(125));
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), false);
        sleep(Duration::from_millis(750));
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), false);
    }
}
