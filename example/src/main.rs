#![no_std]
#![no_main]

use core::fmt::Write;
use l0dable::*;

main!(main);
fn main() {
    writeln!(UART, "Hello from Rust\r").unwrap();
    
    let display = Display::open();
    let light = LightSensor::start();
    for t in 0..Display::W {
        display.clear(Color::yellow());
        display.print(160 - t, 10, b"Hello Rust\0", Color::white(), Color::black());

        let b = Buttons::read();
        if b.left_bottom() {
            display.print(0, 60, b"LB\0", Color::red(), Color::black());
            vibra::set(true);
        }
        if b.right_bottom() {
            display.print(80, 60, b"RB\0", Color::red(), Color::black());
            vibra::set(false);
        }
        if b.left_top() {
            display.print(0, 10, b"LT\0", Color::red(), Color::black());
        }
        if b.right_top() {
            display.print(80, 10, b"RT\0", Color::red(), Color::black());
        }
        if b.right_top() {
            display.print(80, 30, b"Reset\0", Color::red(), Color::black());
        }
        writeln!(UART, "Light: {:?}\r", light.get()).unwrap();
        
        display.update();
    }
}
