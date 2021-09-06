# r8limit
[![Crates.io](https://img.shields.io/crates/v/r8limit)](https://crates.io/crates/r8limit)
[![docs.rs](https://img.shields.io/badge/docs.rs-rustdoc-green)](https://docs.rs/r8limit)

A dead simple Rust library for rate limiting.

## Usage
In your `Cargo.toml`:
```toml
[dependencies]
r8limit = "0.1"
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
