use crate::c64::{C64HiresTileData, C64MulticolourTileData};
use crate::ext::ReadExt;
use std::io;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileData(pub(super) [u8; Self::SIZE_BYTES]);

impl C64TileData {
    pub(super) const WIDTH: usize = 8;
    pub(super) const HEIGHT: usize = 8;
    const SIZE_BYTES: usize = Self::HEIGHT;

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self(reader.read_u8_array()?))
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
