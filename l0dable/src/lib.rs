#![no_std]
#![feature(global_asm)]

use panic_abort as _;

global_asm!(include_str!("crt.s"));

/// Type check the user-supplied entry function.
#[macro_export]
macro_rules! main {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() {
            // type check the given path
            let f: fn() = $path;

            f()
        }
    };
}

#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn Reset_Handler() -> ! {
    extern "C" {
        fn SystemInit();
        // Boundaries of the .bss section, provided by the linker script
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }

    // Zeroes the .bss section
    r0::zero_bss(&mut __bss_start, &mut __bss_end);
    SystemInit();

    extern "Rust" {
        fn main();
    }

    main();
    exit(0);
}

pub mod ctypes {
    #![allow(non_camel_case_types)]

    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_char = u8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub use core::ffi::c_void;
}

pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
use bindings::*;

mod display;
pub use display::{Display, Color, LineStyle, FillStyle};
mod buttons;
pub use buttons::Buttons;
pub mod uart;
pub const UART: uart::Uart = uart::Uart;
mod light_sensor;
pub use light_sensor::LightSensor;
pub mod vibra;
pub mod trng;
mod utime;
pub use utime::time;
mod fmt_buffer;
pub use fmt_buffer::FmtBuffer;

pub fn exit(ret: i32) -> ! {
    unsafe {
        epic_exit(ret);
    }
    unreachable!()
}
