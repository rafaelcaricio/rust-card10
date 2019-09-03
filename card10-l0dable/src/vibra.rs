//! Vibration motor

use card10_sys::*;

/// Set vibration motor to off/on
pub fn set(status: bool) {
    unsafe {
        epic_vibra_set(status.into());
    }
}

/// Vibrate for a duration
pub fn vibrate(millis: i32) {
    unsafe {
        epic_vibra_vibrate(millis);
    }
}
