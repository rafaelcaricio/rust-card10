//! Real-time clock functionality

use card10_sys::*;
use core::ops::Sub;

/// Implemented for `Seconds` and `Milliseconds`
pub trait Time {
    /// Get current time
    fn time() -> Self;
    /// Set the time (TODO)
    fn set_time(&self);
}

#[derive(Clone, Copy, Debug)]
pub struct Seconds(pub u32);

impl From<MilliSeconds> for Seconds {
    fn from(ms: MilliSeconds) -> Seconds {
        Seconds((ms.0 / 1000) as u32)
    }
}

impl Time for Seconds {
    fn time() -> Self {
        let s = unsafe { epic_rtc_get_seconds() };
        Seconds(s)
    }
    /// TODO
    fn set_time(&self) {
    }
}

impl Sub for Seconds {
    type Output = Seconds;
    fn sub(self, rhs: Seconds) -> Self::Output {
        Seconds(self.0 - rhs.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MilliSeconds(pub u64);

impl From<Seconds> for MilliSeconds {
    fn from(s: Seconds) -> MilliSeconds {
        MilliSeconds(s.0 as u64 * 1000)
    }
}

impl Time for MilliSeconds {
    fn time() -> Self {
        let ms = unsafe { epic_rtc_get_milliseconds() };
        MilliSeconds(ms)
    }
    /// TODO
    fn set_time(&self) {
    }
}

impl Sub for MilliSeconds {
    type Output = MilliSeconds;
    fn sub(self, rhs: MilliSeconds) -> Self::Output {
        MilliSeconds(self.0 - rhs.0)
    }
}
