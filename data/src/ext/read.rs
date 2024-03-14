use std::io;
use std::io::Read;

pub trait ReadExt {
    fn read_u8_array<const S: usize>(&mut self) -> io::Result<[u8; S]>;
}

impl<R: Read> ReadExt for R {
    fn read_u8_array<const S: usize>(&mut self) -> io::Result<[u8; S]> {
        let mut buffer = [0u8; S];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}
