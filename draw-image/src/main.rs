#![no_std]
#![no_main]

use card10_l0dable::*;
use embedded_graphics::prelude::*;

use embedded_graphics::image::Image16BPP;
use embedded_graphics::pixelcolor::Rgb565;

main!(main);
fn main() {
    let display = Display::open();
    let mut framebuffer = display.framebuffer();

    let image: Image16BPP<Rgb565> =
        Image16BPP::new(include_bytes!("applewatch-160x80.raw"), 160, 80);
    framebuffer.draw(&image);
    framebuffer.send();

    loop {
        let buttons = Buttons::read();
        if buttons.left_top() {
            break;
        }
    }
}
