use std::ops;
use std::time::Duration;

use crate::Timestamp;

impl ops::Add<Duration> for Timestamp {
    type Output = Timestamp;

    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl ops::Sub<Duration> for Timestamp {
    type Output = Timestamp;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl AsRef<Duration> for Timestamp {
    fn as_ref(&self) -> &Duration {
        &self.0
    }
}

impl AsRef<Timestamp> for Timestamp {
    fn as_ref(&self) -> &Timestamp {
        self
    }
}
