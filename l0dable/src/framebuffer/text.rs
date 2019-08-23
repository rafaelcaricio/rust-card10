use core::fmt::Write;
use super::{FrameBuffer, Font, RawColor};
use crate::{Display};

pub struct TextRenderer<'a, 'd, 'f> {
    pub framebuffer: &'a mut FrameBuffer<'d>,
    pub x: isize,
    pub y: isize,
    pub font: &'f Font,
    pub color: RawColor,
}

impl<'a, 'd, 'f> Write for TextRenderer<'a, 'd, 'f> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        match self.font.get_glyph(c) {
            None => Ok(()),
            Some(glyph) => {
                for y in 0..self.font.h {
                    let y1 = (self.y + y as isize) as u16;
                    if y1 < Display::H {
                        for x in 0..self.font.w {
                            let x1 = (self.x + x as isize) as u16;
                            if x1 < Display::W {
                                if glyph.get_pixel(x as usize, y as usize) {
                                    self.framebuffer[(x1, y1)] = self.color;
                                }
                            }
                        }
                    }
                }
                self.x += self.font.w as isize;
                Ok(())
            }
        }
    }
}
