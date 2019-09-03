use core::fmt::Write;

use card10_sys::*;

/// USB UART, 115200 baud
///
/// Supports use of `write!(Uart, "{}", ...);` Use `println!(...);`
/// for convenience.
pub struct Uart;

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            epic_uart_write_str(s.as_ptr(), s.len() as isize);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    crate::UART.write_fmt(args);
}
