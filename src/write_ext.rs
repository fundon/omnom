use crate::WriteBytes;
use std::io::{self, Write};

/// Extension trait to `Write` to read and write bytes from.
pub trait WriteExt: Write + Sized {
    /// Write bytes as big endian.
    ///
    /// Returns the amount of bytes written.
    fn write_be_bytes<B: WriteBytes>(&mut self, num: B) -> io::Result<usize> {
        num.write_be_bytes(self)
    }

    /// Write bytes as little endian.
    ///
    /// Returns the amount of bytes written.
    fn write_le_bytes<B: WriteBytes>(&mut self, num: B) -> io::Result<usize> {
        num.write_le_bytes(self)
    }

    /// Write bytes using native endianness.
    ///
    /// Returns the amount of bytes written.
    fn write_ne_bytes<B: WriteBytes>(&mut self, num: B) -> io::Result<usize> {
        num.write_ne_bytes(self)
    }
}

impl<T: Write> WriteExt for T {}
