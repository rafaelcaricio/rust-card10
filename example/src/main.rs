#![no_std]
#![no_main]

use panic_abort as _;
use l0dable::entry;

entry!(main);
fn main() -> ! {
    loop {}
}
