use core::fmt;
use std::num::TryFromIntError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod convert;
#[cfg(feature = "chrono")]
mod impl_chrono;
#[cfg(feature = "serde")]
mod impl_serde;
#[cfg(feature = "sqlx")]
mod impl_sqlx;
mod monotonic;
mod operations;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("timestamp parse failed {0}")]
    Parse(String),
    #[error("timestamp conversion failed")]
    ConvertChrono,
    #[error("timestamp number conversion failed: {0}")]
    Convert(#[from] TryFromIntError),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timestamp(Duration);

impl Timestamp {
    /// # Panics
    ///
    /// Will panic if the system time is below 1.01.1970
    pub fn now() -> Self {
        Self(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("System time is below UNIX EPOCH"),
        )
    }
    pub fn now_rounded() -> Self {
        let t = Self::now();
        Self(Duration::from_secs(t.0.as_secs()))
    }
    #[inline]
    pub fn abs_diff(self, other: Timestamp) -> Duration {
        if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        }
    }
    #[inline]
    pub fn as_secs(self) -> u64 {
        self.0.as_secs()
    }
    #[inline]
    pub fn from_secs(value: u64) -> Self {
        Self(Duration::from_secs(value))
    }
    #[inline]
    pub fn as_secs_f64(self) -> f64 {
        self.0.as_secs_f64()
    }
    #[inline]
    pub fn from_secs_f64(value: f64) -> Self {
        Self(Duration::from_secs_f64(value))
    }
    #[inline]
    pub fn as_secs_f32(self) -> f32 {
        self.0.as_secs_f32()
    }
    #[inline]
    pub fn from_secs_f32(value: f32) -> Self {
        Self(Duration::from_secs_f32(value))
    }
    #[inline]
    pub fn as_micros(self) -> u128 {
        self.0.as_micros()
    }
    #[inline]
    pub fn from_micros(value: u64) -> Self {
        Self(Duration::from_micros(value))
    }
    #[inline]
    pub fn as_millis(self) -> u128 {
        self.0.as_millis()
    }
    #[inline]
    pub fn from_millis(value: u64) -> Self {
        Self(Duration::from_millis(value))
    }
    #[inline]
    pub fn as_nanos(self) -> u128 {
        self.0.as_nanos()
    }
    #[inline]
    pub fn from_nanos(value: u64) -> Self {
        Self(Duration::from_nanos(value))
    }
    #[inline]
    pub fn as_duration(self) -> Duration {
        self.into()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.as_nanos())
    }
}

#[cfg(test)]
mod tests {
    use super::Timestamp;
    #[test]
    fn test_time() {
        let timestamp = 1632093707.1893349;
        let time = Timestamp::from_secs_f64(timestamp);
        assert_eq!(time.as_secs_f64(), timestamp);
        assert_eq!(time.as_nanos(), 1632093707189334869);
        let timestamp_ns = 1632093707123456789;
        let time = Timestamp::from_nanos(timestamp_ns);
        assert_eq!(time.as_nanos() as u64, timestamp_ns);
        assert_eq!(time.as_secs_f64(), 1_632_093_707.123_456_7);
        assert_eq!(time.as_millis() as u64, timestamp_ns / 1_000_000);
        assert_eq!(time.as_micros() as u64, timestamp_ns / 1_000);
        let timestamp_us = 1632093707123456;
        let time = Timestamp::from_micros(timestamp_us);
        assert_eq!(time.as_secs_f64(), 1632093707.123456);
        assert_eq!(time.as_millis() as u64, timestamp_us / 1_000);
        assert_eq!(time.as_micros() as u64, timestamp_us);
        assert_eq!(time.as_nanos() as u64, timestamp_us * 1_000);
        let timestamp_ms = 1632093707123u128;
        let time = Timestamp::from_millis(timestamp_ms as u64);
        assert_eq!(time.as_secs_f64(), 1632093707.123);
        assert_eq!(time.as_millis(), timestamp_ms);
        assert_eq!(time.as_micros(), timestamp_ms * 1_000);
        assert_eq!(time.as_nanos(), timestamp_ms * 1_000_000);
    }
}
