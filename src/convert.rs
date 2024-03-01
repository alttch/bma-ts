#[cfg(feature = "chrono")]
use std::str::FromStr;
use std::time::Duration;

use crate::{Error, Timestamp};

#[cfg(feature = "chrono")]
impl FromStr for Timestamp {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(not(feature = "as-float-secs"))]
        if let Ok(v) = s.parse::<u64>() {
            return Ok(v.into());
        }
        #[cfg(feature = "as-float-secs")]
        if let Ok(v) = s.parse::<f64>() {
            return Ok(v.into());
        }
        dateparser::parse(s)
            .map_err(|e| Error::Parse(e.to_string()))?
            .try_into()
    }
}

// Duration
//
impl From<Duration> for Timestamp {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

impl From<Timestamp> for Duration {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

// i64

impl TryFrom<i64> for Timestamp {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self::from_nanos(u64::try_from(value)?))
    }
}

impl TryFrom<Timestamp> for i64 {
    type Error = Error;
    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        value.as_nanos().try_into().map_err(Into::into)
    }
}

// u64

impl From<u64> for Timestamp {
    fn from(value: u64) -> Self {
        Self::from_nanos(value)
    }
}

impl TryFrom<Timestamp> for u64 {
    type Error = Error;
    fn try_from(value: Timestamp) -> Result<Self, Error> {
        value.as_nanos().try_into().map_err(Into::into)
    }
}

// u128

impl From<Timestamp> for u128 {
    fn from(value: Timestamp) -> Self {
        value.as_nanos()
    }
}

impl TryFrom<u128> for Timestamp {
    type Error = Error;
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Ok(Timestamp::from_nanos(value.try_into()?))
    }
}

// f32

impl From<f32> for Timestamp {
    fn from(value: f32) -> Self {
        Self::from_secs_f32(value)
    }
}

impl From<Timestamp> for f32 {
    fn from(value: Timestamp) -> Self {
        value.as_secs_f32()
    }
}

// f64

impl From<f64> for Timestamp {
    fn from(value: f64) -> Self {
        Self::from_secs_f64(value)
    }
}

impl From<Timestamp> for f64 {
    fn from(value: Timestamp) -> Self {
        value.as_secs_f64()
    }
}
