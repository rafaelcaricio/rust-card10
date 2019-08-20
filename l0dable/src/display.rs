use super::bindings::*;

pub struct Display;

impl Display {
    pub fn open() -> Self {
        unsafe { epic_disp_open(); }
        Display
    }

    pub fn print(&self, x: u16, y: u16, s: &str, fg: u16, bg: u16) {
        unsafe {
            epic_disp_print(x, y, s.as_ptr(), fg, bg);
        }
    }

    pub fn update(&self) {
        unsafe {
            epic_disp_update();
        }
    }
}
