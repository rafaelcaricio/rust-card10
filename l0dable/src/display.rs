use super::bindings::*;

pub struct Display;

impl Display {
    pub const W: u16 = 160;
    pub const H: u16 = 80;
    
    pub fn open() -> Self {
        unsafe { epic_disp_open(); }
        Display
    }

    pub fn update(&self) {
        unsafe { epic_disp_update(); }
    }

    /// s must be 0-terminated
    pub fn print(&self, x: u16, y: u16, s: &[u8], fg: u16, bg: u16) {
        unsafe {
            epic_disp_print(x, y, s.as_ptr(), fg, bg);
        }
    }

    pub fn pixel(&self, x: u16, y: u16, color: u16) {
        unsafe {
            epic_disp_pixel(x, y, color);
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { epic_disp_close(); }
    }
}
