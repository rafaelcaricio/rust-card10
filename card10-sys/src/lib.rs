#![no_std]
#![feature(global_asm)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use panic_abort as _;

global_asm!(include_str!(concat!(env!("OUT_DIR"), "/crt.s")));
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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

    epic_exit(0);
    unreachable!()
}

pub mod ctypes {
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

pub mod errno {
    use crate::ctypes::c_int;

    pub const EPERM: c_int = 1; // Operation not permitted
    pub const ENOENT: c_int = 2; // No such file or directory
    pub const ESRCH: c_int = 3; // No such process
    pub const EINTR: c_int = 4; // Interrupted system call
    pub const EIO: c_int = 5; // I/O error
    pub const ENXIO: c_int = 6; // No such device or address
    pub const E2BIG: c_int = 7; // Argument list too long
    pub const ENOEXEC: c_int = 8; // Exec format error
    pub const EBADF: c_int = 9; // Bad file number
    pub const ECHILD: c_int = 10; // No child processes
    pub const EAGAIN: c_int = 11; // Try again
    pub const ENOMEM: c_int = 12; //Out of memory
    pub const EACCES: c_int = 13; //Permission denied
    pub const EFAULT: c_int = 14; //Bad address
    pub const ENOTBLK: c_int = 15; //Block device required
    pub const EBUSY: c_int = 16; //Device or resource busy
    pub const EEXIST: c_int = 17; //File exists
    pub const EXDEV: c_int = 18; //Cross-device link
    pub const ENODEV: c_int = 19; //No such device
    pub const ENOTDIR: c_int = 20; //Not a directory
    pub const EISDIR: c_int = 21; //Is a directory
    pub const EINVAL: c_int = 22; //Invalid argument
    pub const ENFILE: c_int = 23; //File table overflow
    pub const EMFILE: c_int = 24; //Too many open files
    pub const ENOTTY: c_int = 25; //Not a typewriter
    pub const ETXTBSY: c_int = 26; //Text file busy
    pub const EFBIG: c_int = 27; //File too large
    pub const ENOSPC: c_int = 28; //No space left on device
    pub const ESPIPE: c_int = 29; //Illegal seek
    pub const EROFS: c_int = 30; //Read-only file system
    pub const EMLINK: c_int = 31; //Too many links
    pub const EPIPE: c_int = 32; //Broken pipe
    pub const EDOM: c_int = 33; //Math argument out of domain of func
    pub const ERANGE: c_int = 34; //Math result not representable
    pub const EAFNOSUPPORT: c_int = 97; //Address family not supported by protocol
    pub const ECONNRESET: c_int = 104; //Connection timed out
    pub const ETIMEDOUT: c_int = 110; //Connection timed out
    pub const EINPROGRESS: c_int = 115; //Operation now in progress
}
