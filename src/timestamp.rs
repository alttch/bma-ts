use std::time::Duration;

#[cfg(not(target_family = "wasm"))]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(target_family = "wasm")]
use web_time::{SystemTime, UNIX_EPOCH};

use crate::Error;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timestamp(pub(crate) Duration);

const ANSI_EPOCH_DIFF_NANOS: u64 = 11_644_473_600_000_000_000;

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
    #[inline]
    pub fn duration_since(self, earlier: Self) -> Result<Duration, Error> {
        if self >= earlier {
            Ok(self.0 - earlier.0)
        } else {
            Err(Error::TimeWentBackward)
        }
    }
    /// Converts between ANSI (Windows, 1601-01-01) and UNIX (1970-01-01) timestamps
    /// The source timestamp MUST be in nanoseconds (for Windows timestamp - multiply the source by
    /// 100)
    ///
    /// # Example
    ///
    /// ```rust
    /// use bma_ts::Timestamp;
    ///
    /// let windows_timestamp = 133_575_013_473_650_000;
    /// let unix_timestamp = Timestamp::from_nanos(windows_timestamp * 100)
    ///     .try_from_ansi_to_unix().unwrap();
    /// assert_eq!(unix_timestamp.as_nanos(), 1_713_027_747_365_000_000);
    /// ```
    pub fn try_from_ansi_to_unix(self) -> Result<Self, Error> {
        Ok(Self(Duration::from_nanos(
            u64::try_from(self.0.as_nanos())?
                .checked_sub(ANSI_EPOCH_DIFF_NANOS)
                .ok_or_else(|| Error::Convert("Failed to convert from ANSI to UNIX".to_string()))?,
        )))
    }
    /// Converts between UNIX (1970-01-01) and ANSI (Windows, 1601-01-01) timestamps
    ///
    /// The result timestamp is in nanoseconds (for Windows timestamp - divide the target by 100)
    pub fn try_from_unix_to_ansi(self) -> Result<Self, Error> {
        Ok(Self(Duration::from_nanos(
            u64::try_from(self.0.as_nanos())?
                .checked_add(ANSI_EPOCH_DIFF_NANOS)
                .ok_or_else(|| Error::Convert("Failed to convert from UNIX to ANSI".to_string()))?,
        )))
    }
}
