use crate::c64::tile::C64TileSetData;
use std::io;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileBlockSetData {
    pub block_colours: [u8; Self::BLOCK_COUNT],
    pub block_data: [u8; Self::BLOCK_COUNT * Self::BLOCK_SIZE_BYTES],
    pub shared_colours: [u8; Self::SHARED_COLOUR_COUNT],
    pub tile_set: C64TileSetData,
}

impl C64TileBlockSetData {
    const BLOCK_COUNT: usize = 128;
    const BLOCK_WIDTH_TILES: usize = 5;
    const BLOCK_HEIGHT_TILES: usize = 5;
    const BLOCK_SIZE_BYTES: usize = Self::BLOCK_WIDTH_TILES * Self::BLOCK_HEIGHT_TILES;
    const SHARED_COLOUR_COUNT: usize = 3;

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut block_colours = [0u8; Self::BLOCK_COUNT];
        reader.read_exact(&mut block_colours)?;

        let mut block_data = [0u8; Self::BLOCK_COUNT * Self::BLOCK_SIZE_BYTES];
        reader.read_exact(&mut block_data)?;

        let mut shared_colours = [0u8; Self::SHARED_COLOUR_COUNT];
        reader.read_exact(&mut shared_colours)?;

        let tile_set = C64TileSetData::read(reader)?;

        Ok(Self {
            block_colours,
            block_data,
            shared_colours,
            tile_set,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.block_colours)?;
        writer.write_all(&self.block_data)?;
        writer.write_all(&self.shared_colours)?;
        self.tile_set.write(writer)?;
        Ok(())
    }
}
