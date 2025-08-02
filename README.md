
# simple_duration

`simple_duration` is a crate that provides a "simple and minimal dependency" second-precision Duration type for Rust.  
It is optimized for everyday use with hours, minutes, and seconds, and is suitable for embedded (no_std) environments.

[![Crates.io](https://img.shields.io/crates/v/simple-duration.svg)](https://crates.io/crates/simple-duration)
[![Documentation](https://docs.rs/simple-duration/badge.svg)](https://docs.rs/simple-duration)

## Features

- **Simple time representation in seconds**  
  Specialized for use cases where high precision like milliseconds or nanoseconds is not needed.

- **Intuitive creation and formatting**  
  Easily create from hours, minutes, and seconds, and convert to `"hh:mm:ss"` format strings.

- **String parsing support**  
  Create Duration objects from `"hh:mm:ss"` format strings.

- **Addition and subtraction operations**  
  Add and subtract Duration objects (results never become negative).

- **SystemTime integration**  
  Create Duration from two `SystemTime` values (when `std` feature is enabled).

- **no_std support & minimal dependencies**  
  Safe to use in embedded projects or projects that want to minimize dependencies.

- **Safe error handling**  
  Failures like string parsing return explicit errors via Option/Result without panicking.

- **Well-tested and documented**  
  Includes tests and documentation for quality assurance.

## Usage Example

```rust
use simple_duration::Duration;

// Create from hours, minutes, seconds
let duration = Duration::from_hms(1, 30, 45); // 1 hour 30 minutes 45 seconds

// Create from hours
let duration = Duration::from_hours(2); // 2 hours

// Create from minutes
let duration = Duration::from_minutes(90); // 90 minutes (1 hour 30 minutes)

// Create from seconds
let duration = Duration::from_seconds(3661); // 1 hour 1 minute 1 second

// Create from string
let duration = Duration::parse("01:30:45").unwrap();

// Format
println!("{}", duration.format()); // "01:30:45"

// Get total amounts in each unit
assert_eq!(duration.as_seconds(), 5445);
assert_eq!(duration.as_minutes(), 90); // 90 minutes
assert_eq!(duration.as_hours(), 1); // 1 hour (truncated)

// Get each component (in h:m:s format)
assert_eq!(duration.seconds_part(), 45); // seconds component (0-59)
assert_eq!(duration.minutes_part(), 30);   // minutes component (0-59)
assert_eq!(duration.hours_part(), 1);      // hours component

// Arithmetic operations
let d1 = Duration::from_seconds(100);
let d2 = Duration::from_seconds(50);
let sum = d1 + d2; // 150 seconds
let diff = d1 - d2; // 50 seconds
```

### SystemTime integration (when std feature is enabled)

```rust
use simple_duration::Duration;
use std::time::SystemTime;

let start = SystemTime::now();
// Some processing...
let end = SystemTime::now();

if let Some(duration) = Duration::from_system_time_diff(start, end) {
    println!("Elapsed time: {}", duration.format());
}
```

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
simple-duration = "0.1"
```

### Usage in no_std environments

If you want to use it in a no_std environment, disable the default feature:

```toml
[dependencies]
simple-duration = { version = "0.1", default-features = false }
```

## Intended Use Cases

- Time management in embedded/IoT devices
- Simple timers and countdowns
- Web apps or CLI tools where second precision is sufficient
- When you don't need the precision or features of the standard `Duration` or `chrono`

## API Documentation


### Creation Methods

- [`Duration::from_seconds()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.from_seconds) - Create from seconds
- [`Duration::from_minutes()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.from_minutes) - Create from minutes
- [`Duration::from_hours()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.from_hours) - Create from hours
- [`Duration::from_hms()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.from_hms) - Create from hours, minutes, seconds
- [`Duration::parse()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.parse) - Parse from string

### Get total amount in each unit

- [`Duration::as_seconds()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.as_seconds) - Get total seconds
- [`Duration::as_minutes()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.as_minutes) - Get total minutes (truncated)
- [`Duration::as_hours()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.as_hours) - Get total hours (truncated)

### Get each component (h:m:s format)

- [`Duration::seconds_part()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.seconds_part) - Get seconds component (0-59)
- [`Duration::minutes_part()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.minutes_part) - Get minutes component (0-59)
- [`Duration::hours_part()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.hours_part) - Get hours component (0-∞)

### Others

- [`Duration::format()`](https://docs.rs/simple-duration/latest/simple_duration/struct.Duration.html#method.format) - Format as "hh:mm:ss"

## Development & Testing

```bash
# Run tests
cargo test

# Run tests (no_std)
cargo test --no-default-features

# Generate documentation
cargo doc --open

# Format
cargo fmt

# Lint
cargo clippy
```

## License

MIT OR Apache-2.0

## Contribution

Please report bugs or feature requests via GitHub Issues. Pull requests are also welcome.
