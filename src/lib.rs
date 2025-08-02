#![cfg_attr(not(feature = "std"), no_std)]

//! # simple_duration
//!
//! `simple_duration` is a crate that provides a "simple and minimal dependency" second-precision Duration type for Rust.
//! It's optimized for everyday "hours, minutes, seconds" handling and embedded environments (no_std).
//!
//! ## Features
//!
//! - **Simple time representation in seconds**: Specialized for use cases that don't require high precision like milliseconds or nanoseconds
//! - **Intuitive creation and formatting**: Easy creation from hours/minutes/seconds and conversion to `"hh:mm:ss"` format strings
//! - **String parsing support**: Can create Duration objects from `"hh:mm:ss"` format strings
//! - **Addition and subtraction operations**: Duration objects can be added and subtracted (results never become negative)
//! - **SystemTime integration**: Can create Duration from two `SystemTime` instances (when `std` feature is enabled)
//! - **no_std support & minimal dependencies**: Safe to use in embedded projects or projects that want to minimize dependencies
//! - **Safe error handling**: Failures like string parsing return explicit errors via Option/Result without panicking
//!
//! ## Usage Examples
//!
//! ```rust
//! use simple_duration::Duration;
//!
//! // Create from hours, minutes, seconds
//! let duration = Duration::from_hms(1, 30, 45); // 1 hour 30 minutes 45 seconds
//!
//! // Create from hours
//! let duration = Duration::from_hours(2); // 2 hours
//!
//! // Create from minutes
//! let duration = Duration::from_minutes(90); // 90 minutes (1 hour 30 minutes)
//!
//! // Create from seconds
//! let duration = Duration::from_seconds(3661); // 1 hour 1 minute 1 second
//!
//! // Create from string
//! let duration = Duration::parse("01:30:45").unwrap();
//!
//! // Format
//! assert_eq!(duration.format(), "01:30:45");
//!
//! // Get total amounts in each unit
//! assert_eq!(duration.as_seconds(), 5445);
//! assert_eq!(duration.as_minutes(), 90); // 90 minutes
//! assert_eq!(duration.as_hours(), 1); // 1 hour (truncated)
//!
//! // Get each component (in h:m:s format)
//! assert_eq!(duration.seconds_part(), 45); // seconds component (0-59)
//! assert_eq!(duration.minutes_part(), 30);   // minutes component (0-59)
//! assert_eq!(duration.hours_part(), 1);      // hours component
//!
//! // Arithmetic operations
//! let d1 = Duration::from_seconds(100);
//! let d2 = Duration::from_seconds(50);
//! let sum = d1 + d2; // 150 seconds
//! let diff = d1 - d2; // 50 seconds
//! ```

#[cfg(feature = "std")]
use std::time::SystemTime;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, format};

use core::ops::{Add, Sub};

/// Simple Duration type with second precision
///
/// This struct provides time representation in seconds, optimized for hours/minutes/seconds handling.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    seconds: u64,
}

/// Possible errors that can occur during Duration operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DurationError {
    /// Invalid string format
    InvalidFormat,
    /// Invalid value for hours, minutes, or seconds
    InvalidValue,
}

impl Duration {
    /// Create a new Duration from seconds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::from_seconds(3661);
    /// assert_eq!(duration.hours_part(), 1);
    /// assert_eq!(duration.minutes_part(), 1);
    /// assert_eq!(duration.seconds_part(), 1);
    /// ```
    pub fn from_seconds(seconds: u64) -> Self {
        Self { seconds }
    }

    /// Create Duration from minutes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::from_minutes(90);
    /// assert_eq!(duration.as_seconds(), 5400);
    /// assert_eq!(duration.hours_part(), 1);
    /// assert_eq!(duration.minutes_part(), 30);
    /// assert_eq!(duration.seconds_part(), 0);
    /// ```
    pub fn from_minutes(minutes: u64) -> Self {
        Self {
            seconds: minutes * 60,
        }
    }

    /// Create Duration from hours
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::from_hours(2);
    /// assert_eq!(duration.as_seconds(), 7200);
    /// assert_eq!(duration.hours_part(), 2);
    /// assert_eq!(duration.minutes_part(), 0);
    /// assert_eq!(duration.seconds_part(), 0);
    /// ```
    pub fn from_hours(hours: u64) -> Self {
        Self {
            seconds: hours * 3600,
        }
    }

    /// Create Duration from hours, minutes, and seconds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::from_hms(1, 30, 45);
    /// assert_eq!(duration.as_seconds(), 5445);
    /// ```
    pub fn from_hms(hours: u64, minutes: u64, seconds: u64) -> Self {
        Self {
            seconds: hours * 3600 + minutes * 60 + seconds,
        }
    }

