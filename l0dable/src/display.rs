use super::bindings::*;

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
            ((u16::from(r8) & 0xF8) << 8) |
            ((u16::from(g8) & 0xFA) << 3) |
            (u16::from(b8) & 0xF8);
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
        self.r() << 2
    }

    pub fn b(&self) -> u8 {
        (self.0 & 0xF8) as u8
    }

    pub fn b8(&self) -> u8 {
        self.r() << 3
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

impl Display {
    pub const W: u16 = 160;
    pub const H: u16 = 80;
    pub const FONT_W: u16 = 14;
    pub const FONT_H: u16 = 20;

    pub fn open() -> Self {
        unsafe { epic_disp_open(); }
        Display
    }

    pub fn update(&self) {
        unsafe { epic_disp_update(); }
    }

    pub fn clear(&self, color: Color) {
        unsafe { epic_disp_clear(color.0); }
    }

    /// s must be 0-terminated
    pub fn print(&self, x: u16, y: u16, s: &[u8], fg: Color, bg: Color) {
        unsafe {
            epic_disp_print(x, y, s.as_ptr(), fg.0, bg.0);
        }
    }

    pub fn pixel(&self, x: u16, y: u16, color: Color) {
        unsafe {
            epic_disp_pixel(x, y, color.0);
        }
    }

    pub fn line(&self, x1: u16, y1: u16, x2: u16, y2: u16, color: Color, linestyle: LineStyle, pixelsize: u16) {
        unsafe {
            epic_disp_line(x1, y1, x2, y2, color.0, linestyle as u32, pixelsize);
        }
    }

    pub fn rect(&self, x1: u16, y1: u16, x2: u16, y2: u16, color: Color, fillstyle: FillStyle, pixelsize: u16) {
        unsafe {
            epic_disp_rect(x1, y1, x2, y2, color.0, fillstyle as u32, pixelsize);
        }
    }

    pub fn circ(&self, x: u16, y: u16, rad: u16, color: Color, fillstyle: FillStyle, pixelsize: u16) {
        unsafe {
            epic_disp_circ(x, y, rad, color.0, fillstyle as u32, pixelsize);
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { epic_disp_close(); }
    }
}
