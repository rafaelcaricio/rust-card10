//! Lights

use card10_sys::*;

#[derive(Clone, Copy)]
pub enum LEDColor {
    RGB(u8, u8, u8),
    HSV(f32, f32, f32),
}

/// Update all RGB LEDs
///
/// `f` must supply a `LEDColor` for `0..=10`.
pub fn update_rgb_leds<F>(f: F)
where
    F: Fn(i32) -> LEDColor,
{
    for index in 0..=10 {
        let color = f(index);
        match color {
            LEDColor::RGB(r, g, b) => unsafe {
                epic_leds_prep(index, r, g, b);
            }
            LEDColor::HSV(h, s, v) => unsafe {
                epic_leds_prep_hsv(index, h, s, v);
            }
        }
        unsafe {
            epic_leds_update();
        }
    }
}
