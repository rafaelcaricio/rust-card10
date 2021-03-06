use super::framebuffer::FrameBuffer;
use card10_sys::*;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Color(u16);

impl Color {
    pub fn red() -> Self {
        Self::rgb8(0xff, 0, 0)
    }

    pub fn blue() -> Self {
        Self::rgb8(0, 0, 0xff)
    }

    pub fn green() -> Self {
        Self::rgb8(0, 0xff, 0)
    }

    pub fn black() -> Self {
        Self::rgb8(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::rgb8(0xff, 0xff, 0xff)
    }

    pub fn yellow() -> Self {
        Self::rgb8(0xff, 0xff, 0)
    }

    pub fn rgb8(r8: u8, g8: u8, b8: u8) -> Self {
        let c =
            ((u16::from(r8) & 0xF8) << 8) | ((u16::from(g8) & 0xFA) << 3) | (u16::from(b8) & 0xF8);
        Color(c)
    }

    pub fn r(&self) -> u8 {
        (self.0 >> 11) as u8
    }

    pub fn r8(&self) -> u8 {
        self.r() << 3
    }

    pub fn g(&self) -> u8 {
        ((self.0 >> 5) & 0xFA) as u8
    }

    pub fn g8(&self) -> u8 {
        self.g() << 2
    }

    pub fn b(&self) -> u8 {
        (self.0 & 0xF8) as u8
    }

    pub fn b8(&self) -> u8 {
        self.b() << 3
    }
}

#[repr(u32)]
pub enum LineStyle {
    Full = disp_linestyle_LINESTYLE_FULL,
    Dotted = disp_linestyle_LINESTYLE_DOTTED,
}

#[repr(u32)]
pub enum FillStyle {
    Empty = disp_fillstyle_FILLSTYLE_EMPTY,
    Filled = disp_fillstyle_FILLSTYLE_FILLED,
}

pub struct Display;

#[repr(u8)]
pub enum Font {
    Font8 = disp_font_name_DISP_FONT8 as u8,
    Font12 = disp_font_name_DISP_FONT12 as u8,
    Font16 = disp_font_name_DISP_FONT16 as u8,
    Font20 = disp_font_name_DISP_FONT20 as u8,
    Font24 = disp_font_name_DISP_FONT24 as u8,
}

/// Immediate mode routines
impl Display {
    pub const W: u16 = 160;
    pub const H: u16 = 80;
    pub const FONT_W: u16 = 14;
    pub const FONT_H: u16 = 20;

    /// Open the display, return an instance
    pub fn open() -> Self {
        unsafe {
            epic_disp_open();
        }
        Display
    }

    /// Write Epicardium's framebuffer to the display
    pub fn update(&self) {
        unsafe {
            epic_disp_update();
        }
    }

    /// Clear everything with a solid `color`
    pub fn clear(&self, color: Color) {
        unsafe {
            epic_disp_clear(color.0);
        }
    }

    /// Print text
    ///
    /// s must be 0-terminated
    pub fn print(&self, x: u16, y: u16, s: &[u8], fg: Color, bg: Color) {
        unsafe {
            epic_disp_print(x, y, s.as_ptr(), fg.0, bg.0);
        }
    }

    /// Print text with a selected font
    ///
    /// s must be 0-terminated
    pub fn print_adv(&self, font: Font, x: u16, y: u16, s: &[u8], fg: Color, bg: Color) {
        unsafe {
            epic_disp_print_adv(font as u8, x, y, s.as_ptr(), fg.0, bg.0);
        }
    }

    /// Set a pixel
    pub fn pixel(&self, x: u16, y: u16, color: Color) {
        unsafe {
            epic_disp_pixel(x, y, color.0);
        }
    }

    /// Draw a line
    pub fn line(
        &self,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
        color: Color,
        linestyle: LineStyle,
        pixelsize: u16,
    ) {
        unsafe {
            epic_disp_line(x1, y1, x2, y2, color.0, linestyle as u32, pixelsize);
        }
    }

    pub fn rect(
        &self,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
        color: Color,
        fillstyle: FillStyle,
        pixelsize: u16,
    ) {
        unsafe {
            epic_disp_rect(x1, y1, x2, y2, color.0, fillstyle as u32, pixelsize);
        }
    }

    /// Draw a circle
    pub fn circ(
        &self,
        x: u16,
        y: u16,
        rad: u16,
        color: Color,
        fillstyle: FillStyle,
        pixelsize: u16,
    ) {
        unsafe {
            epic_disp_circ(x, y, rad, color.0, fillstyle as u32, pixelsize);
        }
    }

    /// Obtain a handle for a framebuffer.
    ///
    /// Don't use `display.send()` but `framebuffer.send()` as long as
    /// it is in use.
    pub fn framebuffer<'d>(&'d self) -> FrameBuffer<'d> {
        FrameBuffer::uninitialized(self)
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            epic_disp_close();
        }
    }
}

/// Convenience text display
///
/// Requires `card10_alloc::init()` and `extern crate alloc;`
#[macro_export]
macro_rules! display {
    ($disp: expr, $x: expr, $y: expr, $fg: expr, $bg: expr,
     $fmt: expr) => ({
         use alloc::format;
         let s = format!(concat!($fmt, "\0"));
         $disp.print($x, $y, s.as_bytes(), $fg, $bg);
    });
    ($disp: expr, $x: expr, $y: expr, $fg: expr, $bg: expr,
     $fmt: expr, $($arg: tt)*) => ({
         use alloc::format;
         let s = format!(concat!($fmt, "\0"), $($arg)*);
         $disp.print($x, $y, s.as_bytes(), $fg, $bg);
    });
}

/// Convenience text display with selected font
///
/// Requires `card10_alloc::init()` and `extern crate alloc;`
#[macro_export]
macro_rules! display_adv {
    ($disp: expr, $font: tt, $x: expr, $y: expr, $fg: expr, $bg: expr,
     $fmt: expr) => ({
         use alloc::format;
         use card10_l0dable::Font;
         let s = format!(concat!($fmt, "\0"));
         $disp.print_adv(Font::$font, $x, $y, s.as_bytes(), $fg, $bg);
    });
    ($disp: expr, $font: tt, $x: expr, $y: expr, $fg: expr, $bg: expr,
     $fmt: expr, $($arg: tt)*) => ({
         use alloc::format;
         let s = format!(concat!($fmt, "\0"), $($arg)*);
         $disp.print_adv(Font::$font, $x, $y, s.as_bytes(), $fg, $bg);
    });
}