    /// Parse Duration from "hh:mm:ss" format string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::parse("01:30:45").unwrap();
    /// assert_eq!(duration.hours_part(), 1);
    /// assert_eq!(duration.minutes_part(), 30);
    /// assert_eq!(duration.seconds_part(), 45);
    ///
    /// assert!(Duration::parse("invalid").is_err());
    /// ```
    pub fn parse(s: &str) -> Result<Self, DurationError> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(DurationError::InvalidFormat);
        }

        let hours = parts[0].parse::<u64>().map_err(|_| DurationError::InvalidValue)?;
        let minutes = parts[1].parse::<u64>().map_err(|_| DurationError::InvalidValue)?;
        let seconds = parts[2].parse::<u64>().map_err(|_| DurationError::InvalidValue)?;

        if minutes >= 60 || seconds >= 60 {
            return Err(DurationError::InvalidValue);
        }

        Ok(Self::from_hms(hours, minutes, seconds))
    }

    /// Get total seconds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::from_seconds(3661);
    /// assert_eq!(duration.as_seconds(), 3661);
    /// ```
    pub fn as_seconds(&self) -> u64 {
        self.seconds
    }

    /// Get total minutes (truncated)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::from_seconds(150); // 2 minutes 30 seconds
    /// assert_eq!(duration.as_minutes(), 2);
    ///
    /// let duration = Duration::from_seconds(3661); // 1 hour 1 minute 1 second
    /// assert_eq!(duration.as_minutes(), 61);
    /// ```
    pub fn as_minutes(&self) -> u64 {
        self.seconds / 60
    }

    /// Get total hours (truncated)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::from_seconds(3661); // 1 hour 1 minute 1 second
    /// assert_eq!(duration.as_hours(), 1);
    ///
    /// let duration = Duration::from_seconds(7200); // 2 hours
    /// assert_eq!(duration.as_hours(), 2);
    /// ```
    pub fn as_hours(&self) -> u64 {
        self.seconds / 3600
    }

    /// Get seconds component (0-59)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::from_seconds(3661); // 1 hour 1 minute 1 second
    /// assert_eq!(duration.seconds_part(), 1);
    ///
    /// let duration = Duration::from_seconds(150); // 2 minutes 30 seconds
    /// assert_eq!(duration.seconds_part(), 30);
    /// ```
    pub fn seconds_part(&self) -> u64 {
        self.seconds % 60
    }

    /// Get minutes component (0-59)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::from_seconds(3661); // 1 hour 1 minute 1 second
    /// assert_eq!(duration.minutes_part(), 1);
    ///
    /// let duration = Duration::from_seconds(150); // 2 minutes 30 seconds
    /// assert_eq!(duration.minutes_part(), 2);
    /// ```
    pub fn minutes_part(&self) -> u64 {
        (self.seconds % 3600) / 60
    }

    /// Get hours component (0-∞)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::from_seconds(3661); // 1 hour 1 minute 1 second
    /// assert_eq!(duration.hours_part(), 1);
    ///
    /// let duration = Duration::from_seconds(7200); // 2 hours
    /// assert_eq!(duration.hours_part(), 2);
    /// ```
    pub fn hours_part(&self) -> u64 {
        self.seconds / 3600
    }

    /// Format as "hh:mm:ss" string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simple_duration::Duration;
    ///
    /// let duration = Duration::from_hms(1, 5, 30);
    /// assert_eq!(duration.format(), "01:05:30");
    /// ```
    pub fn format(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hours_part(), self.minutes_part(), self.seconds_part())
    }

    /// Create a zero Duration
    pub fn zero() -> Self {
        Self { seconds: 0 }
    }

    /// Check if this Duration is zero
    pub fn is_zero(&self) -> bool {
        self.seconds == 0
    }

    /// Saturating addition (prevents overflow)
    pub fn saturating_add(self, other: Self) -> Self {
        Self {
            seconds: self.seconds.saturating_add(other.seconds),
        }
    }

    /// Saturating subtraction (prevents underflow)
    pub fn saturating_sub(self, other: Self) -> Self {
        Self {
            seconds: self.seconds.saturating_sub(other.seconds),
        }
    }
}

/// SystemTime conversion (only when std feature is enabled)
#[cfg(feature = "std")]
impl Duration {
    /// Create Duration from the time difference between two SystemTimes
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use simple_duration::Duration;
    /// use std::time::SystemTime;
    ///
    /// let start = SystemTime::now();
    /// // Some processing...
    /// let end = SystemTime::now();
    ///
    /// if let Some(duration) = Duration::from_system_time_diff(start, end) {
    ///     println!("Elapsed time: {}", duration.format());
    /// }
    /// ```
    pub fn from_system_time_diff(start: SystemTime, end: SystemTime) -> Option<Self> {
        end.duration_since(start)
            .ok()
            .map(|std_duration| Self::from_seconds(std_duration.as_secs()))
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.saturating_add(other)
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.saturating_sub(other)
    }
}

