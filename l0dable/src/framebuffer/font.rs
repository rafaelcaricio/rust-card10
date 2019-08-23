use core::slice::from_raw_parts;

extern "C" {
    static Font8: Font;
    static Font12: Font;
    static Font16: Font;
    static Font20: Font;
    static Font24: Font;
}

#[repr(C)]
pub struct Font {
    table: *const u8,
    pub w: u16,
    pub h: u16,
}

impl Font {
    pub fn font8() -> &'static Self {
        unsafe { &Font8 }
    }
    pub fn font12() -> &'static Self {
        unsafe { &Font12 }
    }
    pub fn font16() -> &'static Self {
        unsafe { &Font16 }
    }
    pub fn font20() -> &'static Self {
        unsafe { &Font20 }
    }
    pub fn font24() -> &'static Self {
        unsafe { &Font24 }
    }
    
    fn bytes_per_row(&self) -> usize {
        self.w as usize / 8 + 1
    }
    
    pub fn get_glyph(&self, c: char) -> Option<Glyph> {
        let h = self.h as usize;
        let bytes_per_row = self.bytes_per_row();
        let table = unsafe {
            from_raw_parts(self.table, ('~' as usize - (' ' as usize) - 1) * bytes_per_row * h)
        };
        let offset = (c as usize - (' ' as usize)) * bytes_per_row * h;
        if offset < table.len() {
            let table = &table[offset..(offset + bytes_per_row * h)];
            Some(Glyph {
                table,
                bytes_per_row,
            })
        } else {
            None
        }
    }
}

pub struct Glyph<'a> {
    table: &'a [u8],
    bytes_per_row: usize,
}

impl<'a> Glyph<'a> {
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        let offset = x / 8 + y * self.bytes_per_row;
        self.table[offset] & (1 << (7 - (x & 7))) != 0
    }
}
