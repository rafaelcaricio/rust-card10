#![no_std]
#![feature(global_asm)]

global_asm!(include_str!("crt.s"));

/// Type check the user-supplied entry function.
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

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
        fn main() -> !;
    }

    main();
}
