use core::mem::{transmute, uninitialized};
use core::ops::{Deref, DerefMut};
use crate::bindings::*;
use crate::{Color, Display};

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
}

impl<'d> Deref for FrameBuffer<'d> {
    type Target = [[Color; Display::W as usize]; Display::H as usize];
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.buffer.raw) }
    }
}

impl<'d> DerefMut for FrameBuffer<'d> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(&mut self.buffer.raw) }
    }
}
