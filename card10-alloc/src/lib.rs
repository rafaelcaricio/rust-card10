//! Support for dynamically allocated memory
//!
//! Reproduces l0dable hardware.c's `_sbrk()`
//!
//! Unfortunately, we cannot link _sbrk()` directly because it
//! references the unwieldy `errno`.
//!
//! ## Example
//!
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! extern crate alloc;
//! use alloc::vec;
//! use card10_l0dable::*;
//!
//! main!(main);
//! fn main() {
//!     // Pass stack headroom
//!     card10_alloc::init(128 * 1024);
//!     let mut xs = vec![];
//!     xs.push(23);
//! }
//! ```
#![no_std]
#![feature(asm)]
#![feature(alloc_error_handler)]

use core::alloc::Layout;
use alloc_cortex_m::CortexMHeap;
use card10_sys as _;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

extern "C" {
    static mut __heap_start: u32;
}

#[inline(always)]
fn sp() -> usize {
    let mut value;
    unsafe {
        asm!("mov $0, sp" : "=r" (value) ::: "volatile");
    }
    value
}

/// Call this before using anything from `alloc`.
///
/// Consider the size of your stack-allocated variables for the
/// `stack_headroom` parameter.
///
/// Returns heap size
pub fn init(stack_headroom: usize) -> usize {
    let start = unsafe { &__heap_start } as *const _ as usize;
    let size = sp() - stack_headroom - start;
    unsafe { ALLOCATOR.init(start, size); }
    size
}

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    panic!("OOM")
}
