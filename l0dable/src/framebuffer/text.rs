use core::fmt::Write;
use super::{FrameBuffer, Font};
use crate::{Color, Display};

pub struct TextRenderer<'a, 'd, 'f> {
    pub framebuffer: &'a mut FrameBuffer<'d>,
    pub x: isize,
    pub y: isize,
    pub font: &'f Font,
    pub color: Color,
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
                    let y1 = self.y + y as isize;
                    if y1 >= 0 && y1 < Display::H as isize {
                        for x in 0..self.font.w {
                            let x1 = self.x + x as isize;
                            if x1 >= 0 && x1 < Display::W as isize {
                                if glyph.get_pixel(x as usize, y as usize) {
                                    self.framebuffer[y1 as usize][x1 as usize] = self.color;
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
