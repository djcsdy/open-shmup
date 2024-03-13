use crate::c64::tile::tile_block::C64TileBlockData;
use crate::c64::tile::C64TileSetData;
use crate::ext::array::array_from_fallible_fn;
use std::io;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileBlockSetData {
    pub blocks: [C64TileBlockData; Self::BLOCK_COUNT],
    pub shared_colours: [u8; Self::SHARED_COLOUR_COUNT],
    pub tile_set: C64TileSetData,
}

impl C64TileBlockSetData {
    const BLOCK_COUNT: usize = 128;
    const SHARED_COLOUR_COUNT: usize = 3;

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let blocks = array_from_fallible_fn(|_| C64TileBlockData::read(reader))?;

        let mut shared_colours = [0u8; Self::SHARED_COLOUR_COUNT];
        reader.read_exact(&mut shared_colours)?;

        let tile_set = C64TileSetData::read(reader)?;

        Ok(Self {
            blocks,
            shared_colours,
            tile_set,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        for block in &self.blocks {
            block.write(writer)?;
        }
        writer.write_all(&self.shared_colours)?;
        self.tile_set.write(writer)?;
        Ok(())
    }
}
