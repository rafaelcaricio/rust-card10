#![no_std]

mod os;
pub use os::*;
mod display;
pub use display::{Color, Display, Font, FillStyle, LineStyle};
mod buttons;
pub mod framebuffer;
pub use buttons::Buttons;
pub mod uart;
pub const UART: uart::Uart = uart::Uart;
mod light_sensor;
pub use light_sensor::LightSensor;
mod rtc;
pub mod trng;
pub mod vibra;
pub use rtc::{MilliSeconds, Seconds, Time};
mod fmt_buffer;
pub use fmt_buffer::{str_to_cstr, FmtBuffer};
mod bme680;
pub use bme680::BME680;
mod bhi160;
pub use bhi160::{
    Accelerometer, Error as BHI160Error, Gyroscope, Orientation, Sensor as BHI160,
    SensorData as BHI160Data,
};
pub mod fs;
mod leds;
pub use leds::*;

/// Type check the user-supplied entry function.
#[macro_export]
macro_rules! main {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() {
            // type check the given path
            let f: fn() = $path;

            f()
        }
    };
}
