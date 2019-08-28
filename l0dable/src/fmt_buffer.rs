//! Stolen from https://stackoverflow.com/questions/39488327/how-to-format-output-to-a-byte-array-with-no-std-and-no-allocator
use core::fmt::{self, Write};
use core::mem::uninitialized;
use core::str::from_utf8_unchecked;

pub struct FmtBuffer<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> FmtBuffer<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        FmtBuffer {
            buf: buf,
            offset: 0,
        }
    }
}

impl<'a> fmt::Write for FmtBuffer<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();

        // Skip over already-copied data
        let remainder = &mut self.buf[self.offset..];
        // Check if there is space remaining (return error instead of panicking)
        if remainder.len() < bytes.len() { return Err(core::fmt::Error); }
        // Make the two slices the same length
        let remainder = &mut remainder[..bytes.len()];
        // Copy
        remainder.copy_from_slice(bytes);

        // Update offset to avoid overwriting
        self.offset += bytes.len();

        Ok(())
    }
}

/// 255 bytes ought to be enough for any C string
pub fn str_to_cstr(s: &str) -> [u8; 256] {
    let mut buf: [u8; 256] = unsafe { uninitialized() };
    let mut fmt = FmtBuffer::new(buf.as_mut());
    write!(fmt, "{}\0", s).unwrap();
    buf
}

impl AsRef<str> for FmtBuffer<'_> {
    fn as_ref(&self) -> &str {
        let len = self.buf.iter().position(|b| *b == 0)
            .unwrap_or(0);
        unsafe {
            from_utf8_unchecked(&self.buf[0..len])
        }
    }
}
