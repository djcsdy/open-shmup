use std::io;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileBlockSetData {
    pub block_colours: [u8; Self::BLOCK_COUNT],
    pub block_data: [u8; Self::BLOCK_COUNT * Self::BLOCK_SIZE_BYTES],
    pub shared_colours: [u8; Self::SHARED_COLOUR_COUNT],
    pub tiles: [u8; Self::TILE_COUNT * Self::TILE_SIZE_BYTES],
}

impl C64TileBlockSetData {
    const TILE_COUNT: usize = 254;
    const TILE_SIZE_BYTES: usize = 8;
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

        let mut tiles = [0u8; Self::TILE_COUNT * Self::TILE_SIZE_BYTES];
        reader.read_exact(&mut tiles)?;

        Ok(Self {
            block_colours,
            block_data,
            shared_colours,
            tiles,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.block_colours)?;
        writer.write_all(&self.block_data)?;
        writer.write_all(&self.shared_colours)?;
        writer.write_all(&self.tiles)?;
        Ok(())
    }
}
