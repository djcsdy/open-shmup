use crate::c64::{C64HiresTileData, C64MulticolourTileData};
use std::io;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileData([u8; Self::SIZE_BYTES]);

impl C64TileData {
    const SIZE_BYTES: usize = 8;

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut buffer = [0u8; Self::SIZE_BYTES];
        reader.read_exact(&mut buffer)?;
        Ok(Self(buffer))
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.0)
    }

    pub fn as_array(&self) -> &[u8; Self::SIZE_BYTES] {
        &self.0
    }

    pub fn as_multicolour(&self) -> C64MulticolourTileData {
        return C64MulticolourTileData::new(self);
    }

    pub fn as_hires(&self) -> C64HiresTileData {
        return C64HiresTileData::new(self);
    }
}
