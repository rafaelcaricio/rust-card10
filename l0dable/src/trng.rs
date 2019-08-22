use super::bindings::*;

pub fn read(dest: &mut [u8]) -> bool {
    unsafe { epic_trng_read(dest.as_mut_ptr(), dest.len()) != 0 }
}
