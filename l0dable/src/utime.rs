use super::bindings::*;

pub fn time() -> u32 {
    unsafe { epic_rtc_get_seconds() }
}
