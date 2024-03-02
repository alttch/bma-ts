use std::ops;
use std::time::Duration;

use crate::{Monotonic, Timestamp};

macro_rules! impl_common_operations {
    ($t: ty) => {
        impl ops::Add<Duration> for $t {
            type Output = $t;

            fn add(self, rhs: Duration) -> Self::Output {
                Self(self.0 + rhs)
            }
        }

        impl ops::AddAssign<Duration> for $t {
            fn add_assign(&mut self, rhs: Duration) {
                self.0 = self.0 + rhs;
            }
        }

        impl ops::Sub<Duration> for $t {
            type Output = $t;

            fn sub(self, rhs: Duration) -> Self::Output {
                Self(self.0 - rhs)
            }
        }

        impl ops::SubAssign<Duration> for $t {
            fn sub_assign(&mut self, rhs: Duration) {
                self.0 = self.0 - rhs;
            }
        }

        impl ops::Sub<$t> for $t {
            type Output = Duration;

            fn sub(self, rhs: $t) -> Self::Output {
                self.0 - rhs.0
            }
        }

        impl Default for $t {
            fn default() -> Self {
                Self::now()
            }
        }

        impl AsRef<Duration> for $t {
            fn as_ref(&self) -> &Duration {
                &self.0
            }
        }

        impl AsRef<$t> for $t {
            fn as_ref(&self) -> &$t {
                self
            }
        }
    };
}

impl_common_operations!(Timestamp);
impl_common_operations!(Monotonic);
