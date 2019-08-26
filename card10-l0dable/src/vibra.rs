use card10_sys::*;

pub fn set(status: bool) {
    unsafe {
        epic_vibra_set(status.into());
    }
}

pub fn vibrate(millis: i32) {
    unsafe {
        epic_vibra_vibrate(millis);
    }
}
