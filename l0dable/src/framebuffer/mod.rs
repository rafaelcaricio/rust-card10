use core::mem::{transmute, uninitialized};
use core::ops::{Index, IndexMut};
use crate::bindings::*;
use crate::Display;

mod font;
pub use font::*;
mod text;
pub use text::TextRenderer;

pub struct FrameBuffer<'d> {
    _display: &'d Display,
    buffer: disp_framebuffer,
}


impl<'d> FrameBuffer<'d> {
    pub fn uninitialized(display: &'d Display) -> Self {
        FrameBuffer {
            _display: display,
            buffer: unsafe {
                uninitialized()
            },
        }
    }

    pub fn send(&mut self) {
        unsafe {
            epic_disp_framebuffer(&mut self.buffer);
        }
    }

    pub fn text<'a, 'f>(&'a mut self, x: isize, y: isize, font: &'f Font, color: RawColor) -> TextRenderer<'a, 'd, 'f> {
        TextRenderer {
            framebuffer: self,
            x, y, font, color,
        }
    }
}

impl<'d> Index<(u16, u16)> for FrameBuffer<'d> {
    type Output = RawColor;
    fn index(&self, (x, y): (u16, u16)) -> &Self::Output {
        let x = usize::from(Display::W - 1 - x);
        let y = usize::from(Display::H - 1 - y);
        unsafe {
            transmute(&self.buffer.fb[y][x])
        }
    }
}

impl<'d> IndexMut<(u16, u16)> for FrameBuffer<'d> {
    fn index_mut(&mut self, (x, y): (u16, u16)) -> &mut Self::Output {
        let x = usize::from(Display::W - 1 - x);
        let y = usize::from(Display::H - 1 - y);
        unsafe {
            transmute(&mut self.buffer.fb[y][x])
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RawColor([u8; 2]);

impl RawColor {
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
        let b1 = (r8 & 0xF8) | (g8 >> 5);
        let b2 = ((g8 & 0xFA) << 3) | (b8 >> 3);
        RawColor([b1, b2])
    }
}
