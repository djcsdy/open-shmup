use std::io;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileBlockData {
    pub colour_data: u8,
    pub tile_indices: [u8; Self::SIZE_BYTES],
}

impl C64TileBlockData {
    const WIDTH_TILES: usize = 5;
    const HEIGHT_TILES: usize = 5;
    const SIZE_BYTES: usize = Self::WIDTH_TILES * Self::HEIGHT_TILES;

    pub fn read<R: Read>(reader: &mut R, colour_data: u8) -> io::Result<Self> {
        let mut tile_indices = [0u8; Self::SIZE_BYTES];
        reader.read_exact(&mut tile_indices)?;
        Ok(Self {
            colour_data,
            tile_indices,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.tile_indices)
    }
}
