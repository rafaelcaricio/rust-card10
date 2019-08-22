//! Stolen from https://stackoverflow.com/questions/39488327/how-to-format-output-to-a-byte-array-with-no-std-and-no-allocator
use core::fmt;

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
