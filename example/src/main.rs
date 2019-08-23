#![no_std]
#![no_main]

use core::fmt::Write;
use card10_l0dable::*;

main!(main);
fn main() {
    writeln!(UART, "Hello from Rust\r").unwrap();
    let bme = BME680::start();
    let a = BHI160::<Accelerometer>::start();
    let g = BHI160::<Gyroscope>::start();
    let o = BHI160::<Orientation>::start();
    
    let display = Display::open();
    let light = LightSensor::start();
    for t in 0..Display::W {
        writeln!(UART, "BME: {:?}\r", bme.read()).unwrap();
        writeln!(UART, "A:\r").unwrap();
        for d in &a.read() {
            writeln!(UART, " - {:?}\r", d).unwrap();
        }
        writeln!(UART, "O:\r").unwrap();
        for d in &o.read() {
            writeln!(UART, " - {:?}\r", d).unwrap();
        }
        writeln!(UART, "G:\r").unwrap();
        for d in &g.read() {
            writeln!(UART, " - {:?}\r", d).unwrap();
        }
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
