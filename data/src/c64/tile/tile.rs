use crate::c64::{C64HiresTileData, C64MulticolourTileData};
use std::io;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileData(pub(super) [u8; Self::SIZE_BYTES]);

impl C64TileData {
    pub(super) const WIDTH: usize = 8;
    pub(super) const HEIGHT: usize = 8;
    const SIZE_BYTES: usize = Self::HEIGHT;

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut buffer = [0u8; Self::SIZE_BYTES];
        reader.read_exact(&mut buffer)?;
        Ok(Self(buffer))
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.0)
    }

    pub fn as_multicolour(&self) -> C64MulticolourTileData {
        return C64MulticolourTileData::new(self);
    }

    pub fn as_hires(&self) -> C64HiresTileData {
        return C64HiresTileData::new(self);
    }
}
