//! True Random Number Generator

use card10_sys::*;

/// Read bytes from the True Random Number Generated
pub fn read(dest: &mut [u8]) -> bool {
    unsafe { epic_trng_read(dest.as_mut_ptr(), dest.len()) != 0 }
}
