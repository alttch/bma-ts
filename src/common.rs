use crate::{Monotonic, Timestamp};
use core::fmt;
use std::time::Duration;

macro_rules! impl_common {
    ($t: ty) => {
        impl $t {
            #[inline]
            pub fn abs_diff(self, other: $t) -> Duration {
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
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0.as_nanos())
            }
        }
    };
}

impl_common!(Timestamp);
impl_common!(Monotonic);
