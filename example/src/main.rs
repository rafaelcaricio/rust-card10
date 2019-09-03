#![no_std]
#![no_main]

use core::fmt::{self, Write};
use card10_l0dable::*;
/// Allows you to `use alloc::*;`
extern crate alloc;

main!(main);
fn main() {
    let heap_size = card10_alloc::init(4096);
    println!("Heap size: {}", heap_size);

    let result = run();
    if let Err(error) = result {
        writeln!(UART, "error: {}\r", error).unwrap();
    }
}

fn run() -> Result<(), Error> {
    writeln!(UART, "Hello from Rust\r")?;

    let bme = BME680::start();
    let a = BHI160::<Accelerometer>::start()?;
    let g = BHI160::<Gyroscope>::start()?;
    let o = BHI160::<Orientation>::start()?;

    let display = Display::open();
    let light = LightSensor::start();

    for t in 0..Display::W {
        writeln!(UART, "BME: {:?}\r", bme.read())?;
        writeln!(UART, "A:\r")?;

        for d in &a.read()? {
            writeln!(UART, " - {:?}\r", d)?;
        }

        writeln!(UART, "O:\r")?;
        for d in &o.read()? {
            writeln!(UART, " - {:?}\r", d)?;
        }

        writeln!(UART, "G:\r")?;
        for d in &g.read()? {
            writeln!(UART, " - {:?}\r", d)?;
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
        writeln!(UART, "Light: {:?}\r", light.get())?;

        display.update();
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Error
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub enum Error {
    UartWriteFailed(fmt::Error),
    SensorInteractionFailed(BHI160Error),
}

impl From<fmt::Error> for Error {
    fn from(error: fmt::Error) -> Self {
        Error::UartWriteFailed(error)
    }
}

impl From<BHI160Error> for Error {
    fn from(error: BHI160Error) -> Self {
        Error::SensorInteractionFailed(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UartWriteFailed(error) => error.fmt(f),
            Error::SensorInteractionFailed(error) => error.fmt(f),
        }
    }
}
