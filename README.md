# r8limit
[![Crates.io](https://img.shields.io/crates/v/r8limit)](https://crates.io/crates/r8limit)
![Downloads](https://img.shields.io/crates/d/r8limit)
[![docs.rs](https://img.shields.io/badge/docs.rs-rustdoc-green)](https://docs.rs/r8limit)

A dead simple Rust library for rate limiting.

## Usage
In your `Cargo.toml`:
```toml
[dependencies]
r8limit = "0.2"
```

In your code:
```rust
use std::time::Duration;

fn main() {
    // Allow 3 attempts every 5 seconds
    let mut limiter = r8limit::RateLimiter::new(3, Duration::from_secs(5));
    println!("{}", limiter.attempt()); // true
    println!("{}", limiter.attempt()); // true
    println!("{}", limiter.attempt()); // true
    println!("{}", limiter.attempt()); // false
}
```

## Refill policies
The refill policy is what determines how the number of executions in an interval are refilled.

### Full
By default, the refill policy is set to `Full`. This means that every time the difference between now and the
start of the current window has reached or exceeded the specified interval, the number of executions is reset to
the maximum number of executions allowed during the interval.

```rust
use std::time::Duration;
use std::thread::sleep;
use crate::RateLimiter;

fn main() {
    let mut limiter = RateLimiter::new(3, Duration::from_secs(1));
    limiter.attempt(); // returns true; executions remaining in window: 2
    limiter.attempt(); // returns true; executions remaining in window: 1
    limiter.attempt(); // returns true; executions remaining in window: 0
    limiter.attempt(); // returns false; executions remaining in window: 0
    // Remember that the interval is set to 1s
    sleep(Duration::from_millis(500)); // executions remaining in window: 0
    // As you can see, even though half of the interval has passed, there are still 0 executions available.
    assert_eq!(limiter.attempt(), false); // returns false; executions remaining for window: 0
    // That is what the default refill policy, RefillPolicy::Full, does.
    // We'll sleep for the remainder of the window, which means that the next attempt will reset the window
    sleep(Duration::from_millis(500)); // executions remaining in window: 3
    limiter.attempt(); // returns true; executions remaining in window: 2
    limiter.attempt(); // returns true; executions remaining in window: 1
    limiter.attempt(); // returns true; executions remaining in window: 0
    limiter.attempt(); // returns false; executions remaining in window: 0
}
```

### Gradual
Unlike the `Full` refill policy, the `Gradual` refill policy does not wait until the interval has completely elapsed
to refill the number of executions remaining. 

```rust
use std::time::Duration;
use std::thread::sleep;
use crate::{RateLimiter, RefillPolicy};

fn main() {
    let mut limiter = RateLimiter::new(2, Duration::from_millis(500)).with_refill_policy(RefillPolicy::Gradual);
    limiter.attempt(); // returns true; executions remaining in window: 1
    limiter.attempt(); // returns true; executions remaining in window: 0
    limiter.attempt(); // returns false; executions remaining in window: 0
    // The Gradual refill policy calculates the percentage of the interval which has 
    // elapsed, and multiplies that by the number of executions allowed per interval.
    // This means that if we wait for half of the interval, half of the executions will become available again
    sleep(Duration::from_millis(250)); // executions remaining in window: 1
    limiter.attempt(); // returns true; executions remaining in window: 0
    limiter.attempt(); // returns false; executions remaining in window: 0
    sleep(Duration::from_millis(500)); // executions remaining in window: 2
    limiter.attempt(); // returns true; executions remaining in window: 1
    limiter.attempt(); // returns true; executions remaining in window: 0
    limiter.attempt(); // returns false; executions remaining in window: 0
}
```
