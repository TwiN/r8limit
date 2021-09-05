# ratelimiter
A dead simple Rust library for rate limiting.

## Usage
```rust
extern crate ratelimiter;

use std::time::Duration;

fn main() {
    // Allow 3 attempts every 5 seconds
    let mut limiter = ratelimiter::RateLimiter::new(3, Duration::from_secs(5));
    println!("{}", limiter.attempt()); // true
    println!("{}", limiter.attempt()); // true
    println!("{}", limiter.attempt()); // true
    println!("{}", limiter.attempt()); // false
}
```