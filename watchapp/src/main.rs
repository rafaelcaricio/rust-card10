#![no_std]
#![no_main]

use panic_abort as _;
use card10::{entry, lcd};


#[entry]
fn main() -> ! {
    card10::init();

    lcd::set_backlight(1000);
    let mut t = 0;
    loop {
        for x in 0..lcd::W {
            for y in 0..lcd::H {
                if (((x - 2 * t) / 8) + ((y + t) / 8)) % 2 == 0 {
                    lcd::put_pixel(x, y, 0);
                } else {
                    lcd::put_pixel(x, y, 0xffff);
                }
            }
        }
        lcd::update();

        t += 1;
    }
}