impl core::fmt::Display for Duration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        // Creation from seconds
        let d1 = Duration::from_seconds(3661); // 1 hour 1 minute 1 second
        assert_eq!(d1.as_seconds(), 3661);
        assert_eq!(d1.seconds_part(), 1);
        assert_eq!(d1.minutes_part(), 1);
        assert_eq!(d1.hours_part(), 1);

        // Creation from minutes
        let d2 = Duration::from_minutes(150); // 2 hours 30 minutes
        assert_eq!(d2.as_seconds(), 9000);
        assert_eq!(d2.format(), "02:30:00");

        // Creation from hours
        let d3 = Duration::from_hours(3);
        assert_eq!(d3.as_seconds(), 10800);
        assert_eq!(d3.format(), "03:00:00");

        // Creation from hours, minutes, seconds
        let d4 = Duration::from_hms(2, 30, 45);
        assert_eq!(d4.as_seconds(), 9045);
        assert_eq!(d4.format(), "02:30:45");
    }

    #[test]
    fn test_unit_conversions() {
        let duration = Duration::from_seconds(3661); // 1 hour 1 minute 1 second
        
        // Get total amount in each unit
        assert_eq!(duration.as_seconds(), 3661);
        assert_eq!(duration.as_minutes(), 61); // 61 minutes
        assert_eq!(duration.as_hours(), 1); // 1 hour (truncated)
        
        // Get each component
        assert_eq!(duration.seconds_part(), 1);
        assert_eq!(duration.minutes_part(), 1);
        assert_eq!(duration.hours_part(), 1);

        // More complex example
        let duration2 = Duration::from_seconds(7890); // 2 hours 11 minutes 30 seconds
        assert_eq!(duration2.as_minutes(), 131); // 131 minutes
        assert_eq!(duration2.as_hours(), 2); // 2 hours
        assert_eq!(duration2.seconds_part(), 30);
        assert_eq!(duration2.minutes_part(), 11);
        assert_eq!(duration2.hours_part(), 2);
    }

    #[test]
    fn test_string_parsing() {
        // Normal parsing - boundary value test
        let duration = Duration::parse("23:59:59").unwrap(); // Maximum valid h:m:s
        assert_eq!(duration.seconds_part(), 59);
        assert_eq!(duration.minutes_part(), 59);
        assert_eq!(duration.hours_part(), 23);

        // Minimum value test
        let duration_min = Duration::parse("00:00:00").unwrap();
        assert_eq!(duration_min.seconds_part(), 0);
        assert_eq!(duration_min.minutes_part(), 0);
        assert_eq!(duration_min.hours_part(), 0);

        // Abnormal parsing - cases exceeding boundary values
        assert!(Duration::parse("invalid").is_err());
        assert!(Duration::parse("1:2").is_err()); // Invalid format
        assert!(Duration::parse("1:60:30").is_err()); // Minutes is 60 (exceeds boundary)
        assert!(Duration::parse("1:30:60").is_err()); // Seconds is 60 (exceeds boundary)
        assert!(Duration::parse("24:59:59").is_ok()); // 24 hours is valid (crosses day)
        assert!(Duration::parse("00:59:59").is_ok()); // Within boundary
        assert!(Duration::parse("00:00:59").is_ok()); // Within boundary
    }

    #[test]
    fn test_formatting() {
        let cases = [
            (Duration::from_hms(1, 5, 30), "01:05:30"),
            (Duration::from_hms(12, 0, 0), "12:00:00"),
            (Duration::zero(), "00:00:00"),
        ];

        for (duration, expected) in cases {
            assert_eq!(duration.format(), expected);
            assert_eq!(format!("{}", duration), expected);
        }
    }

    #[test]
    fn test_arithmetic() {
        let d1 = Duration::from_seconds(100);
        let d2 = Duration::from_seconds(50);
        
        // Normal addition
        assert_eq!((d1 + d2).as_seconds(), 150);
        
        // Normal subtraction
        assert_eq!((d1 - d2).as_seconds(), 50);
        
        // Underflow (saturating subtraction)
        assert_eq!((d2 - d1).as_seconds(), 0);
        
        // Overflow (saturating addition) test
        let max_duration = Duration::from_seconds(u64::MAX);
        let small_duration = Duration::from_seconds(1);
        
        // Adding 1 to u64::MAX should remain u64::MAX (saturated)
        assert_eq!((max_duration + small_duration).as_seconds(), u64::MAX);
        
        // Addition of large values should also saturate
        let large1 = Duration::from_seconds(u64::MAX - 10);
        let large2 = Duration::from_seconds(20);
        assert_eq!((large1 + large2).as_seconds(), u64::MAX);
        
        // Boundary test just before overflow (no overflow case)
        let near_max = Duration::from_seconds(u64::MAX - 50);
        let small = Duration::from_seconds(30);
        assert_eq!((near_max + small).as_seconds(), u64::MAX - 20);
    }

    #[test]
    fn test_utility_methods() {
        let zero = Duration::zero();
        assert!(zero.is_zero());
        assert_eq!(zero.as_seconds(), 0);

        let non_zero = Duration::from_seconds(1);
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn test_comparison() {
        let d1 = Duration::from_seconds(100);
        let d2 = Duration::from_seconds(200);
        
        assert!(d1 < d2);
        assert!(d2 > d1);
        assert_eq!(d1, Duration::from_seconds(100));
        assert_ne!(d1, d2);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_system_time_integration() {
        use std::time::SystemTime;
        
        let start = SystemTime::now();
        let end = start + std::time::Duration::from_secs(100);
        
        let duration = Duration::from_system_time_diff(start, end).unwrap();
        assert_eq!(duration.as_seconds(), 100);
    }
}
