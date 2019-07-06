#![no_std]
#![no_main]

use panic_abort as _;
use card10::entry;


#[entry]
fn main() -> ! {
    card10::init();

    panic!("TODO");
}
