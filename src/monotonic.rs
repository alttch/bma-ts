#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Monotonic(pub(crate) Duration);

use std::time::Duration;
#[cfg(not(target_family = "unix"))]
use std::time::Instant;

#[cfg(not(target_family = "unix"))]
static STARTED_AT: once_cell::sync::Lazy<Instant> = once_cell::sync::Lazy::new(|| Instant::now());

impl Monotonic {
    /// On non-UNIX platforms returns time since the first access
    ///
    /// # Panics
    ///
    /// On UNIX platforms will panic if the system monotonic clock is not available
    #[inline]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    #[cfg(target_family = "unix")]
    pub fn now() -> Self {
        let t = nix::time::clock_gettime(nix::time::ClockId::CLOCK_MONOTONIC).unwrap();
        Self(Duration::new(t.tv_sec() as u64, t.tv_nsec() as u32))
    }
    /// # Panics
    ///
    /// On UNIX platforms will panic if the system monotonic clock is not available
    #[inline]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    #[cfg(target_family = "unix")]
    pub fn now_rounded() -> Self {
        let t = nix::time::clock_gettime(nix::time::ClockId::CLOCK_MONOTONIC).unwrap();
        Self(Duration::new(t.tv_sec() as u64, 0))
    }
    #[cfg(not(target_family = "unix"))]
    #[inline]
    pub fn now_monotonic() -> Self {
        STARTED_AT.elapsed().into()
    }
    #[cfg(not(target_family = "unix"))]
    #[inline]
    pub fn now_monotonic_rounded() -> Self {
        Timestamp::from_secs(STARTED_AT.elapsed().as_secs())
    }
    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now().0 - self.0
    }
}
