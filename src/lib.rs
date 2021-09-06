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

pub struct RateLimiter {
    max_executions_per_window: u64,
    executions_left_in_window: u64,
    window_start_time: Instant,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_executions_per_internal: u64, interval: Duration) -> RateLimiter {
        RateLimiter {
            max_executions_per_window: max_executions_per_internal,
            executions_left_in_window: max_executions_per_internal,
            window_start_time: Instant::now(),
            window_duration: interval,
        }
    }

    pub fn attempt(&mut self) -> bool {
        if self.window_start_time.elapsed() > self.window_duration {
            self.window_start_time = Instant::now();
            self.executions_left_in_window = self.max_executions_per_window;
        }
        if self.executions_left_in_window == 0 {
            return false;
        }
        self.executions_left_in_window -= 1;
        return true;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread::sleep;
    use crate::RateLimiter;

    #[test]
    fn it_works() {
        let mut limiter = RateLimiter::new(3, Duration::from_secs(1));
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), false);
        assert_eq!(limiter.attempt(), false);
        assert_eq!(limiter.attempt(), false);
        sleep(Duration::from_secs(1));
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), true);
        assert_eq!(limiter.attempt(), false);
        assert_eq!(limiter.attempt(), false);
    }
}
