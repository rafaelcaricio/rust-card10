#![no_std]
#![no_main]

use core::fmt::Write;
use panic_abort as _;
use l0dable::*;

main!(main);
fn main() {
    writeln!(UART, "Hello from Rust\r").unwrap();

    let display = Display::open();
    for x in 0..160 {
        display.print(160 - x, 10, "Hello Rust\0", 0xffff, 0);
        display.update();
    }
}
