use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::Error;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timestamp(pub(crate) Duration);

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
    pub fn elapsed(self) -> Result<Duration, Error> {
        let now = Timestamp::now();
        if now >= self {
            Ok(now.0 - self.0)
        } else {
            Err(Error::TimeWentBackward)
        }
    }
}
